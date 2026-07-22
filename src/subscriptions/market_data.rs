use crate::{
    models::asyncapi_subs::TickerSlimNotification,
    subscriptions::channel_specs::market_data::TickerSlimChannelSpec,
    types::{ClientError, EventStream},
};

// pub struct TickerChannelSpec {
//     pub instrument: String,
//     pub interval: String,
// }

// impl ChannelSpec for TickerChannelSpec {
//     type Output = TickerSlimNotification;

//     fn scope(&self) -> RequestScope {
//         RequestScope::Public
//     }

//     fn channel(&self) -> String {
//         let res = format!("ticker_slim.{}.{}", self.instrument, self.interval);
//         res
//     }
// }

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

    // pub async fn orderbook<F, Fut>(
    //     &self,
    //     instrument: &str,
    //     depth: Depth,
    //     group: Group,
    //     mut callback: F,
    // ) -> Result<String, ClientError>
    // where
    //     F: FnMut(
    //             crate::models::OrderbookInstrumentNameGroupDepthPublisherDataSchema,
    //             String,
    //         ) -> Fut
    //         + Send
    //         + 'static,
    //     Fut: Future<Output = ()> + Send + 'static,
    // {
    //     let channel = format!("orderbook.{instrument}.{group}.{depth}");
    //     self.client
    //         .subscribe_channel(
    //             RequestScope::Public,
    //             channel.clone(),
    //             move |msg: OrderbookNotification| callback(msg.params.data, msg.params.channel),
    //         )
    //         .await?;
    //     Ok(channel)
    // }
}
