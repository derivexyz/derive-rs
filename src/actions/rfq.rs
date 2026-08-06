use std::{collections::HashMap, hash::Hash};

use alloy::hex::encode_prefixed;
use alloy::primitives::keccak256;
use alloy::signers::SignerSync;
use alloy::{
    primitives::{I256, U256},
    signers::local::PrivateKeySigner,
};
use alloy_sol_types::{SolValue, sol};
use anyhow::Result;
use bigdecimal::BigDecimal;
use serde::Deserialize;
use uuid::Uuid;

use crate::actions::utils::to_e18;
use crate::constants::{CLIENT_NAME, REFFERAL_CODE};
use crate::models::ExecuteQuoteRequest;
use crate::models::SendQuoteRequest;
use crate::{
    actions::{ActionData, ModuleData, ModuleType, utils::decimal_to_u256},
    models::{
        Address, Direction, Instrument, PricedLegParamsAndResponse, SignedTransferQuoteRequest,
        TransferPositionsRequest,
    },
    types::Environment,
};

use bon::Builder;

pub trait ExecuteQuoteArgsLike {
    fn max_fee(&self) -> &BigDecimal;
    fn legs(&self) -> &[PricedLegParamsAndResponse];
}

#[derive(Clone, Debug, Deserialize, Builder)]
pub struct TransferPositionsArgs {
    pub legs: Vec<PricedLegParamsAndResponse>,
    pub from_subaccount_id: u64,
    pub to_subaccount_id: u64,
    pub maker_direction: Direction,
    pub max_fee: BigDecimal,
}

impl ExecuteQuoteArgsLike for TransferPositionsArgs {
    fn max_fee(&self) -> &BigDecimal {
        &self.max_fee
    }

    fn legs(&self) -> &[PricedLegParamsAndResponse] {
        &self.legs
    }
}

#[derive(Clone, Debug, Deserialize, Builder)]
pub struct SendQuoteArgs {
    pub legs: Vec<PricedLegParamsAndResponse>,
    pub rfq_id: Uuid,
    pub max_fee: BigDecimal,
}

#[derive(Clone, Debug, Deserialize, Builder)]
pub struct ExecuteQuoteArgs {
    pub legs: Vec<PricedLegParamsAndResponse>,
    pub rfq_id: Uuid,
    pub quote_id: Uuid,
    pub max_fee: BigDecimal,
}

impl ExecuteQuoteArgsLike for ExecuteQuoteArgs {
    fn max_fee(&self) -> &BigDecimal {
        &self.max_fee
    }

    fn legs(&self) -> &[PricedLegParamsAndResponse] {
        &self.legs
    }
}

sol! {
    #![sol(all_derives)]
    struct RfqPositionTransferLeg {
        address asset;
        uint256 subId;
        uint256 price;
        int256 amount;
    }

    struct TransferPositionsData {
        uint256 maxFee;
        RfqPositionTransferLeg[] legs;
    }

    struct RfqExecuteData {
        bytes32 orderHash;
        uint256 maxFee;
    }
}

fn build_rfq_position_transfer_legs(
    legs: &[PricedLegParamsAndResponse],
    quote_direction: Direction,
    perspective_sign: i8,
    instrument_map: &HashMap<String, Instrument>,
) -> Result<Vec<RfqPositionTransferLeg>> {
    let mut sorted_legs = legs.to_vec();
    sorted_legs.sort_by(|a, b| a.instrument_name.cmp(&b.instrument_name));
    sorted_legs
        .into_iter()
        .map(|leg| {
            // let leg: RfqLeg = leg.into();

            let instrument = instrument_map
                .get(&leg.instrument_name)
                .expect("Didn't find leg");

            let asset = instrument
                .base_asset_address
                .parse::<Address>()
                .expect("Failed to parse base asset address");

            let leg_sign = match leg.direction {
                Direction::Buy => 1,
                Direction::Sell => -1,
            };

            let direction_sign = match quote_direction {
                Direction::Buy => 1,
                Direction::Sell => -1,
            };

            let sign = leg_sign * direction_sign * perspective_sign;

            let amount_magnitude = decimal_to_u256(&leg.amount)?;
            let mut signed_amount = I256::try_from(amount_magnitude)?;

            if sign < 0 {
                signed_amount = -signed_amount;
            }

            Ok(RfqPositionTransferLeg {
                asset: asset
                    .to_string()
                    .parse()
                    .expect("Failed to parse asset address"),
                subId: U256::from(
                    instrument
                        .base_asset_sub_id
                        .parse::<u64>()
                        .expect("Failed to parse subaccount id"),
                ),
                price: decimal_to_u256(&leg.price)?,
                amount: signed_amount,
            })
        })
        .collect()
}

impl TransferPositionsData {
    pub fn from_args(
        args: TransferPositionsArgs,
        quote_direction: Direction,
        perspective_sign: i8,
        instrument_map: &HashMap<String, Instrument>,
    ) -> Result<Self> {
        Ok(Self {
            maxFee: decimal_to_u256(&BigDecimal::from(0))?,
            legs: build_rfq_position_transfer_legs(
                &args.legs,
                quote_direction,
                perspective_sign,
                instrument_map,
            )?,
        })
    }

    pub fn from_send_quote_args(
        args: SendQuoteArgs,
        quote_direction: Direction,
        perspective_sign: i8,
        instrument_map: &HashMap<String, Instrument>,
    ) -> Result<Self> {
        Ok(Self {
            maxFee: to_e18(&args.max_fee).expect("Unable to scale fee"),
            legs: build_rfq_position_transfer_legs(
                &args.legs,
                quote_direction,
                perspective_sign,
                instrument_map,
            )?,
        })
    }
}

impl ModuleData for TransferPositionsData {
    fn get_action_data(&self) -> Vec<u8> {
        self.abi_encode()
    }
}

