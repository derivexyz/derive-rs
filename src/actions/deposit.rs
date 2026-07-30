use std::sync::Arc;

use alloy::{
    primitives::{Address, TxHash, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};
use bigdecimal::BigDecimal;
use bon::Builder;
use dashmap::DashMap;
use reqwest::Url;
use serde::Deserialize;
use strum::EnumString;

use crate::{
    actions::utils::decimal_to_u256_with_prec,
    models::openapi::SpotAssetEntry,
    types::{ClientError, Environment},
};
const ONCHAIN_ACTION_MANAGER: &str = "0x1b4f369b585D40a27F66775844FC265151f278A4";

#[derive(Clone, Debug, Deserialize, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum SupportDepositAssets {
    USDC,
}

#[derive(Clone, Debug, Deserialize)]
pub enum DirectDepositType {
    Deposit,
    DepositToNewSubaccount,
}

#[derive(Clone, Debug, Deserialize)]
pub enum DepositTypes {
    Direct(DirectDepositType),
}

sol! {
     #[sol(rpc)]
    contract OnchainActionManager {
        function deposit(
            address asset,
            uint256 amount,
            uint64 subaccountId,
            address fallbackRecipient
        ) returns (uint256 actionId);

        function depositToNewSubaccount(
            address asset,
            uint256 amount,
            uint32 managerId,
            address owner
        ) returns (uint256 actionId);

        function submit(
            uint256 actionType,
            bytes data
        ) payable returns (uint256 actionId);
    }
}

sol! {
    #[sol(rpc)]
    contract Erc20 {
        function approve(address spender, uint256 amount) returns (bool);
        function allowance(address owner, address spender) view returns (uint256);

        function balanceOf(address account) view returns (uint256);
    }
}

#[derive(Clone, Debug, Deserialize, Builder)]
pub struct DepositArgs {
    #[builder(into)]
    asset: SupportDepositAssets,
    amount: BigDecimal,
    #[builder(into)]
    recepient_address: Address,
    subaccount_id: Option<u64>,
    rpc_provider: Option<String>,
    // we use a default to specific subaccount
    #[builder(default=DepositTypes::Direct(DirectDepositType::Deposit))]
    deposit_type: DepositTypes,
}

struct AssetInfo {
    token_address: Address,
    underlying_erc20: Address,
    decimals: u8,
}

pub struct DepositManager {
    deposit_args: DepositArgs,
    private_key: PrivateKeySigner,
    wallet_address: Address,
    erc20_cache: Arc<DashMap<String, SpotAssetEntry>>,
    rpc_url: Url,
    manager_address: Address,
}

impl DepositManager {
    pub fn new(
        deposit_args: &DepositArgs,
        private_key: &PrivateKeySigner,
        wallet_address: &str,
        env: &Environment,
        erc20_cache: Arc<DashMap<String, SpotAssetEntry>>,
    ) -> Result<Self, ClientError> {
        let rpc_url = deposit_args
            .rpc_provider
            .clone()
            .unwrap_or_else(|| env.get_default_rpc())
            .parse::<Url>()
            .map_err(Self::string_error)?;

        let wallet_address = wallet_address
            .parse::<Address>()
            .map_err(Self::string_error)?;

        let manager_address = ONCHAIN_ACTION_MANAGER
            .parse::<Address>()
            .map_err(Self::string_error)?;

        Ok(Self {
            deposit_args: deposit_args.clone(),
            private_key: private_key.clone(),
            wallet_address,
            erc20_cache,
            rpc_url,
            manager_address,
        })
    }

    pub async fn deposit(&self) -> Result<Vec<TxHash>, ClientError> {
        let asset = &self.deposit_args.asset;
        let amount = &self.deposit_args.amount;

        if !self.check_balance(asset, amount).await? {
            return Err(ClientError::InsufficientBalance(format!(
                "Insufficient balance for {:?}: required {}",
                asset, amount
            )));
        }

        let mut tx_hashes = Vec::with_capacity(2);

        if !self.check_allowance(asset, amount).await? {
            let approval_hash = self.approve(asset, amount).await?;
            tx_hashes.push(approval_hash);
        }

        match &self.deposit_args.deposit_type {
            DepositTypes::Direct(deposit_type) => match deposit_type {
                DirectDepositType::Deposit => {
                    if let Some(subaccount_id) = self.deposit_args.subaccount_id {
                        return Err(ClientError::SubaccountError(format!(
                            "Subaccount ID {} is not supported for direct deposit",
                            subaccount_id
                        )));
                    }

                    let tx_hash = self.send_deposit().await?;
                    tx_hashes.push(tx_hash);
                }

                DirectDepositType::DepositToNewSubaccount => {
                    if self.deposit_args.subaccount_id.is_none() {
                        return Err(ClientError::SubaccountError(
                            "Subaccount ID is required for DepositToNewSubaccount".into(),
                        ));
                    }

                    let tx_hash = self.send_deposit().await?;
                    tx_hashes.push(tx_hash);
                }
            },
        }

        Ok(tx_hashes)
    }

