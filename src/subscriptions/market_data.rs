use crate::{
    models::asyncapi_subs::*,
    subscriptions::channel_specs::market_data::*,
    types::{ClientError, EventStream},
};

pub struct MarketDataSubscriptions<'a> {
    client: &'a crate::ws_client::WsClient,
}
impl<'a> MarketDataSubscriptions<'a> {
    pub fn new(client: &'a crate::ws_client::WsClient) -> Self {
        Self { client }
    }
    pub async fn ticker(
        &self,
        instrument: &str,
        interval: &str,
    ) -> Result<EventStream<TickerSlimNotification>, ClientError> {
        self.client
            .subscribe(TickerSlimChannelSpec {
                instrument_name: instrument.to_owned(),
                interval: interval.to_owned(),
            })
            .await
    }
}
