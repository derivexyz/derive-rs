use crate::{
    models::asyncapi_subs::*, subscriptions::channel_specs::liquidations::*,
    types::{ClientError, EventStream},
    ws_client::WsClient,
};
pub struct LiquidationsSubscriptions<'a> {
    client: &'a WsClient,
}
impl<'a> LiquidationsSubscriptions<'a> {
    pub fn new(client: &'a WsClient) -> Self {
        Self { client }
    }
    pub async fn auctions_watch(
        &self,
    ) -> Result<EventStream<AuctionsWatchNotification>, ClientError> {
        self.client.subscribe(AuctionsWatchChannelSpec {}).await
    }
    pub async fn margin_watch(
        &self,
    ) -> Result<EventStream<MarginWatchNotification>, ClientError> {
        self.client.subscribe(MarginWatchChannelSpec {}).await
    }
}
