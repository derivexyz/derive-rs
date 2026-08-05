use std::{collections::HashMap, hash::Hash};

use alloy::hex::encode_prefixed;
use alloy::primitives::{B256, keccak256};
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
use bigdecimal::ToPrimitive;

use crate::actions::get_domain_separator;
use crate::actions::utils::to_e18;
use crate::constants::{CLIENT_NAME, REFFERAL_CODE};
use crate::models::openapi::SendQuoteRequest;
use crate::models::openapi::ExecuteQuoteRequest;
use crate::{
    actions::{ActionData, ModuleData, ModuleType, utils::decimal_to_u256},
    models::openapi::{
        Address, Direction, Instrument, PricedLegParamsAndResponse, SignedTransferQuoteRequest,
        TransferPositionsRequest,
    },
    types::Environment,
};

use bon::Builder;

#[derive(Clone, Debug, Deserialize, Builder)]
pub struct TransferPositionsArgs {
    pub legs: Vec<PricedLegParamsAndResponse>,
    pub from_subaccount_id: u64,
    pub to_subaccount_id: u64,
    pub maker_direction: Direction,
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

impl TransferPositionsData {
    pub fn from_args(
        args: TransferPositionsArgs,
        quote_direction: Direction,
        perspective_sign: i8,
        instrument_map: &HashMap<String, Instrument>,
    ) -> Result<Self> {
        let mut sorted_legs = args.legs;

        sorted_legs.sort_by(|a, b| a.instrument_name.cmp(&b.instrument_name));

        let direction_sign: i8 = match quote_direction {
            Direction::Buy => 1,
            Direction::Sell => -1,
        };

        let mut legs = Vec::with_capacity(sorted_legs.len());

        for leg in sorted_legs {
            let instrument = instrument_map
                .get(&leg.instrument_name)
                .expect("Didn't find leg");

            let asset = instrument
                .base_asset_address
                .parse::<Address>()
                .expect("Failed to parse base asset address");

            let leg_sign: i8 = match leg.direction {
                Direction::Buy => 1,
                Direction::Sell => -1,
            };

            let sign = leg_sign * direction_sign * perspective_sign;

            let amount_magnitude = decimal_to_u256(&leg.amount)?;
            let mut signed_amount = I256::try_from(amount_magnitude)?;

            if sign < 0 {
                signed_amount = -signed_amount;
            }

            legs.push(RfqPositionTransferLeg {
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
            });
        }

        Ok(Self {
            maxFee: decimal_to_u256(&BigDecimal::from(0))?,
            legs,
        })
    }

