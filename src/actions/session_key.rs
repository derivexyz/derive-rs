use alloy::{
    hex::{self, encode_prefixed},
    primitives::{Address, U256},
    signers::{SignerSync, local::PrivateKeySigner},
};
use alloy_sol_types::{SolValue, sol};
use anyhow::Result;
use bon::Builder;
use serde::{Deserialize, Serialize};
use strum::{Display, FromRepr};
use tracing::debug;

use crate::{
    actions::{ActionData, ModuleData},
    models::SetSessionKeyRequest,
    types::Environment,
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, FromRepr, Deserialize, Serialize)]
pub enum ProtocolScope {
    #[strum(to_string = "admin")]
    Admin = 0,
    #[strum(to_string = "withdraw")]
    Withdraw = 1,
    #[strum(to_string = "trade:all")]
    TradeAll = 2,
    #[strum(to_string = "trade:orderbook:all")]
    TradeOrderbookAll = 3,
    #[strum(to_string = "trade:orderbook:spot")]
    TradeOrderbookSpot = 4,
    #[strum(to_string = "trade:orderbook:perp")]
    TradeOrderbookPerp = 5,
    #[strum(to_string = "trade:orderbook:option")]
    TradeOrderbookOption = 6,
    #[strum(to_string = "trade:rfq:all")]
    TradeRfqAll = 7,
    #[strum(to_string = "trade:rfq:spot")]
    TradeRfqSpot = 8,
    #[strum(to_string = "trade:rfq:perp")]
    TradeRfqPerp = 9,
    #[strum(to_string = "trade:rfq:option")]
    TradeRfqOption = 10,
    #[strum(to_string = "transfer:all")]
    TransferAll = 11,
    #[strum(to_string = "transfer:existing_subaccount")]
    TransferExistingSubaccount = 12,
    #[strum(to_string = "transfer:new_subaccount")]
    TransferNewSubaccount = 13,
    #[strum(to_string = "transfer:different_owner_subaccount")]
    TransferDifferentOwnerSubaccount = 14,
    #[strum(to_string = "create_session_key")]
    CreateSessionKey = 15,
    #[strum(to_string = "liquidate")]
    Liquidate = 16,
    #[strum(to_string = "vault:all")]
    VaultAll = 17,
    #[strum(to_string = "vault:curator_create")]
    VaultCuratorCreate = 18,
    #[strum(to_string = "vault:curator_mint_and_burn")]
    VaultCuratorMintAndBurn = 19,
    #[strum(to_string = "vault:user_deposit")]
    VaultUserDeposit = 20,
    #[strum(to_string = "vault:user_withdraw")]
    VaultUserWithdraw = 21,
    #[strum(to_string = "vault:user_cancel")]
    VaultUserCancel = 22,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, FromRepr, Deserialize, Serialize)]
pub enum OffChainScope {
    #[strum(to_string = "account_info")]
    AccountInfo = 0,
}

// use bigdecimal::{BigDecimal, FromPrimitive};
#[derive(Clone, Debug, Deserialize, Builder)]
pub struct SetSessionKeyArgs {
    pub public_session_key: String,
    pub expiry_second: u64,
    pub protocol_scopes: Vec<ProtocolScope>,
    pub off_chain_scopes: Vec<OffChainScope>,
    pub label: String,
    pub subaccount_ids: Vec<u64>,
    pub ip_whitelist: Option<Vec<String>>,
}

sol! {
    #![sol(all_derives)]

    struct SetSessionKeyData {
        address session_key;
        uint256 expiry_sec;
        uint256[] scopes;
        uint256[] subaccount_ids;
    }
}

impl ModuleData for SetSessionKeyData {
    fn get_action_data(&self) -> Vec<u8> {
        let bytes = (
            self.session_key,
            self.expiry_sec,
            self.scopes.clone(),
            self.subaccount_ids.clone(),
        )
            .abi_encode_params();
        debug!("{}", hex::encode(&bytes));
        bytes
    }
}

impl From<SetSessionKeyArgs> for SetSessionKeyData {
    fn from(args: SetSessionKeyArgs) -> Self {
        let session_key = args
            .public_session_key
            .parse::<Address>()
            .expect("Invalid public session key address");
        let expiry_sec = U256::from(args.expiry_second);
        let scopes = args
            .protocol_scopes
            .into_iter()
            .map(|scope| U256::from(scope as u8))
            .collect();
        let subaccount_ids = args
            .subaccount_ids
            .into_iter()
            .map(|id| U256::from(id))
            .collect();

        Self {
            session_key,
            expiry_sec,
            scopes,
            subaccount_ids,
        }
    }
}

impl ActionData {
    pub fn populate_set_session_key_params(
        self,
        signer: &PrivateKeySigner,
        args: SetSessionKeyArgs,
        env: &Environment,
        scw_address: String,
    ) -> Result<SetSessionKeyRequest> {
        let encoded_data_hashed = &self.hash(env);
        debug!("typed_data_hash: {:?}", encoded_data_hashed);
        let signature = format!("{}", signer.sign_hash_sync(encoded_data_hashed)?);
        let offchain_scopes = args
            .off_chain_scopes
            .into_iter()
            .map(|scope| scope.to_string())
            .collect();
        let protocol_scopes = args
            .protocol_scopes
            .into_iter()
            .map(|scope| scope.to_string())
            .collect();
        Ok(SetSessionKeyRequest {
            expiry_sec: args.expiry_second,
            ip_whitelist: args.ip_whitelist,
            label: Some(args.label),
            nonce: self.nonce.to_string(),
            offchain_scopes,
            protocol_scopes,
            public_session_key: args.public_session_key,
            signature,
            signature_expiry_sec: u64::try_from(&self.expiry)?,
            signer: encode_prefixed(self.signer),
            subaccount_ids: Some(args.subaccount_ids),
            wallet: scw_address,
        })
    }
}
