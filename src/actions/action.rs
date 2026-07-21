// use crate::actions::helpers::ModuleData;
use anyhow::Result;
use ethers::abi::AbiEncode;
use ethers::prelude::{Address, EthAbiCodec, EthAbiType, U256};
use ethers::utils::hex;
use tracing::debug;

use crate::types::Environment;

pub trait ModuleData {
    fn address(&self) -> Address;
}

pub enum ModuleType {
    Trade,
}

fn get_trade_module(env: &Environment, module: ModuleType) -> &'static str {
    match (env, module) {
        (Environment::Mainnet, ModuleType::Trade) => "0xB8D20c2B7a1Ad2EE33Bc50eF10876eD3035b5e7b",
        (Environment::Testnet, ModuleType::Trade) => "0x87F2863866D85E3192a35A73b388BD625D83f2be",
    }
}

fn get_action_typehash(env: &Environment) -> &'static str {
    match env {
        Environment::Mainnet => {
            "0x4d7a9f27c403ff9c0f19bce61d76d82f9aa29f8d6d4b0c5474607d9770d1af17"
        }
        Environment::Testnet => {
            "0x4d7a9f27c403ff9c0f19bce61d76d82f9aa29f8d6d4b0c5474607d9770d1af17"
        }
    }
}

fn get_domain_separator(env: &Environment) -> &'static str {
    match env {
        Environment::Mainnet => {
            "0xd96e5f90797da7ec8dc4e276260c7f3f87fedf68775fbe1ef116e996fc60441b"
        }
        Environment::Testnet => {
            "0x9bcf4dc06df5d8bf23af818d5716491b995020f377d3b7b64c29ed14e3dd1105"
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, EthAbiType, EthAbiCodec)]
pub struct ActionData {
    pub action_typehash: [u8; 32],
    pub subaccount_id: U256,
    pub nonce: U256,
    pub module: Address,
    pub data: [u8; 32],
    pub expiry: U256,
    pub owner: Address,
    pub signer: Address,
}

impl ActionData {
    fn get_nonce_and_expiry() -> (i64, i64) {
        let now = chrono::Utc::now();
        #[allow(deprecated)]
        // chrono::Utc::now().timestamp_nanos() is deprecated, but we need it for nonce This use is valid for the next 200 years, so we can ignore the deprecation warning until then..
        let nonce = now.timestamp_nanos();
        let signature_expiry_sec = (now + chrono::Duration::seconds(3_000_000)).timestamp();
        (nonce, signature_expiry_sec)
    }
    pub fn new<T: AbiEncode + ModuleData>(
        module_data: T,
        subaccount_id: i64,
        signer_address: Address,
        derive_smart_contract_address: &Address,
        env: &Environment,
        module_type: ModuleType,
    ) -> Result<ActionData> {
        let (nonce, signature_expiry_sec) = ActionData::get_nonce_and_expiry();
        let module_addr = get_trade_module(env, module_type).parse::<Address>()?;
        debug!("Using module address: {:?}", module_addr);
        let encoded_data = module_data.encode();
        debug!("Generated encoded_data: {:?}", hex::encode(&encoded_data));
        let hashed_data = ethers::utils::keccak256(&encoded_data);
        debug!(
            "generated encoded_data_hashed: {:?}",
            hex::encode(hashed_data)
        );
        let owner = *derive_smart_contract_address;
        let action_typehash = get_action_typehash(env);
        let action_typehash = hex::const_decode_to_array::<32>(action_typehash.as_bytes())?;

        Ok(ActionData {
            action_typehash,
            subaccount_id: subaccount_id.into(),
            nonce: nonce.into(),
            module: module_addr,
            data: hashed_data,
            expiry: signature_expiry_sec.into(),
            owner,
            signer: signer_address,
        })
    }

    fn action_hash(self) -> [u8; 32] {
        let action_hash = ethers::utils::keccak256(self.encode());
        debug!("action_hash: {:?}", hex::encode(action_hash));
        action_hash
    }

    pub fn hash(self, env: &Environment) -> [u8; 32] {
        let domain_sep = get_domain_separator(env);
        let domain_sep = hex::decode(domain_sep).expect("hex::decode failed for DOMAIN_SEPARATOR");
        let prefix = hex::decode("1901").expect("hex::decode failed for prefix");
        let action_hash = self.action_hash();
        let hash = ethers::utils::keccak256([prefix, domain_sep, action_hash.into()].concat());
        debug!("typed_data_hash: {:?}", hex::encode(hash));
        hash
    }
}
