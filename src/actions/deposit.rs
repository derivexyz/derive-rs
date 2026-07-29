use std::sync::Arc;

use alloy::providers::ProviderBuilder;
use alloy::{
    primitives::{Address, TxHash},
    signers::local::PrivateKeySigner,
};
use alloy_sol_types::sol;
use bigdecimal::BigDecimal;
use bon::Builder;
use dashmap::DashMap;
use serde::Deserialize;
use strum::EnumString;

use crate::actions::utils::decimal_to_u256_with_prec;
use crate::{
    models::openapi::SpotAssetEntry,
    types::{ClientError, Environment},
};

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

use std::str::FromStr;
pub struct DepositManager<'a> {
    deposit_args: DepositArgs,
    private_key: &'a PrivateKeySigner,
    wallet_address: &'a str,
    _env: &'a Environment,
    erc20_cache: &'a Arc<DashMap<String, SpotAssetEntry>>,
    rpc_url: String,
}

impl<'a> DepositManager<'a> {
    pub async fn new(
        deposit_args: &DepositArgs,
        private_key: &'a PrivateKeySigner,
        wallet_address: &'a str,
        env: &'a Environment,
        erc20_cache: &'a Arc<DashMap<String, SpotAssetEntry>>,
    ) -> Result<Self, ClientError> {
        // Implement the logic to create a new DepositManager instance

        let rpc_url = match deposit_args.rpc_provider.clone() {
            Some(url) => url,
            None => env.get_default_rpc(),
        };

        Ok(Self {
            deposit_args: deposit_args.clone(),
            private_key,
            wallet_address,
            _env: env,
            erc20_cache,
            rpc_url,
        })
    }

    pub async fn deposit(&self) -> Result<Vec<TxHash>, ClientError> {
        // Implement the deposit logic here

        let mut resulting_hashes = vec![];

        if !self
            .check_approvals(
                self.deposit_args.asset.clone(),
                self.deposit_args.amount.clone(),
            )
            .await?
        {
            // attempt to approve the necessary amount of the asset for deposit
            let approval_tx_hash = self
                .approve_asset(
                    self.deposit_args.asset.clone(),
                    self.deposit_args.amount.clone(),
                )
                .await?;
            resulting_hashes.push(approval_tx_hash);
        }

        // perform the contract call to deposit or depositToNewSubaccount based on the deposit type.
        match self.deposit_args.deposit_type {
            DepositTypes::Direct(ref direct_type) => match direct_type {
                DirectDepositType::Deposit => {
                    // Call the deposit function on the OnchainActionManager contract
                    // and push the resulting TxHash to resulting_hashes
                    if let Some(sub_id) = self.deposit_args.subaccount_id {
                        return Err(ClientError::SubaccountError(format!(
                            "Subaccount ID {} is not supported for direct deposit. Please use DepositToNewSubaccount type.",
                            sub_id
                        )));
                    }
                    todo!(
                        "Implement the deposit function call and push the resulting TxHash to resulting_hashes"
                    );
                }
                DirectDepositType::DepositToNewSubaccount => {
                    // Call the depositToNewSubaccount function on the OnchainActionManager contract
                    // and push the resulting TxHash to resulting_hashes
                    if self.deposit_args.subaccount_id.is_none() {
                        return Err(ClientError::SubaccountError(
                            "Subaccount ID is required for DepositToNewSubaccount type."
                                .to_string(),
                        ));
                    }
                    let tx_hash = self.handle_direct_deposit().await?;
                    resulting_hashes.push(tx_hash);
                }
            },
        }

        Ok(resulting_hashes)
    }

    pub async fn check_approvals(
        &self,
        asset: SupportDepositAssets,
        amount: BigDecimal,
    ) -> Result<bool, ClientError> {
        // we get the erc20 address from the erc20_cache
        let asset_entry = self.erc20_cache.get(&format!("{:?}", asset)).expect("ERC20 asset details not found in cache. Please ensure the asset is supported and cached.");
        let erc20_address = asset_entry
            .erc20
            .underlying_erc20
            .clone()
            .expect("Underlying ERC20 address not found")
            .parse::<Address>()
            .map_err(|e| ClientError::StringError(Box::new(e)))?;

        let url = self
            .rpc_url
            .parse::<reqwest::Url>()
            .map_err(|e| ClientError::StringError(Box::new(e)))?;
        let provider = ProviderBuilder::new().connect_http(url);

        println!("Erc20 Address: {:?}", erc20_address);
        let erc20_contract = Erc20::new(erc20_address, provider);
        let manager_address = "0x1b4f369b585D40a27F66775844FC265151f278A4"
            .parse()
            .expect("Invalid manager address");
        let allowance = erc20_contract
            .allowance(
                self.wallet_address.parse().expect("Invalid wallet address"),
                manager_address,
            )
            .call()
            .await
            .map_err(|e| ClientError::StringError(Box::new(e)))?;
        let decimals = self.erc20_cache.get(&format!("{:?}", asset)).expect("ERC20 asset details not found in cache. Please ensure the asset is supported and cached.").erc20.decimals;
        let amount_in_wei = amount * BigDecimal::from(10u64.pow(decimals as u32));
        let allowance_decimal = BigDecimal::from_str(&allowance.to_string())
            .map_err(|e| ClientError::StringError(Box::new(e)))?;
        println!(
            "Allowance: {}, Amount in Wei: {}",
            allowance_decimal, amount_in_wei
        );
        Ok(allowance_decimal >= amount_in_wei)
    }

