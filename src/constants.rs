use crate::Environment;

pub const REFFERAL_CODE: &str = "0x9135BA0f495244dc0A5F029b25CDE95157Db89AD";
pub const CLIENT_NAME: &str = "derive-rs";
pub const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

// V3 settlement contracts (Sepolia), per https://docs.derive.xyz/getting-started/contracts
pub const TESTNET_ACTION_MANAGER: &str = "0xd3625eCf97E5554C62A48Ac1c9284C9dCeFceB68";
pub const TESTNET_VAPP: &str = "0x1573bde26338A9E6AA358638679A796c81E33246";
pub const TESTNET_WITHDRAWAL_OUTBOX: &str = "0xFbB62CE2BbFdFdc8a60DDC22115aC29cf91B4566";
pub const TESTNET_SPOT_VAULT: &str = "0x3FB79aafCD401CDD19e3d729241545955DDE0D48";
pub const TESTNET_WITHDRAWAL_FACTORY: &str = "0x6A8566c585903E4558A9BE722FFa55f173c719ef";

pub const MAINNET_ACTION_MANAGER: &str = "0xE366CcA474968e33b777E13905829A3b800CFAD3";
pub const MAINNET_VAPP: &str = "0xd330145C17fB6EF2a21ACf1275Ce683A305F58BB";
pub const MAINNET_WITHDRAWAL_OUTBOX: &str = "0x7c743D79C9C595cE332314503d84cBBb4E85FA63";
pub const MAINNET_SPOT_VAULT: &str = "0x7c743D79C9C595cE332314503d84cBBb4E85FA63";
pub const MAINNET_WITHDRAWAL_FACTORY: &str = "0xFec48bBC648bd2284520d862Ec67fc68F709915e";

pub struct OnChainConstants {
    env: Environment,
}
impl OnChainConstants {
    pub fn new(env: Environment) -> Self {
        Self { env }
    }

    pub fn action_manager_address(&self) -> &str {
        match self.env {
            Environment::Mainnet => MAINNET_ACTION_MANAGER,
            Environment::Testnet => TESTNET_ACTION_MANAGER,
        }
    }

    pub fn vapp_address(&self) -> &str {
        match self.env {
            Environment::Mainnet => MAINNET_VAPP,
            Environment::Testnet => TESTNET_VAPP,
        }
    }

    pub fn withdrawal_outbox_address(&self) -> &str {
        match self.env {
            Environment::Mainnet => MAINNET_WITHDRAWAL_OUTBOX,
            Environment::Testnet => TESTNET_WITHDRAWAL_OUTBOX,
        }
    }

    pub fn spot_vault_address(&self) -> &str {
        match self.env {
            Environment::Mainnet => MAINNET_SPOT_VAULT,
            Environment::Testnet => TESTNET_SPOT_VAULT,
        }
    }

    pub fn withdrawal_factory_address(&self) -> &str {
        match self.env {
            Environment::Mainnet => MAINNET_WITHDRAWAL_FACTORY,
            Environment::Testnet => TESTNET_WITHDRAWAL_FACTORY,
        }
    }
}