    pub fn from_send_quote_args(
        args: SendQuoteArgs,
        quote_direction: Direction,
        perspective_sign: i8,
        instrument_map: &HashMap<String, Instrument>,
    ) -> Result<Self> {
        let legs = args
            .legs
            .into_iter()
            .map(|leg| {
                let instrument = instrument_map
                    .get(&leg.instrument_name)
                    .expect("Didn't find leg");

                let asset = instrument
                    .base_asset_address
                    .parse::<Address>()
                    .expect("Failed to parse base asset address");

                let leg_sign: i8 = match leg.direction {
                    Direction::Buy => 1,
                    Direction::Sell => -1,
                };

                let direction_sign: i8 = match quote_direction {
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
            .collect::<Result<Vec<RfqPositionTransferLeg>>>()?;

        Ok(Self {
            maxFee: to_e18(&args.max_fee).expect("Unable to scale fee"),
            legs,
        })
    }

    pub fn from_execute_quote_args(
        args: ExecuteQuoteArgs,
        quote_direction: Direction,
        perspective_sign: i8,
        instrument_map: &HashMap<String, Instrument>,
    ) -> Result<Self> {
        let legs = args
            .legs
            .into_iter()
            .map(|leg| {
                let instrument = instrument_map
                    .get(&leg.instrument_name)
                    .expect("Didn't find leg");

                let asset = instrument
                    .base_asset_address
                    .parse::<Address>()
                    .expect("Failed to parse base asset address");

                let leg_sign: i8 = match leg.direction {
                    Direction::Buy => 1,
                    Direction::Sell => -1,
                };

                let direction_sign: i8 = match quote_direction {
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
            .collect::<Result<Vec<RfqPositionTransferLeg>>>()?;

        Ok(Self {
            maxFee: to_e18(&args.max_fee).expect("Unable to scale fee"),
            legs,
        })
    }

    pub fn inverse_legs(&mut self) {
        for leg in &mut self.legs {
            leg.amount = -leg.amount;
        }
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
    pub fn from_execute_quote_args(
        transfer_data: TransferPositionsData,
        execute_args: ExecuteQuoteArgs,
    ) -> Result<Self> {

        let inversed_transfer_data = {
            let mut data = transfer_data.clone();
            data.inverse_legs();
            data
        };


        let encoded_legs = inversed_transfer_data.abi_encode();
        let order_hash = keccak256(&encoded_legs);
        println!("Legs hashed: {:?}", order_hash);

        Ok(Self {
            orderHash: order_hash,
            maxFee: to_e18(&execute_args.max_fee).expect("Unable to scale fee"),
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
        let taker_module_data = TransferPositionsData::from_args(
            args.clone(),
            args.maker_direction.opposite(),
            -1,
            instruments,
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
            nonce: self.nonce.to_string().parse().expect("Couldnt parse nonce to u64"),
            referral_code: REFFERAL_CODE.to_string(),
            rfq_id: args.rfq_id,
            signature: signature,
            signature_expiry_sec: i64::try_from(&self.expiry)?,
            signer: encode_prefixed(signer.address()).parse().expect("Couldnt parse signer address"),
            subaccount_id: subaccount_id as u64,
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
        transfer_data: TransferPositionsData,
    ) -> Result<ExecuteQuoteRequest> {

        let order_hash = keccak256(RfqExecuteData::from_execute_quote_args(transfer_data, args.clone())?.abi_encode());
        println!("Order Hash: {:?}", order_hash);
        let encoded_data_hashed = &self.get_order_hash(env, order_hash.as_slice().to_vec());
        println!("Encoded Data Hashed: {:?}", encoded_data_hashed);

        let signature = format!("{}", signer.sign_hash_sync(encoded_data_hashed)?);
        Ok(ExecuteQuoteRequest {
            client: CLIENT_NAME.to_string(),
            referral_code: REFFERAL_CODE.to_string(),
            direction: Direction::Buy,
            enable_taker_protection: false,
            label: "Test".to_string(),
            legs: args.legs,
            max_fee: args.max_fee,
            nonce: self.nonce.to_string().parse().expect("Couldnt parse nonce to u64"),
            quote_id: args.quote_id,
            rfq_id: args.rfq_id,
            signature,
            signature_expiry_sec: i64::try_from(&self.expiry)?,
            signer: encode_prefixed(signer.address()).parse().expect("Couldnt parse signer address"),
            subaccount_id,
        })
    }

    fn get_order_hash(&self, env: &Environment, action_vec: Vec<u8>) -> B256 {
        let domain_separator = get_domain_separator(env)
            .parse::<B256>()
            .expect("invalid DOMAIN_SEPARATOR");

        //  * `orderHash = keccak256(abi.encode(trades[]))` — the maker's legs only,

        println!("domain_separator: {:?}", domain_separator);
        println!("action_vec: {:?}", action_vec);
        let mut encoded = [0u8; 66];
        encoded[0] = 0x19;
        encoded[1] = 0x01;
        encoded[2..34].copy_from_slice(domain_separator.as_slice());
        encoded[34..66].copy_from_slice(action_vec.as_slice());
        // we print out the bytes as a hex string for easier debugging
        println!("encoded: {:?}", encode_prefixed(encoded.to_vec()));

        keccak256(encoded)
    }
}
