use crate::{
    models::asyncapi_subs::*, subscriptions::channel_specs::trading::*,
    types::{ClientError, EventStream},
    ws_client::WsClient,
};
pub struct TradingSubscriptions<'a> {
    client: &'a WsClient,
}
impl<'a> TradingSubscriptions<'a> {
    pub fn new(client: &'a WsClient) -> Self {
        Self { client }
    }
    pub async fn subaccount_orders(
        &self,
        subaccount_id: &str,
    ) -> Result<EventStream<SubaccountOrdersNotification>, ClientError> {
        self.client
            .subscribe(SubaccountOrdersChannelSpec {
                subaccount_id: subaccount_id.to_owned(),
            })
            .await
    }
    pub async fn subaccount_trades(
        &self,
        subaccount_id: &str,
    ) -> Result<EventStream<SubaccountTradesNotification>, ClientError> {
        self.client
            .subscribe(SubaccountTradesChannelSpec {
                subaccount_id: subaccount_id.to_owned(),
            })
            .await
    }
    pub async fn subaccount_trades_tx_status(
        &self,
        subaccount_id: &str,
        tx_status: &str,
    ) -> Result<EventStream<SubaccountTradesTxStatusNotification>, ClientError> {
        self.client
            .subscribe(SubaccountTradesTxStatusChannelSpec {
                subaccount_id: subaccount_id.to_owned(),
                tx_status: tx_status.to_owned(),
            })
            .await
    }
}
