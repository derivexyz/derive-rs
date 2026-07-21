use crate::actions::utils::{decimal_to_i256, decimal_to_u256};
use crate::actions::{ActionData, ModuleData};
use crate::models::{
    DirectionEnum, InstrumentPublicResponseSchema, OrderTypeEnum, TimeInForceEnum,
};
use crate::types::Environment;
use anyhow::Result;
use bigdecimal::BigDecimal;
use ethers::prelude::{Address, EthAbiCodec, EthAbiType, I256, LocalWallet, U256};
use ethers::utils::hex;
use serde::Deserialize;

use crate::models::PrivateOrderParamsSchema as OrderParams;
use crate::models::PrivateReplaceParamsSchema as ReplaceParams;

const REFFERAL_CODE: &str = "0x9135BA0f495244dc0A5F029b25CDE95157Db89AD";
const CLIENT_NAME: &str = "8ballers-rust-sdk";

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

#[derive(Clone, Debug, Deserialize)]
pub struct OrderArgs {
    pub amount: BigDecimal,
    pub limit_price: BigDecimal,
    pub direction: DirectionEnum,
    pub time_in_force: TimeInForceEnum,
    pub order_type: OrderTypeEnum,
    pub label: String,
    pub mmp: bool,
}

#[derive(Clone, Debug, Default, PartialEq, EthAbiCodec, EthAbiType)]
pub struct TradeData {
    pub asset_address: Address,
    pub sub_id: U256,
    pub limit_price: I256,
    pub amount: I256,
    pub max_fee: U256,
    pub subaccount_id: U256,
    pub is_bid: bool,
}

pub fn get_reject_millis(time_in_force: &TimeInForceEnum) -> Result<i64> {
    let reject_millis: i64 = 5000;
    let taker_speedbump: i64 = 150;
    let reject_millis = match time_in_force {
        TimeInForceEnum::PostOnly => reject_millis,
        _ => reject_millis + taker_speedbump,
    };
    Ok(reject_millis)
}

impl TradeData {
    pub fn new(
        instrument: &InstrumentPublicResponseSchema,
        // ticker: &TickerSlimSchema,
        subaccount_id: i64,
        limit_price: BigDecimal,
        amount: BigDecimal,
        is_bid: bool,
    ) -> Result<Self> {
        Ok(Self {
            asset_address: instrument.base_asset_address.parse()?,
            sub_id: instrument.base_asset_sub_id.parse::<u128>()?.into(),
            limit_price: decimal_to_i256(&limit_price)?,
            amount: decimal_to_i256(&amount)?,
            max_fee: decimal_to_u256(&default_max_fee())?,
            subaccount_id: subaccount_id.into(),
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
}

impl ActionData {
    pub fn populate_order_params(
        self,
        signer: &LocalWallet,
        args: OrderParams,
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
            is_atomic_signing: Some(Some(false)),
            // we can set these values from the action data
            subaccount_id: self.subaccount_id.as_u64() as i64,
            max_fee: default_max_fee(),
            nonce: self.nonce.as_u64() as i64,
            signature_expiry_sec: self.expiry.as_u64() as i64,
            signer: hex::encode_prefixed(self.signer),
            referral_code: Some(REFFERAL_CODE.to_string()),
            signature: format!("0x{}", signer.sign_hash(self.hash(env).into())?),
            client: Some(Some(CLIENT_NAME.to_string())),
            trigger_price: None,
            trigger_price_type: None,
            trigger_type: None,
            extra_fee: None,
            algo_duration_sec: None,
            algo_num_slices: None,
            algo_type: None,
        })
    }

    pub fn populate_replace_params(
        self,
        signer: &LocalWallet,
        args: ReplaceParams,
        env: &Environment,
    ) -> Result<ReplaceParams> {
        // let reject_millis = get_reject_millis(&args.time_in_force)?;
        // we dont want to use the ticker as its fucking stupid.
        Ok(ReplaceParams {
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
            is_atomic_signing: Some(Some(false)),
            // we can set these values from the action data
            subaccount_id: self.subaccount_id.as_u64() as i64,
            max_fee: default_max_fee(),
            nonce: self.nonce.as_u64() as i64,
            signature_expiry_sec: self.expiry.as_u64() as i64,
            signer: hex::encode_prefixed(self.signer),
            referral_code: Some(REFFERAL_CODE.to_string()),
            signature: format!("0x{}", signer.sign_hash(self.hash(env).into())?),
            client: Some(Some(CLIENT_NAME.to_string())),
            trigger_price: None,
            trigger_price_type: None,
            trigger_type: None,
            extra_fee: None,
            algo_duration_sec: None,
            algo_num_slices: None,
            algo_type: None,
            nonce_to_cancel: args.nonce_to_cancel,
            order_id_to_cancel: args.order_id_to_cancel,
            expected_filled_amount: args.expected_filled_amount,
        })
    }
}