    pub fn approve(
        &self,
        _asset: SupportDepositAssets,
        _amount: BigDecimal,
    ) -> Result<TxHash, ClientError> {
        // Implement the logic to approve the necessary amount of the asset for deposit
        Ok(TxHash::default())
    }

    async fn handle_direct_deposit(&self) -> Result<TxHash, ClientError> {
        println!(
            "Handling direct deposit for asset: {:?}",
            self.deposit_args.asset
        );
        let url = self
            .rpc_url
            .parse::<reqwest::Url>()
            .map_err(|e| ClientError::StringError(Box::new(e)))?;
        let provider = ProviderBuilder::new()
            .wallet(self.private_key.clone())
            .connect_http(url);

        let manager_address = "0x1b4f369b585D40a27F66775844FC265151f278A4"
            .parse()
            .expect("Invalid manager address");
        println!("Manager Address: {:?}", manager_address);
        let onchain_action_manager = OnchainActionManager::new(manager_address, provider);

        let asset_address = self.erc20_cache.get(&format!("{:?}", self.deposit_args.asset)).expect("ERC20 asset details not found in cache. Please ensure the asset is supported and cached.").address.clone().parse::<Address>().map_err(|e| ClientError::StringError(Box::new(e)))?;
        println!("Asset Address: {:?}", asset_address);
        let decimals = self.erc20_cache.get(&format!("{:?}", self.deposit_args.asset)).expect("ERC20 asset details not found in cache. Please ensure the asset is supported and cached.").erc20.decimals;
        let amount_u256 = decimal_to_u256_with_prec(&self.deposit_args.amount, decimals as u32)
            .expect("Failed to convert amount to U256");

        println!(
            "Depositing asset: {:?}, amount: {:?}, subaccount_id: {:?}, recepient_address: {:?}",
            self.deposit_args.asset,
            amount_u256,
            self.deposit_args.subaccount_id,
            self.deposit_args.recepient_address
        );
        let txn = onchain_action_manager
            .deposit(
                asset_address,
                amount_u256,
                self.deposit_args
                    .subaccount_id
                    .expect("Subaccount ID is required for DepositToNewSubaccount type"),
                self.deposit_args.recepient_address,
            )
            .send()
            .await
            .map_err(|e| ClientError::StringError(Box::new(e)))?;
        Ok(*txn.tx_hash())
    }

    async fn approve_asset(
        &self,
        asset: SupportDepositAssets,
        amount: BigDecimal,
    ) -> Result<TxHash, ClientError> {
        let url = self
            .rpc_url
            .parse::<reqwest::Url>()
            .map_err(|e| ClientError::StringError(Box::new(e)))?;
        let provider = ProviderBuilder::new()
            .wallet(self.private_key.clone())
            .connect_http(url);

        let asset_entry = self.erc20_cache.get(&format!("{:?}", asset)).expect("ERC20 asset details not found in cache. Please ensure the asset is supported and cached.");
        let erc20_address = asset_entry
            .erc20
            .underlying_erc20
            .clone()
            .expect("Underlying ERC20 address not found")
            .parse::<Address>()
            .map_err(|e| ClientError::StringError(Box::new(e)))?;

        let erc20_contract = Erc20::new(erc20_address, provider);
        let manager_address = "0x1b4f369b585D40a27F66775844FC265151f278A4"
            .parse()
            .expect("Invalid manager address");
        let decimals = asset_entry.erc20.decimals;
        let amount_u256 = decimal_to_u256_with_prec(&amount, decimals as u32)
            .expect("Failed to convert amount to U256");

        let txn = erc20_contract
            .approve(manager_address, amount_u256)
            .send()
            .await
            .map_err(|e| ClientError::StringError(Box::new(e)))?;
        let txn_hash = *txn.tx_hash();
        println!("Approval transaction sent: {:?}", txn_hash);
        let receipt = txn.get_receipt().await;
        println!("Approval transaction receipt: {:?}", receipt);
        Ok(txn_hash)
    }
}
