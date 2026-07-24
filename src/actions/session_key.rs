use alloy::{hex::encode_prefixed, primitives::{Address, Bytes, U256}, signers::local::PrivateKeySigner};
use anyhow::Result;
use alloy_sol_types::sol;
use bon::Builder;
use serde::{Deserialize, Serialize};
use alloy_sol_types::SolValue;

use strum::{Display, FromRepr};

use crate::{actions::{ActionData, ModuleData}, models::openapi::CreateSessionKeyRequest, types::Environment};

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
    #[strum(to_string = "delete_session_key")]
    DeleteSessionKey = 1,
}

// use bigdecimal::{BigDecimal, FromPrimitive};
#[derive(Clone, Debug, Deserialize, Builder)]
pub struct CreateSessionKeyArgs {
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

    struct CreateSessionKeyData {
        address session_key;
        uint256 expiry_sec;
        uint256[] scopes;
        uint256[] subaccount_ids;
    }
}

impl ModuleData for CreateSessionKeyData {
    fn address(&self) -> Address {
        panic!(
            "ModuleData for TradeData should not be used directly, it should be encoded into ActionData with ActionData::new"
        );
    }
    fn get_action_data(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(
            32 * (4 + self.scopes.len() + self.subaccount_ids.len()),
        );

        // Packed encoding of Address would normally be only 20 bytes.
        // into_word() left-pads it to the required 32-byte word.
        self.session_key
            .into_word()
            .abi_encode_packed_to(&mut output);

        self.expiry_sec.abi_encode_packed_to(&mut output);

        U256::from(self.scopes.len())
            .abi_encode_packed_to(&mut output);

        U256::from(self.subaccount_ids.len())
            .abi_encode_packed_to(&mut output);

        self.scopes.abi_encode_packed_to(&mut output);
        self.subaccount_ids
            .abi_encode_packed_to(&mut output);

        output.into()
    }
}

impl From<CreateSessionKeyArgs> for CreateSessionKeyData {
    fn from(args: CreateSessionKeyArgs) -> Self {
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

use alloy::signers::SignerSync;
impl ActionData {
    pub fn populate_create_session_key_params(
        self,
        signer: &PrivateKeySigner,
        args: CreateSessionKeyArgs,
        env: &Environment,
        scw_address: String,
    ) -> Result<CreateSessionKeyRequest> {
        let signature = format!("{}", signer.sign_hash_sync(&self.hash(env).clone())?);
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
        Ok(CreateSessionKeyRequest{
            expiry_sec: args.expiry_second,
            ip_whitelist: args.ip_whitelist,
            label: Some(args.label),
            nonce: self.nonce.to_string(),
            offchain_scopes: offchain_scopes,
            protocol_scopes: protocol_scopes,
            public_session_key: args.public_session_key,
            signature,
            signature_expiry_sec: u64::try_from(&self.expiry)?,
            signer: encode_prefixed(self.signer),
            subaccount_ids: Some(args.subaccount_ids),
            wallet: scw_address,
        })
    }
}