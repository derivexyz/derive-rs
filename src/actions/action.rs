use crate::types::Environment;
use alloy::{
    hex::encode,
    primitives::{Address, B256, U256, keccak256},
};
use alloy_sol_types::{SolValue, sol};
use anyhow::Result;
use chrono::{Duration, Utc};

pub trait ModuleData {
    fn get_action_data(&self) -> Vec<u8>;
}

pub enum ModuleType {
    Trade,
    SpotTransfer,
    Withdraw,
    RfqPositionTransfer,
    SetSessionKey,
    // ExternalTransfer,
    // WhitelistedRecipient,
    // Vault,
}

fn get_trade_module(module: ModuleType) -> &'static str {
    match module {
        ModuleType::Trade => "0xB8D20c2B7a1Ad2EE33Bc50eF10876eD3035b5e7b",
        ModuleType::SetSessionKey => "0xe330CF64ff6EbF41699aad344Cb21d78db1D2bb6",
        ModuleType::Withdraw => "0x9d0E8f5b25384C7310CB8C6aE32C8fbeb645d083",
        ModuleType::SpotTransfer => "0x01259207A40925b794C8ac320456F7F6c8FE2636",
        ModuleType::RfqPositionTransfer => "0x9371352CCef6f5b36EfDFE90942fFE622Ab77F1D",
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

pub fn get_domain_separator(env: &Environment) -> &'static str {
    match env {
        Environment::Mainnet => {
            "0xd96e5f90797da7ec8dc4e276260c7f3f87fedf68775fbe1ef116e996fc60441b"
        }
        Environment::Testnet => {
            "0x24d674cd5f2b9d564691c51e9d88f649b99246a2244dd74ce27b96578d773e85"
        }
    }
}

use anyhow::Context;
sol! {
    #![sol(all_derives)]

    struct ActionData {
        bytes32 action_typehash;
        uint256 subaccount_id;
        uint256 nonce;
        address module;
        bytes32 data;
        uint256 expiry;
        address owner;
        address signer;
    }
}

impl ActionData {
    fn get_nonce_and_expiry() -> Result<(U256, U256)> {
        let now = Utc::now();

        let nonce = now
            .timestamp_nanos_opt()
            .context("current timestamp cannot be represented in nanoseconds")?;
        let signature_expiry_sec = (now + Duration::seconds(3_500)).timestamp();
        Ok((
            U256::from(u64::try_from(nonce)?),
            U256::from(u64::try_from(signature_expiry_sec)?),
        ))
    }

    pub fn new<T>(
        module_data: T,
        subaccount_id: u64,
        signer_address: Address,
        derive_smart_contract_address: &Address,
        env: &Environment,
        module_type: ModuleType,
    ) -> Result<Self>
    where
        T: SolValue + ModuleData,
    {
        let encoded_data = module_data.get_action_data();
        let hex = format!("0x{}", encode(&encoded_data));
        println!("encoded_data: {hex}");
        let (nonce, expiry) = Self::get_nonce_and_expiry()?;
        let module = get_trade_module(module_type).parse::<Address>()?;
        println!("module: {:?}", module);
        let data = keccak256(&encoded_data);
        println!("encoded_data_hashed: {:?}", data);
        let action_typehash = get_action_typehash(env).parse::<B256>()?;
        println!("action_typehash: {:?}", action_typehash);
        Ok(Self {
            action_typehash,
            subaccount_id: U256::from(subaccount_id),
            nonce,
            module,
            data,
            expiry,
            owner: *derive_smart_contract_address,
            signer: signer_address,
        })
    }

    pub fn action_hash(&self) -> B256 {
        keccak256(self.abi_encode())
    }

    pub fn hash(&self, env: &Environment) -> B256 {
        let domain_separator = get_domain_separator(env)
            .parse::<B256>()
            .expect("invalid DOMAIN_SEPARATOR");
        let action_hash = self.action_hash();
        println!("domain_separator: {:?}", domain_separator);
        println!("action_hash: {:?}", action_hash);
        let mut encoded = [0u8; 66];
        encoded[0] = 0x19;
        encoded[1] = 0x01;
        encoded[2..34].copy_from_slice(domain_separator.as_slice());
        encoded[34..66].copy_from_slice(action_hash.as_slice());

        keccak256(encoded)
    }
}