    pub async fn check_allowance(
        &self,
        asset: &SupportDepositAssets,
        amount: &BigDecimal,
    ) -> Result<bool, ClientError> {
        let asset_info = self.asset_info(asset)?;
        let required = self.amount_to_units(amount, asset_info.decimals)?;

        let provider = ProviderBuilder::new().connect_http(self.rpc_url.clone());
        let erc20 = Erc20::new(asset_info.underlying_erc20, provider);

        let allowance = erc20
            .allowance(self.wallet_address, self.manager_address)
            .call()
            .await
            .map_err(Self::string_error)?;

        Ok(allowance >= required)
    }

    pub async fn check_balance(
        &self,
        asset: &SupportDepositAssets,
        amount: &BigDecimal,
    ) -> Result<bool, ClientError> {
        let asset_info = self.asset_info(asset)?;
        let required = self.amount_to_units(amount, asset_info.decimals)?;

        let provider = ProviderBuilder::new().connect_http(self.rpc_url.clone());
        let erc20 = Erc20::new(asset_info.underlying_erc20, provider);

        let balance = erc20
            .balanceOf(self.wallet_address)
            .call()
            .await
            .map_err(Self::string_error)?;

        Ok(balance >= required)
    }

    pub async fn approve(
        &self,
        asset: &SupportDepositAssets,
        amount: &BigDecimal,
    ) -> Result<TxHash, ClientError> {
        let asset_info = self.asset_info(asset)?;
        let amount = self.amount_to_units(amount, asset_info.decimals)?;

        let provider = ProviderBuilder::new()
            .wallet(self.private_key.clone())
            .connect_http(self.rpc_url.clone());

        let erc20 = Erc20::new(asset_info.underlying_erc20, provider);

        let pending_tx = erc20
            .approve(self.manager_address, amount)
            .send()
            .await
            .map_err(Self::string_error)?;

        let tx_hash = *pending_tx.tx_hash();

        // The deposit depends on the approval being mined.
        pending_tx.get_receipt().await.map_err(Self::string_error)?;

        Ok(tx_hash)
    }

    async fn send_deposit(&self) -> Result<TxHash, ClientError> {
        let asset_info = self.asset_info(&self.deposit_args.asset)?;
        let amount = self.amount_to_units(&self.deposit_args.amount, asset_info.decimals)?;

        let subaccount_id = self.deposit_args.subaccount_id.ok_or_else(|| {
            ClientError::SubaccountError("Subaccount ID is required for this deposit type".into())
        })?;

        let provider = ProviderBuilder::new()
            .wallet(self.private_key.clone())
            .connect_http(self.rpc_url.clone());

        let manager = OnchainActionManager::new(self.manager_address, provider);

        let pending_tx = manager
            .deposit(
                asset_info.token_address,
                amount,
                subaccount_id,
                self.deposit_args.recepient_address,
            )
            .send()
            .await
            .map_err(Self::string_error)?;

        Ok(*pending_tx.tx_hash())
    }

    fn asset_info(&self, asset: &SupportDepositAssets) -> Result<AssetInfo, ClientError> {
        let key = format!("{asset:?}");

        let entry = self.erc20_cache.get(&key).ok_or_else(|| {
            Self::message_error(format!(
                "Asset {:?} is not present in the ERC20 cache",
                asset
            ))
        })?;

        let token_address = entry
            .address
            .parse::<Address>()
            .map_err(Self::string_error)?;

        let underlying_erc20 = entry
            .erc20
            .underlying_erc20
            .as_deref()
            .ok_or_else(|| {
                Self::message_error(format!("Asset {:?} has no underlying ERC20 address", asset))
            })?
            .parse::<Address>()
            .map_err(Self::string_error)?;

        Ok(AssetInfo {
            token_address,
            underlying_erc20,
            decimals: entry.erc20.decimals,
        })
    }

    fn amount_to_units(&self, amount: &BigDecimal, decimals: u8) -> Result<U256, ClientError> {
        decimal_to_u256_with_prec(amount, decimals as u32).map_err(|_| {
            Self::message_error(format!(
                "Failed to convert amount {} to U256 with {} decimals",
                amount, decimals
            ))
        })
    }

    fn string_error<E>(error: E) -> ClientError
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        ClientError::StringError(Box::new(error))
    }

    fn message_error(message: String) -> ClientError {
        ClientError::StringError(Box::new(std::io::Error::other(message)))
    }
}
