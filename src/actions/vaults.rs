use alloy::signers::SignerSync;
use alloy::{hex::encode_prefixed, primitives::U256, signers::local::PrivateKeySigner, sol};
use alloy_sol_types::SolValue;
use anyhow::Result;
use bigdecimal::BigDecimal;
use bon::Builder;
use serde::Deserialize;

use crate::constants::ZERO_ADDRESS;
use crate::models::{
    BurnSharesRequest, CancelVaultRequestRequest, MintSharesRequest, RequestVaultWithdrawRequest,
    VaultRequestId,
};
use crate::{
    Environment,
    actions::{ActionData, ModuleData, utils::to_e18},
    models::{CreateVaultRequest, RequestVaultDepositRequest},
};

pub enum VaultActionKind {
    Create = 0,
    Deposit = 1,
    Withdraw = 2,
    Cancel = 3,
    MintShares = 4,
    BurnShares = 5,
}

impl From<VaultActionKind> for U256 {
    fn from(kind: VaultActionKind) -> Self {
        match kind {
            VaultActionKind::Create => U256::from(0),
            VaultActionKind::Deposit => U256::from(1),
            VaultActionKind::Withdraw => U256::from(2),
            VaultActionKind::Cancel => U256::from(3),
            VaultActionKind::MintShares => U256::from(4),
            VaultActionKind::BurnShares => U256::from(5),
        }
    }
}
#[derive(Debug, Clone, Deserialize, Builder)]
pub struct CreateVaultArgs {
    subaccount_id: u64,
    manager_id: u64,
    deposit_spot_asset: String,
    initial_deposit: BigDecimal,
    initial_share_price_usd: BigDecimal,
    management_fee_bps: u64,
    performance_fee_bps: u64,
    max_slippage_bps: u64,
    cooldown_sec: u64,
    max_fee_usd: BigDecimal,
    benchmark_asset: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct DepositVaultArgs {
    vault_id: u64,
    deposit_spot_asset: String,
    deposit_amount: BigDecimal,
    subaccount_id: u64,
}

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct WithdrawVaultArgs {
    vault_id: u64,
    shares_to_burn: BigDecimal,
    subaccount_id: u64,
}

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct MintVaultSharesArgs {
    vault_id: u64,
    share_price: BigDecimal,
    user_action_hash: String,
    request_id: VaultRequestId,
}

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct BurnVaultSharesArgs {
    vault_id: u64,
    share_price: BigDecimal,
    user_action_hash: String,
    request_id: VaultRequestId,
}

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct CancelAllVaultRequestsArgs {
    subaccount_id: u64,
    vault_id: u64,
}

sol! {
    #![sol(all_derives)]

    struct CreateVaultData {
        uint256 action_kind;
        uint256 manager_id;
        address deposit_spot_asset_address;
        uint256 initial_deposit;
        uint32 management_fee_bps;
        uint32 performance_fee_bps;
        uint32 max_slippage_bps;
        uint32 cooldown_sec;
        uint256 max_fee_usd;
        uint256 initial_share_price_usd;
        address benchmark_asset_address;
        bool use_benchmark_asset;
    }

    struct DepositVaultData {
        uint256 action_kind;
        uint256 vault_id;
        uint256 deposit_spot_asset_address;
        uint256 deposit_amount;
    }

    struct WithdrawVaultData {
        uint256 action_kind;
        uint256 vault_id;
        uint256 shares_to_burn;
    }

    struct MintVaultSharesData {
        uint256 action_kind;
        uint256 share_price;
        bytes32 user_action_hash;
    }
    struct BurnVaultSharesData {
        uint256 action_kind;
        uint256 share_price;
        bytes32 user_action_hash;
    }

    struct CancelAllVaultRequestsData {
        uint256 action_kind;
        uint256 vault_id;
    }

}

impl ModuleData for CreateVaultData {
    fn get_action_data(&self) -> Vec<u8> {
        self.abi_encode()
    }
}

impl ModuleData for DepositVaultData {
    fn get_action_data(&self) -> Vec<u8> {
        self.abi_encode()
    }
}
impl ModuleData for MintVaultSharesData {
    fn get_action_data(&self) -> Vec<u8> {
        self.abi_encode()
    }
}
impl ModuleData for BurnVaultSharesData {
    fn get_action_data(&self) -> Vec<u8> {
        self.abi_encode()
    }
}

impl ModuleData for WithdrawVaultData {
    fn get_action_data(&self) -> Vec<u8> {
        self.abi_encode()
    }
}

impl ModuleData for CancelAllVaultRequestsData {
    fn get_action_data(&self) -> Vec<u8> {
        self.abi_encode()
    }
}

impl WithdrawVaultData {
    pub fn from_args(args: WithdrawVaultArgs) -> Self {
        Self {
            action_kind: VaultActionKind::Withdraw.into(),
            vault_id: U256::from(args.vault_id),
            shares_to_burn: to_e18(&args.shares_to_burn)
                .expect("Couldnt convert shares_to_burn to e18"),
        }
    }
}

impl CreateVaultData {
    pub fn from_args(args: CreateVaultArgs) -> Self {
        let benchmark_asset_address = args
            .benchmark_asset
            .clone()
            .map(|s| s.parse().unwrap())
            .unwrap_or_else(|| ZERO_ADDRESS.parse().unwrap());
        Self {
            action_kind: VaultActionKind::Create.into(),
            manager_id: U256::from(args.manager_id),
            deposit_spot_asset_address: args.deposit_spot_asset.parse().unwrap(),
            initial_deposit: to_e18(&args.initial_deposit)
                .expect("Couldnt convert initial_deposit to e18"),
            initial_share_price_usd: to_e18(&args.initial_share_price_usd)
                .expect("Couldnt convert initial_share_price_usd to e18"),
            management_fee_bps: args.management_fee_bps as u32,
            performance_fee_bps: args.performance_fee_bps as u32,
            max_slippage_bps: args.max_slippage_bps as u32,
            cooldown_sec: args.cooldown_sec as u32,
            max_fee_usd: to_e18(&args.max_fee_usd).expect("Couldnt convert max_fee_usd to e18"),
            benchmark_asset_address,
            use_benchmark_asset: args.benchmark_asset.is_some(),
        }
    }
}

impl DepositVaultData {
    pub fn from_args(args: DepositVaultArgs) -> Self {
        Self {
            action_kind: VaultActionKind::Deposit.into(),
            vault_id: U256::from(args.vault_id),
            deposit_spot_asset_address: args.deposit_spot_asset.parse().unwrap(),
            deposit_amount: to_e18(&args.deposit_amount)
                .expect("Couldnt convert deposit_amount to e18"),
        }
    }
}

impl MintVaultSharesData {
    pub fn from_args(args: MintVaultSharesArgs) -> Self {
        Self {
            action_kind: VaultActionKind::MintShares.into(),
            share_price: to_e18(&args.share_price).expect("Couldnt convert share_price to e18"),
            user_action_hash: args.user_action_hash.parse().unwrap(),
        }
    }
}

impl BurnVaultSharesData {
    pub fn from_args(args: BurnVaultSharesArgs) -> Self {
        Self {
            action_kind: VaultActionKind::BurnShares.into(),
            share_price: to_e18(&args.share_price).expect("Couldnt convert share_price to e18"),
            user_action_hash: args.user_action_hash.parse().unwrap(),
        }
    }
}

impl CancelAllVaultRequestsData {
    pub fn from_args(args: CancelAllVaultRequestsArgs) -> Self {
        Self {
            action_kind: VaultActionKind::Cancel.into(),
            vault_id: U256::from(args.vault_id),
        }
    }
}

impl ActionData {
    pub fn populate_create_vault_params(
        &self,
        signer: &PrivateKeySigner,
        args: CreateVaultArgs,
        env: &Environment,
    ) -> Result<CreateVaultRequest> {
        let encoded_data_hashed = &self.hash(env);
        let signature = format!("{}", signer.sign_hash_sync(encoded_data_hashed)?);
        let nonce = self.nonce.to_string().parse()?;
        let signature_expiry_sec = u64::try_from(&self.expiry)?;

        Ok(CreateVaultRequest {
            benchmark_asset: args.benchmark_asset.map(|s| s.parse().unwrap()),
            cooldown_sec: args.cooldown_sec,
            deposit_spot_asset: args.deposit_spot_asset.parse()?,
            initial_deposit: args.initial_deposit,
            initial_share_price_usd: args.initial_share_price_usd,
            management_fee_bps: args.management_fee_bps,
            manager_id: args.manager_id,
            max_fee_usd: args.max_fee_usd,
            max_slippage_bps: args.max_slippage_bps,
            nonce,
            performance_fee_bps: args.performance_fee_bps,
            signature_expiry_sec,
            signer: encode_prefixed(self.signer).parse()?,
            subaccount_id: args.subaccount_id,
            signature,
        })
    }

