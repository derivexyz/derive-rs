use crate::subscriptions::{
    accounting::AccountingSubscriptions, liquidations::LiquidationsSubscriptions,
    market_data::MarketDataSubscriptions, rfqs::RfqsSubscriptions, trading::TradingSubscriptions,
};

pub mod accounting;
pub mod liquidations;
pub mod market_data;
pub mod rfqs;
pub mod trading;
// pub mod private;
pub mod channel_specs;

pub struct Subscriptions<'a> {
    pub client: &'a crate::ws_client::WsClient,
}

impl<'a> Subscriptions<'a> {
    pub fn market_data(&self) -> MarketDataSubscriptions<'a> {
        MarketDataSubscriptions::new(self.client)
    }
    pub fn accounting(&self) -> AccountingSubscriptions<'a> {
        AccountingSubscriptions::new(self.client)
    }
    pub fn liquidations(&self) -> LiquidationsSubscriptions<'a> {
        LiquidationsSubscriptions::new(self.client)
    }
    pub fn rfqs(&self) -> RfqsSubscriptions<'a> {
        RfqsSubscriptions::new(self.client)
    }
    pub fn trading(&self) -> TradingSubscriptions<'a> {
        TradingSubscriptions::new(self.client)
    }
}
