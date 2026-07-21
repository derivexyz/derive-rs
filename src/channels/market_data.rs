use serde::Deserialize;

use crate::{
    models::{
        Depth, Group, Interval, OrderbookInstrumentNameGroupDepthPublisherDataSchema,
        TickerSlimSchema,
    },
    types::{ClientError, RequestScope},
};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct TickerSlimInstrumentNameIntervalNotificationParamsSchema {
    timestamp: i64,
    instrument_ticker: TickerSlimSchema,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct TickerNotificationParams {
    channel: String,
    data: TickerSlimInstrumentNameIntervalNotificationParamsSchema,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct NotificationSchema {
    method: String,
    params: TickerNotificationParams,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct OrderbookNotificationParams {
    channel: String,
    data: OrderbookInstrumentNameGroupDepthPublisherDataSchema,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct OrderbookNotification {
    method: String,
    params: OrderbookNotificationParams,
}

pub struct MarketDataSubscriptions<'a> {
    client: &'a crate::ws_client::WsClient,
}
impl<'a> MarketDataSubscriptions<'a> {
    pub fn new(client: &'a crate::ws_client::WsClient) -> Self {
        Self { client }
    }
    pub async fn ticker<F, Fut>(
        &self,
        instrument: &str,
        interval: Interval,
        mut callback: F,
    ) -> Result<String, ClientError>
    where
        F: FnMut(TickerSlimSchema, String) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = format!("ticker_slim.{instrument}.{interval}",);
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel.clone(),
                move |msg: NotificationSchema| {
                    callback(msg.params.data.instrument_ticker, msg.params.channel)
                },
            )
            .await?;
        Ok(channel)
    }

    pub async fn orderbook<F, Fut>(
        &self,
        instrument: &str,
        depth: Depth,
        group: Group,
        mut callback: F,
    ) -> Result<String, ClientError>
    where
        F: FnMut(
                crate::models::OrderbookInstrumentNameGroupDepthPublisherDataSchema,
                String,
            ) -> Fut
            + Send
            + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = format!("orderbook.{instrument}.{group}.{depth}");
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel.clone(),
                move |msg: OrderbookNotification| callback(msg.params.data, msg.params.channel),
            )
            .await?;
        Ok(channel)
    }
}
