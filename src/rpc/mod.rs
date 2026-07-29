use crate::{
    rpc::{
        account::AccountNamespace, history::HistoryNamespace,
        market_data::MarketDataNamespace,
        market_maker_protection::MarketMakerProtectionNamespace,
        onchain_actions::OnchainActionsNamespace, orderbook::OrderbookNamespace,
        other::OtherNamespace, referrals::ReferralsNamespace, rfq::RfqNamespace,
        session_keys::SessionKeysNamespace, subaccounts::SubaccountsNamespace,
        system::SystemNamespace, transfers_withdrawals::TransfersWithdrawalsNamespace,
        vault_curators::VaultCuratorsNamespace,
        vault_shareholders::VaultShareholdersNamespace,
    },
    ws_client::WsClient,
};
pub mod account;
pub mod history;
pub mod market_data;
pub mod market_maker_protection;
pub mod onchain_actions;
pub mod orderbook;
pub mod other;
pub mod referrals;
pub mod rfq;
pub mod session_keys;
pub mod subaccounts;
pub mod system;
pub mod transfers_withdrawals;
pub mod vault_curators;
pub mod vault_shareholders;
pub struct Rpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> Rpc<'a> {
    pub fn new(client: &'a WsClient) -> Self {
        Self { client }
    }
    pub fn account(&self) -> AccountNamespace<'a> {
        AccountNamespace::new(self.client)
    }
    pub fn history(&self) -> HistoryNamespace<'a> {
        HistoryNamespace::new(self.client)
    }
    pub fn market_data(&self) -> MarketDataNamespace<'a> {
        MarketDataNamespace::new(self.client)
    }
    pub fn market_maker_protection(&self) -> MarketMakerProtectionNamespace<'a> {
        MarketMakerProtectionNamespace::new(self.client)
    }
    pub fn onchain_actions(&self) -> OnchainActionsNamespace<'a> {
        OnchainActionsNamespace::new(self.client)
    }
    pub fn orderbook(&self) -> OrderbookNamespace<'a> {
        OrderbookNamespace::new(self.client)
    }
    pub fn other(&self) -> OtherNamespace<'a> {
        OtherNamespace::new(self.client)
    }
    pub fn referrals(&self) -> ReferralsNamespace<'a> {
        ReferralsNamespace::new(self.client)
    }
    pub fn rfq(&self) -> RfqNamespace<'a> {
        RfqNamespace::new(self.client)
    }
    pub fn session_keys(&self) -> SessionKeysNamespace<'a> {
        SessionKeysNamespace::new(self.client)
    }
    pub fn subaccounts(&self) -> SubaccountsNamespace<'a> {
        SubaccountsNamespace::new(self.client)
    }
    pub fn system(&self) -> SystemNamespace<'a> {
        SystemNamespace::new(self.client)
    }
    pub fn transfers_withdrawals(&self) -> TransfersWithdrawalsNamespace<'a> {
        TransfersWithdrawalsNamespace::new(self.client)
    }
    pub fn vault_curators(&self) -> VaultCuratorsNamespace<'a> {
        VaultCuratorsNamespace::new(self.client)
    }
    pub fn vault_shareholders(&self) -> VaultShareholdersNamespace<'a> {
        VaultShareholdersNamespace::new(self.client)
    }
}
