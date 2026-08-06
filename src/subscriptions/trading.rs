use crate::{
    models::*, subscriptions::channel_specs::trading::*,
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
    pub async fn subaccount_trades_batch_status(
        &self,
        subaccount_id: &str,
        batch_status: &str,
    ) -> Result<EventStream<SubaccountTradesBatchStatusNotification>, ClientError> {
        self.client
            .subscribe(SubaccountTradesBatchStatusChannelSpec {
                subaccount_id: subaccount_id.to_owned(),
                batch_status: batch_status.to_owned(),
            })
            .await
    }
    pub async fn trades_by_instrument_type_currency_batch_status(
        &self,
        instrument_type: &str,
        currency: &str,
        batch_status: &str,
    ) -> Result<
        EventStream<TradesByInstrumentTypeCurrencyBatchStatusNotification>,
        ClientError,
    > {
        self.client
            .subscribe(TradesByInstrumentTypeCurrencyBatchStatusChannelSpec {
                instrument_type: instrument_type.to_owned(),
                currency: currency.to_owned(),
                batch_status: batch_status.to_owned(),
            })
            .await
    }
}
