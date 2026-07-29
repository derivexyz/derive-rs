use alloy::primitives::U256;
use alloy::signers::SignerSync;
use alloy::{hex::encode_prefixed, primitives::Address, signers::local::PrivateKeySigner};
use alloy_sol_types::{SolValue, sol};
use anyhow::Result;
use bigdecimal::BigDecimal;
use bon::Builder;
use serde::Deserialize;

use crate::models::openapi::{Asset, PrivateTransferSpotRequest};
use crate::{
    actions::{ActionData, ModuleData, utils::to_e18},
    models::openapi::SpotAssetEntry,
    types::Environment,
};

// export interface TransferFields {
//   toSubaccountId: number | bigint;
//   /**
//    * Routing sentinel: 0 credits the existing `toSubaccountId`; non-zero
//    * creates a new subaccount under this manager id instead (the
//    * destination id is then ignored).
//    */
//   newSubaccountManager: number | bigint;
//   /** Protocol spot-asset address (not the underlying ERC-20). */
//   asset: string;
//   subId: number | bigint;
//   /** Strictly positive; signed at e18. */
//   amount: DecimalLike;
//   /** Maximum fee the signer authorises, in USD; signed at e18. */
//   maxFeeUsd: DecimalLike;
// }

// const TRANSFER_ABI = ['uint256', 'uint256', 'address', 'uint256', 'uint256', 'uint256'];

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct SpotTransferArgs {
    #[builder(default = 0)]
    pub to_subaccount_id: u64,
    pub new_subaccount_manager: u32,
    pub asset: String,
    pub subaccount_id: u64,
    pub amount: BigDecimal,
    pub max_fee_usd: BigDecimal,
}

sol! {
    #![sol(all_derives)]

    struct SpotTransferData {
        uint256 toSubaccountId;
        uint256 newSubaccountManager;
        address protocol_asset_address;
        uint256 subId;
        uint256 amount;
        uint256 max_fee_usd;
    }
}

impl ModuleData for SpotTransferData {
    fn address(&self) -> Address {
        panic!(
            "ModuleData for SpotTransferData should not be used directly, it should be encoded into ActionData with ActionData::new"
        );
    }
    fn get_action_data(&self) -> Vec<u8> {
        self.abi_encode()
    }
}

impl SpotTransferData {
    pub fn from_args(
        args: SpotTransferArgs,
        erc20_details: SpotAssetEntry,
        asset: Asset,
    ) -> Result<Self> {
        let scaled_amt = to_e18(&args.amount)?;
        let scaled_fee = to_e18(&args.max_fee_usd)?;
        println!(
            "Scaled amount: {:?}, Scaled fee: {:?}",
            scaled_amt, scaled_fee
        );
        Ok(Self {
            toSubaccountId: U256::from(args.to_subaccount_id),
            newSubaccountManager: U256::from(args.new_subaccount_manager),
            protocol_asset_address: erc20_details
                .address
                .parse()
                .expect("Couldnt parse underlying_erc20_address"),
            subId: U256::from(
                asset
                    .sub_id
                    .parse::<u64>()
                    .expect("Couldnt parse base_asset_sub_id"),
            ),
            amount: scaled_amt,
            max_fee_usd: scaled_fee,
        })
    }
}

impl ActionData {
    pub fn populate_transfer_spot_params(
        self,
        signer: &PrivateKeySigner,
        args: SpotTransferArgs,
        env: &Environment,
        asset: &Asset,
    ) -> Result<PrivateTransferSpotRequest> {
        let encoded_data_hashed = &self.hash(env);
        let signature = format!("{}", signer.sign_hash_sync(encoded_data_hashed)?);
        let params = PrivateTransferSpotRequest {
            to_subaccount_id: args.to_subaccount_id,
            new_subaccount_manager: args.new_subaccount_manager,
            subaccount_id: args.subaccount_id,
            amount: args.amount,
            max_fee_usd: args.max_fee_usd,
            signature,
            asset_name: args.asset,
            nonce: self
                .nonce
                .to_string()
                .parse()
                .expect("Couldnt parse nonce to u64"),
            signature_expiry_sec: u64::try_from(&self.expiry)?,
            signer: encode_prefixed(self.signer),
            sub_id: asset
                .sub_id
                .parse::<u64>()
                .expect("Couldnt parse asset sub_id"),
        };
        Ok(params)
    }
}
