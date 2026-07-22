use serde::Deserialize;

use crate::models::{BalanceUpdateSchema, OrderResponseSchema, TradeResponseSchema};
use crate::{
    models::subaccount_id_balances_notification_params_schema::SubaccountIdBalancesNotificationParamsSchema,
    models::subaccount_id_orders_notification_params_schema::SubaccountIdOrdersNotificationParamsSchema,
    models::subaccount_id_trades_notification_params_schema::SubaccountIdTradesNotificationParamsSchema,
    types::{ClientError, RequestScope},
};

use std::future::Future;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct BalanceNotificationSchema {
    method: String,
    params: SubaccountIdBalancesNotificationParamsSchema,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct TradesNotificationSchema {
    method: String,
    params: SubaccountIdTradesNotificationParamsSchema,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct OrdersNotificationSchema {
    method: String,
    params: SubaccountIdOrdersNotificationParamsSchema,
}
pub struct PrivateChannels<'a> {
    client: &'a crate::ws_client::WsClient,
}

impl<'a> PrivateChannels<'a> {
    pub fn new(client: &'a crate::ws_client::WsClient) -> Self {
        Self { client }
    }

    pub async fn balances<F, Fut>(&self, mut callback: F) -> Result<String, ClientError>
    where
        F: FnMut(Vec<BalanceUpdateSchema>) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let subaccount_id = self.client.subaccount_id.unwrap();
        let channel = format!("{subaccount_id}.balances");
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel.clone(),
                move |msg: BalanceNotificationSchema| callback(msg.params.data),
            )
            .await?;
        Ok(channel)
    }
    pub async fn trades<F, Fut>(&self, mut callback: F) -> Result<String, ClientError>
    where
        F: FnMut(Vec<TradeResponseSchema>) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let subaccount_id = self.client.subaccount_id.unwrap();
        let channel = format!("{subaccount_id}.trades");
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel.clone(),
                move |msg: TradesNotificationSchema| callback(msg.params.data),
            )
            .await?;
        Ok(channel)
    }
    pub async fn orders<F, Fut>(&self, mut callback: F) -> Result<String, ClientError>
    where
        F: FnMut(Vec<OrderResponseSchema>) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let subaccount_id = self.client.subaccount_id.unwrap();
        let channel = format!("{subaccount_id}.orders");
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel.clone(),
                move |msg: OrdersNotificationSchema| callback(msg.params.data),
            )
            .await?;
        Ok(channel)
    }
}
