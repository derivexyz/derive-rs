use std::{collections::HashMap, hash::Hash, str::FromStr};

use alloy::signers::SignerSync;
use alloy::{
    primitives::{I256, U256},
    signers::local::PrivateKeySigner,
};
use alloy_sol_types::{SolValue, sol};
use anyhow::Result;
use bigdecimal::BigDecimal;
use serde::Deserialize;

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

sol! {
    #![sol(all_derives)]
    struct RfqPositionTransferLeg {
        address asset;
        uint256 amount;
        uint256 subaccountId;
        int256 direction;
    }

    struct TransferPositionsData {
        RfqPositionTransferLeg[] legs;
    }
}

impl TransferPositionsData {
    pub fn from_args(
        args: TransferPositionsArgs,
        direction: Direction,
        instrument_map: &HashMap<String, Instrument>,
    ) -> Result<Self> {
        let mut sorted_legs = args.legs.clone();

        sorted_legs.sort_by(|a, b| a.instrument_name.cmp(&b.instrument_name));
        let mut legs = Vec::new();
        for leg in sorted_legs {
            let instrument = instrument_map
                .get(&leg.instrument_name)
                .expect("Didnt find leg!");
            let address = instrument
                .base_asset_address
                .parse::<Address>()
                .expect("Failed to parse base asset address");

            let direction = match direction {
                Direction::Buy => 1,
                Direction::Sell => -1,
            };

            let leg = RfqPositionTransferLeg {
                asset: address.parse().expect("Failed to parse asset address"),
                amount: decimal_to_u256(&leg.amount)?,
                subaccountId: U256::from(if direction == 1 {
                    args.from_subaccount_id
                } else {
                    args.to_subaccount_id
                }),
                direction: I256::from_str(&direction.to_string())?,
            };
            legs.push(leg);
        }
        // we now sort the legs
        Ok(Self { legs })
    }
}

impl ModuleData for TransferPositionsData {
    fn get_action_data(&self) -> Vec<u8> {
        self.abi_encode()
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
            TransferPositionsData::from_args(args.clone(), args.maker_direction, instruments)?;
        let taker_module_data = TransferPositionsData::from_args(
            args.clone(),
            args.maker_direction.opposite(),
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
}