    pub fn populate_deposit_vault_params(
        &self,
        signer: &PrivateKeySigner,
        args: DepositVaultArgs,
        env: &Environment,
    ) -> Result<RequestVaultDepositRequest> {
        let encoded_data_hashed = &self.hash(env);
        let signature = format!("{}", signer.sign_hash_sync(encoded_data_hashed)?);
        let nonce = self.nonce.to_string().parse()?;
        let signature_expiry_sec = u64::try_from(&self.expiry)?;

        Ok(RequestVaultDepositRequest {
            amount: args.deposit_amount,
            deposit_spot_asset: args.deposit_spot_asset.parse()?,
            nonce,
            signature,
            signature_expiry_sec,
            signer: encode_prefixed(self.signer).parse()?,
            subaccount_id: args.subaccount_id,
            vault_subaccount_id: args.vault_id,
        })
    }

    pub fn populate_mint_vault_shares_params(
        &self,
        signer: &PrivateKeySigner,
        args: MintVaultSharesArgs,
        env: &Environment,
    ) -> Result<MintSharesRequest> {
        let encoded_data_hashed = &self.hash(env);
        let signature = format!("{}", signer.sign_hash_sync(encoded_data_hashed)?);
        let nonce = self.nonce.to_string().parse()?;
        let signature_expiry_sec = u64::try_from(&self.expiry)?;

        Ok(MintSharesRequest {
            deposit_hash: args.user_action_hash.parse()?,
            nonce,
            request_id: args.request_id,
            share_price: args.share_price,
            signature,
            signature_expiry_sec,
            signer: encode_prefixed(self.signer).parse()?,
            subaccount_id: args.vault_id,
        })
    }

