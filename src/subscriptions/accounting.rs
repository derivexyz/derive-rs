use crate::{
    models::*, subscriptions::channel_specs::accounting::*,
    types::{ClientError, EventStream},
    ws_client::WsClient,
};
pub struct AccountingSubscriptions<'a> {
    client: &'a WsClient,
}
impl<'a> AccountingSubscriptions<'a> {
    pub fn new(client: &'a WsClient) -> Self {
        Self { client }
    }
    pub async fn subaccount_balances(
        &self,
        subaccount_id: &str,
    ) -> Result<EventStream<SubaccountBalancesNotification>, ClientError> {
        self.client
            .subscribe(SubaccountBalancesChannelSpec {
                subaccount_id: subaccount_id.to_owned(),
            })
            .await
    }
}
