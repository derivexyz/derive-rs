use crate::{models::*, types::ClientError, ws_client::WsClient};
pub struct OtherNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> OtherNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn get_subaccount_value_history(
        &self,
        params: GetSubaccountValueHistoryRequest,
    ) -> Result<SubaccountValueHistoryResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client
            .send_rpc("private/get_subaccount_value_history", params_json)
            .await
    }
    pub async fn set_socialization_feed_data(
        &self,
        params: PublicSetSocializationFeedDataRequest,
    ) -> Result<OperationAckResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/set_socialization_feed_data", params_json).await
    }
}
