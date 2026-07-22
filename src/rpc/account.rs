use crate::{models::openapi::*, types::ClientError, ws_client::WsClient};
pub struct AccountNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> AccountNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn get_account(
        &self,
        params: GetAccountRequest,
    ) -> Result<PrivateGetAccountResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_account", params_json).await
    }
}
