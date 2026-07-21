use crate::channels::market_data::MarketDataSubscriptions;
use crate::channels::private::PrivateChannels;

pub struct Subscriptions<'a> {
    pub client: &'a crate::ws_client::WsClient,
}

impl<'a> Subscriptions<'a> {
    pub fn market_data(&self) -> MarketDataSubscriptions<'a> {
        MarketDataSubscriptions::new(self.client)
    }
    pub fn private(&self) -> PrivateChannels<'a> {
        PrivateChannels::new(self.client)
    }
}
