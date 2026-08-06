use crate::{models::*, types::ClientError, ws_client::WsClient};
pub struct SystemNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> SystemNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn get_rate_limits(&self) -> Result<RateLimitResult, ClientError> {
        self.ws_client.send_rpc("public/getRateLimits", serde_json::json!({})).await
    }
    pub async fn get_time(&self) -> Result<i64, ClientError> {
        self.ws_client.send_rpc("public/get_time", serde_json::json!({})).await
    }
    pub async fn get_transaction(
        &self,
        params: GetTransactionParams,
    ) -> Result<GetTransactionResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_transaction", params_json).await
    }
}
