use crate::{
    models::asyncapi_subs::*, subscriptions::channel_specs::rfqs::*,
    types::{ClientError, EventStream},
    ws_client::WsClient,
};
pub struct RfqsSubscriptions<'a> {
    client: &'a WsClient,
}
impl<'a> RfqsSubscriptions<'a> {
    pub fn new(client: &'a WsClient) -> Self {
        Self { client }
    }
    pub async fn subaccount_best_quotes(
        &self,
        subaccount_id: &str,
    ) -> Result<EventStream<SubaccountBestQuotesNotification>, ClientError> {
        self.client
            .subscribe(SubaccountBestQuotesChannelSpec {
                subaccount_id: subaccount_id.to_owned(),
            })
            .await
    }
    pub async fn subaccount_quotes(
        &self,
        subaccount_id: &str,
    ) -> Result<EventStream<SubaccountQuotesNotification>, ClientError> {
        self.client
            .subscribe(SubaccountQuotesChannelSpec {
                subaccount_id: subaccount_id.to_owned(),
            })
            .await
    }
    pub async fn wallet_rfqs(
        &self,
        wallet: &str,
    ) -> Result<EventStream<WalletRfqsNotification>, ClientError> {
        self.client
            .subscribe(WalletRfqsChannelSpec {
                wallet: wallet.to_owned(),
            })
            .await
    }
}
