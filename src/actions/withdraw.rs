use alloy::{
    hex::encode_prefixed,
    primitives::Address,
    signers::{SignerSync, local::PrivateKeySigner},
};
use alloy_sol_types::{SolValue, sol};
use anyhow::Result;
use bigdecimal::BigDecimal;
use bon::Builder;
use serde::Deserialize;

use crate::{
    actions::{
        ActionData, ModuleData,
        utils::{decimal_to_u256_with_prec, to_e18},
    },
    models::openapi::{PrivateWithdrawRequest, SpotAssetEntry},
    types::Environment,
};

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct WithdrawArgs {
    pub asset: String,
    pub max_fee_usd: BigDecimal,
    pub recepient_address: Address,
    pub amount: BigDecimal,
    pub force_batch: bool,
}

sol! {
    #![sol(all_derives)]

    struct WithdrawData {
        address protocol_asset_address;
        uint256 max_fee_usd;
        address recepient_address;
        uint256 amount;
        bool force_batch;
    }
}

impl ModuleData for WithdrawData {
    fn get_action_data(&self) -> Vec<u8> {
        self.abi_encode()
    }
}

impl WithdrawData {
    pub fn from_args(args: WithdrawArgs, er20_details: SpotAssetEntry) -> Result<Self> {
        Ok(Self {
            protocol_asset_address: er20_details
                .address
                .parse()
                .expect("Couldnt parse underlying_erc20_address"),
            max_fee_usd: to_e18(&args.max_fee_usd).expect("Couldnt convert max_fee_usd to e18"),
            recepient_address: args.recepient_address,
            amount: decimal_to_u256_with_prec(&args.amount, er20_details.erc20.decimals as u32)
                .expect("Couldnt convert amount to e18"),
            force_batch: args.force_batch,
        })
    }
}

impl ActionData {
    pub fn populate_withdraw_params(
        self,
        signer: &PrivateKeySigner,
        args: WithdrawArgs,
        env: &Environment,
        subaccount_id: u64,
    ) -> Result<PrivateWithdrawRequest> {
        let encoded_data_hashed = &self.hash(env);
        let signature = format!("{}", signer.sign_hash_sync(encoded_data_hashed)?);
        Ok(PrivateWithdrawRequest {
            amount_in_underlying: args.amount.to_string(),
            asset_name: args.asset,
            force_batch: args.force_batch,
            max_fee_usd: args.max_fee_usd,
            nonce: self
                .nonce
                .to_string()
                .parse()
                .expect("Couldnt parse nonce to u64"),
            signature,
            signature_expiry_sec: u64::try_from(&self.expiry)?,
            signer: encode_prefixed(self.signer),
            subaccount_id,
        })
    }
}