impl ModuleData for RfqExecuteData {
    fn get_action_data(&self) -> Vec<u8> {
        self.abi_encode()
    }
}

impl RfqExecuteData {
    // this is such that we can hash the legs and then use that hash to create the RfqExecuteData struct
    pub fn from_execute_quote_args<T>(
        execute_args: T,
        instrument_map: &HashMap<String, Instrument>,
        direction: Direction,
        perspective_sign: i8,
    ) -> Result<Self>
    where
        T: ExecuteQuoteArgsLike,
    {
        let leg_abis = build_rfq_position_transfer_legs(
            execute_args.legs(),
            direction,
            perspective_sign,
            instrument_map,
        )?;
        let order_hash = keccak256(leg_abis.abi_encode());
        println!("Order Hash: {:?}", order_hash);

        Ok(Self {
            orderHash: order_hash,
            maxFee: to_e18(execute_args.max_fee()).expect("Unable to scale fee"),
        })
    }
}

impl ActionData {
    pub fn populate_transfer_positions(
        signer: &PrivateKeySigner,
        args: TransferPositionsArgs,
        env: &Environment,
        wallet: &str,
        instruments: &HashMap<String, Instrument>,
    ) -> Result<TransferPositionsRequest> {
        let maker_module_data =
            TransferPositionsData::from_args(args.clone(), args.maker_direction, 1, instruments)?;
        let taker_module_data = RfqExecuteData::from_execute_quote_args(
            args.clone(),
            instruments,
            args.maker_direction.opposite(),
            -1,
        )?;

        let maker_action_data = ActionData::new(
            maker_module_data,
            args.from_subaccount_id,
            signer.address(),
            &wallet.parse().expect("Couldnt parse wallet address"),
            env,
            ModuleType::RfqPositionTransfer,
        )?;

        let taker_action_data = ActionData::new(
            taker_module_data,
            args.to_subaccount_id,
            signer.address(),
            &wallet.parse().expect("Couldnt parse wallet address"),
            env,
            ModuleType::RfqPositionTransfer,
        )?;

        let signer_address: Address = format!("{:?}", signer.address())
            .parse()
            .expect("Failed to parse signer address");

        Ok(TransferPositionsRequest {
            maker_params: SignedTransferQuoteRequest {
                direction: args.maker_direction,
                legs: args.legs.clone(),
                max_fee: BigDecimal::from(0),
                nonce: maker_action_data.nonce.to_string(),
                signature: format!(
                    "{}",
                    signer.sign_hash_sync(&maker_action_data.hash(env).clone())?
                ),
                signature_expiry_sec: maker_action_data
                    .expiry
                    .to_string()
                    .parse()
                    .expect("Failed to parse maker expiry"),
                signer: signer_address.clone(),
                subaccount_id: args.from_subaccount_id as i64,
            },
            taker_params: SignedTransferQuoteRequest {
                direction: args.maker_direction.opposite(),
                legs: args.legs.clone(),
                max_fee: BigDecimal::from(0),
                nonce: taker_action_data.nonce.to_string(),
                signature: format!(
                    "{}",
                    signer.sign_hash_sync(&taker_action_data.hash(env).clone())?
                ),
                signature_expiry_sec: taker_action_data
                    .expiry
                    .to_string()
                    .parse()
                    .expect("Failed to parse taker expiry"),
                signer: signer_address.clone(),
                subaccount_id: args.to_subaccount_id as i64,
            },
            wallet: wallet.parse().expect("Failed to parse wallet address"),
        })
    }

    pub fn populate_send_quote(
        &self,
        signer: &PrivateKeySigner,
        args: SendQuoteArgs,
        env: &Environment,
        subaccount_id: u64,
    ) -> Result<SendQuoteRequest> {
        let encoded_data_hashed = &self.hash(env);
        let signature = format!("{}", signer.sign_hash_sync(encoded_data_hashed)?);
        Ok(SendQuoteRequest {
            client: CLIENT_NAME.to_string(),
            direction: Direction::Sell,
            extra_fee: BigDecimal::from(0),
            legs: args.legs,
            max_fee: args.max_fee,
            nonce: self
                .nonce
                .to_string()
                .parse()
                .expect("Couldnt parse nonce to u64"),
            referral_code: REFFERAL_CODE.to_string(),
            rfq_id: args.rfq_id,
            signature,
            signature_expiry_sec: i64::try_from(&self.expiry)?,
            signer: encode_prefixed(signer.address())
                .parse()
                .expect("Couldnt parse signer address"),
            subaccount_id,
            label: "Test".to_string(),
            mmp: false,
        })
    }

    pub fn populate_execute_quote(
        &self,
        signer: &PrivateKeySigner,
        args: ExecuteQuoteArgs,
        env: &Environment,
        subaccount_id: u64,
    ) -> Result<ExecuteQuoteRequest> {
        let encoded_data_hashed = &self.hash(env);
        let signature = format!("{}", signer.sign_hash_sync(encoded_data_hashed)?);
        Ok(ExecuteQuoteRequest {
            client: CLIENT_NAME.to_string(),
            referral_code: REFFERAL_CODE.to_string(),
            direction: Direction::Buy,
            enable_taker_protection: false,
            label: "Test".to_string(),
            legs: args.legs,
            max_fee: args.max_fee,
            nonce: self
                .nonce
                .to_string()
                .parse()
                .expect("Couldnt parse nonce to u64"),
            quote_id: args.quote_id,
            rfq_id: args.rfq_id,
            signature,
            signature_expiry_sec: i64::try_from(&self.expiry)?,
            signer: encode_prefixed(signer.address())
                .parse()
                .expect("Couldnt parse signer address"),
            subaccount_id,
        })
    }
}
