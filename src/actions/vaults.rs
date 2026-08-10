use alloy::signers::SignerSync;
use alloy::{hex::encode_prefixed, primitives::U256, signers::local::PrivateKeySigner, sol};
use alloy_sol_types::SolValue;
use anyhow::Result;
use bigdecimal::BigDecimal;
use bon::Builder;
use serde::Deserialize;

use crate::constants::ZERO_ADDRESS;
use crate::{
    Environment,
    actions::{ActionData, ModuleData, utils::to_e18},
    models::CreateVaultRequest,
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
}

impl ModuleData for CreateVaultData {
    fn get_action_data(&self) -> Vec<u8> {
        self.abi_encode()
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
}
