use crate::{
    actions::{
        ActionData, ModuleData,
        utils::{decimal_to_i256, decimal_to_u256},
    },
    models::openapi::{Direction, Instrument, OrderType, TimeInForce},
    types::Environment,
};
use alloy::{
    hex::encode_prefixed,
    primitives::{Address, U256},
    signers::local::PrivateKeySigner,
};
use alloy_sol_types::sol;
use anyhow::Result;
use bigdecimal::BigDecimal;
// use ethers::prelude::{Address, EthAbiCodec, EthAbiType, I256, LocalWallet, U256};
// use ethers::utils::hex;
use serde::Deserialize;

use crate::models::openapi::CreateOrderRequest as OrderParams;
// use crate::models::PrivateReplaceParamsSchema as ReplaceParams;

const REFFERAL_CODE: &str = "0x9135BA0f495244dc0A5F029b25CDE95157Db89AD";
const CLIENT_NAME: &str = "8ballers-rust-sdk";

use bon::Builder;

// “max_fee” here is a fascinating concept.
//
// One might assume a field called max_fee would depend on:
// - order size
// - order price
// - user balance or margin
//
// Naturally, it does none of those.
//
// Instead, it is a fixed, instrument-level constant derived from
// a price proxy and some metadata, then reused for every order
// regardless of notional. Large order, small order, same cap.
//
// Empirically, the API accepts virtually any size with this value,
// which means this “max” fee does not actually bound or scale with
// anything the user does.
//
// So despite the name, this is not a true maximum fee for an order.
// It is closer to a decorative upper-fee hint that keeps the API happy.
fn default_max_fee() -> BigDecimal {
    BigDecimal::from(1000u64)
}

// use bigdecimal::{BigDecimal, FromPrimitive};
#[derive(Clone, Debug, Deserialize, Builder)]
pub struct OrderArgs {
    #[builder(into)]
    pub amount: BigDecimal,
    #[builder(into)]
    pub limit_price: BigDecimal,
    pub direction: Direction,
    pub time_in_force: TimeInForce,
    pub order_type: OrderType,
    pub label: Option<String>,
    pub mmp: Option<bool>,
    pub instrument_name: String,
    pub reduce_only: Option<bool>,
    pub reject_post_only: Option<bool>,
}
pub fn get_reject_millis(time_in_force: &TimeInForce) -> Result<i64> {
    let reject_millis: i64 = 5000;
    let taker_speedbump: i64 = 150;
    let reject_millis = match time_in_force {
        TimeInForce::PostOnly => reject_millis,
        _ => reject_millis + taker_speedbump,
    };
    Ok(reject_millis)
}

sol! {
    #![sol(all_derives)]

    struct TradeData {
        address asset_address;
        uint256 sub_id;
        int256 limit_price;
        int256 amount;
        uint256 max_fee;
        uint256 subaccount_id;
        bool is_bid;
    }
}

impl TradeData {
    pub fn new(
        instrument: &Instrument,
        subaccount_id: u64,
        limit_price: BigDecimal,
        amount: BigDecimal,
        is_bid: bool,
    ) -> Result<Self> {
        Ok(Self {
            asset_address: instrument.base_asset_address.parse::<Address>()?,
            sub_id: U256::from(instrument.base_asset_sub_id.parse::<u128>()?),
            limit_price: decimal_to_i256(&limit_price)?,
            amount: decimal_to_i256(&amount)?,
            max_fee: decimal_to_u256(&default_max_fee())?,
            subaccount_id: U256::try_from(subaccount_id)?,
            is_bid,
        })
    }
}

impl ModuleData for TradeData {
    fn address(&self) -> Address {
        panic!(
            "ModuleData for TradeData should not be used directly, it should be encoded into ActionData with ActionData::new"
        );
    }
    fn get_action_data(&self) -> Vec<u8> {
        self.abi_encode()
    }
}
use alloy::signers::SignerSync;
use alloy_sol_types::SolValue;

impl ActionData {
    pub fn populate_order_params(
        self,
        signer: &PrivateKeySigner,
        args: OrderArgs,
        env: &Environment,
    ) -> Result<OrderParams> {
        // let reject_millis = get_reject_millis(&args.time_in_force)?;
        // we dont want to use the ticker as its fucking stupid.
        Ok(OrderParams {
            instrument_name: args.instrument_name,
            amount: args.amount,
            limit_price: args.limit_price,
            direction: args.direction,
            time_in_force: args.time_in_force,
            order_type: args.order_type,
            mmp: args.mmp,
            label: args.label,
            reduce_only: args.reduce_only,
            reject_post_only: args.reject_post_only,
            // reject_timestamp: effectively time till liveness
            reject_timestamp: None,
            // only used for vault controller orders
            is_atomic_signing: Some(false),
            // we can set these values from the action data
            subaccount_id: i64::try_from(&self.subaccount_id)?,
            max_fee: default_max_fee(),
            nonce: self.nonce.to_string(),
            signature_expiry_sec: i64::try_from(&self.expiry)?,
            signer: encode_prefixed(self.signer),
            referral_code: Some(REFFERAL_CODE.to_string()),
            signature: format!("{}", signer.sign_hash_sync(&self.hash(env).clone())?),
            client: Some(CLIENT_NAME.to_string()),
            trigger_price: None,
            trigger_price_type: None,
            trigger_type: None,
            extra_fee: None,
            algo_duration_sec: None,
            algo_num_slices: None,
            algo_type: None,
        })
    }

    // pub fn populate_replace_params(
    //     self,
    //     signer: &LocalWallet,
    //     args: ReplaceParams,
    //     env: &Environment,
    // ) -> Result<ReplaceParams> {
    //     // let reject_millis = get_reject_millis(&args.time_in_force)?;
    //     // we dont want to use the ticker as its fucking stupid.
    //     Ok(ReplaceParams {
    //         instrument_name: args.instrument_name,
    //         amount: args.amount,
    //         limit_price: args.limit_price,
    //         direction: args.direction,
    //         time_in_force: args.time_in_force,
    //         order_type: args.order_type,
    //         mmp: args.mmp,
    //         label: args.label,
    //         reduce_only: args.reduce_only,
    //         reject_post_only: args.reject_post_only,
    //         // reject_timestamp: effectively time till liveness
    //         reject_timestamp: None,
    //         // only used for vault controller orders
    //         is_atomic_signing: Some(Some(false)),
    //         // we can set these values from the action data
    //         subaccount_id: self.subaccount_id.as_u64() as i64,
    //         max_fee: default_max_fee(),
    //         nonce: self.nonce.as_u64() as i64,
    //         signature_expiry_sec: self.expiry.as_u64() as i64,
    //         signer: hex::encode_prefixed(self.signer),
    //         referral_code: Some(REFFERAL_CODE.to_string()),
    //         signature: format!("0x{}", signer.sign_hash(self.hash(env).into())?),
    //         client: Some(Some(CLIENT_NAME.to_string())),
    //         trigger_price: None,
    //         trigger_price_type: None,
    //         trigger_type: None,
    //         extra_fee: None,
    //         algo_duration_sec: None,
    //         algo_num_slices: None,
    //         algo_type: None,
    //         nonce_to_cancel: args.nonce_to_cancel,
    //         order_id_to_cancel: args.order_id_to_cancel,
    //         expected_filled_amount: args.expected_filled_amount,
    //     })
    // }
}
