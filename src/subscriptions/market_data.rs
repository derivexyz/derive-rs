use crate::{
    models::asyncapi_subs::*, subscriptions::channel_specs::market_data::*,
    types::{ClientError, EventStream},
    ws_client::WsClient,
};
pub struct MarketDataSubscriptions<'a> {
    client: &'a WsClient,
}
impl<'a> MarketDataSubscriptions<'a> {
    pub fn new(client: &'a WsClient) -> Self {
        Self { client }
    }
    pub async fn orderbook(
        &self,
        instrument_name: &str,
        group: &str,
        depth: &str,
    ) -> Result<EventStream<OrderbookNotification>, ClientError> {
        self.client
            .subscribe(OrderbookChannelSpec {
                instrument_name: instrument_name.to_owned(),
                group: group.to_owned(),
                depth: depth.to_owned(),
            })
            .await
    }
    pub async fn spot_feed(
        &self,
        currency: &str,
    ) -> Result<EventStream<SpotFeedNotification>, ClientError> {
        self.client
            .subscribe(SpotFeedChannelSpec {
                currency: currency.to_owned(),
            })
            .await
    }
    pub async fn ticker_slim(
        &self,
        instrument_name: &str,
        interval: &str,
    ) -> Result<EventStream<TickerSlimNotification>, ClientError> {
        self.client
            .subscribe(TickerSlimChannelSpec {
                instrument_name: instrument_name.to_owned(),
                interval: interval.to_owned(),
            })
            .await
    }
    pub async fn trades_by_instrument(
        &self,
        instrument_name: &str,
    ) -> Result<EventStream<TradesByInstrumentNotification>, ClientError> {
        self.client
            .subscribe(TradesByInstrumentChannelSpec {
                instrument_name: instrument_name.to_owned(),
            })
            .await
    }
    pub async fn trades_by_instrument_type_currency(
        &self,
        instrument_type: &str,
        currency: &str,
    ) -> Result<EventStream<TradesByInstrumentTypeCurrencyNotification>, ClientError> {
        self.client
            .subscribe(TradesByInstrumentTypeCurrencyChannelSpec {
                instrument_type: instrument_type.to_owned(),
                currency: currency.to_owned(),
            })
            .await
    }
}
