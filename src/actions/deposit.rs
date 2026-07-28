// export const ONCHAIN_ACTION_MANAGER_ABI = [
//   'function deposit(address asset, uint256 amount, uint64 subaccountId, address fallbackRecipient) returns (uint256 actionId)',
//   'function depositToNewSubaccount(address asset, uint256 amount, uint32 managerId, address owner) returns (uint256 actionId)',
//   'function submit(uint256 actionType, bytes data) payable returns (uint256 actionId)',
// ];

use std::sync::Arc;

use alloy::providers::{Provider, ProviderBuilder};
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
}

use std::str::FromStr;
pub struct DepositManager<'a> {
    deposit_args: DepositArgs,
    private_key: &'a PrivateKeySigner,
    wallet_address: &'a str,
    env: &'a Environment,
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
            env,
            erc20_cache,
            rpc_url,
        })
    }

    pub async fn deposit(&self) -> Result<Vec<TxHash>, ClientError> {
        // Implement the deposit logic here

        let resulting_hashes = vec![];

        if !self
            .check_approvals(
                self.deposit_args.asset.clone(),
                self.deposit_args.amount.clone(),
            )
            .await?
        {
            return Err(ClientError::ApprovalError(
                "Insufficient approval for deposit".to_string(),
            ));
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

        let erc20_address = asset_entry.address.clone();

        let url = self
            .rpc_url
            .parse::<reqwest::Url>()
            .map_err(|e| ClientError::StringError(Box::new(e)))?;
        let provider = ProviderBuilder::new().connect_http(url);

        let erc20_contract = Erc20::new(
            erc20_address.parse().expect("Invalid ERC20 address"),
            provider,
        );
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
}