    pub fn populate_burn_vault_shares_params(
        &self,
        signer: &PrivateKeySigner,
        args: BurnVaultSharesArgs,
        env: &Environment,
    ) -> Result<BurnSharesRequest> {
        let encoded_data_hashed = &self.hash(env);
        let signature = format!("{}", signer.sign_hash_sync(encoded_data_hashed)?);
        let nonce = self.nonce.to_string().parse()?;
        let signature_expiry_sec = u64::try_from(&self.expiry)?;
        Ok(BurnSharesRequest {
            nonce,
            request_id: args.request_id,
            share_price: args.share_price,
            signature,
            signature_expiry_sec,
            signer: encode_prefixed(self.signer).parse()?,
            subaccount_id: args.vault_id,
            withdraw_hash: args.user_action_hash.parse()?,
        })
    }

    pub fn populate_withdraw_vault_params(
        &self,
        signer: &PrivateKeySigner,
        args: WithdrawVaultArgs,
        env: &Environment,
    ) -> Result<RequestVaultWithdrawRequest> {
        let encoded_data_hashed = &self.hash(env);
        let signature = format!("{}", signer.sign_hash_sync(encoded_data_hashed)?);
        let nonce = self.nonce.to_string().parse()?;
        let signature_expiry_sec = u64::try_from(&self.expiry)?;

        Ok(RequestVaultWithdrawRequest {
            nonce,
            signature,
            signature_expiry_sec,
            signer: encode_prefixed(self.signer).parse()?,
            subaccount_id: args.subaccount_id,
            vault_subaccount_id: args.vault_id,
            shares_to_burn: args.shares_to_burn,
        })
    }

    pub fn populate_cancel_all_vault_requests_params(
        &self,
        signer: &PrivateKeySigner,
        args: CancelAllVaultRequestsArgs,
        env: &Environment,
    ) -> Result<CancelVaultRequestRequest> {
        let encoded_data_hashed = &self.hash(env);
        let signature = format!("{}", signer.sign_hash_sync(encoded_data_hashed)?);
        let nonce = self.nonce.to_string().parse()?;
        let signature_expiry_sec = u64::try_from(&self.expiry)?;

        Ok(CancelVaultRequestRequest {
            nonce,
            signature,
            signature_expiry_sec,
            signer: encode_prefixed(self.signer).parse()?,
            subaccount_id: args.subaccount_id,
            vault_subaccount_id: args.vault_id,
        })
    }
}
