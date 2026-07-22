use crate::subscriptions::market_data::MarketDataSubscriptions;

pub mod market_data;
// pub mod private;
pub mod channel_specs;

pub struct Subscriptions<'a> {
    pub client: &'a crate::ws_client::WsClient,
}

impl<'a> Subscriptions<'a> {
    pub fn market_data(&self) -> MarketDataSubscriptions<'a> {
        MarketDataSubscriptions::new(self.client)
    }
    // pub fn private(&self) -> PrivateChannels<'a> {
    //     PrivateChannels::new(self.client)
    // }
}
