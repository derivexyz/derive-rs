use crate::{models::*, types::ClientError, ws_client::WsClient};
pub struct SessionKeysNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> SessionKeysNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn edit_session_key(
        &self,
        params: EditSessionKeyRequest,
    ) -> Result<SessionKey, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/edit_session_key", params_json).await
    }
    pub async fn session_keys(
        &self,
        params: SessionKeysRequest,
    ) -> Result<PrivateSessionKeysResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/session_keys", params_json).await
    }
    pub async fn get_wallets_from_session_key(
        &self,
        params: GetWalletsFromSessionKeyRequest,
    ) -> Result<PublicGetWalletsFromSessionKeyResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_wallets_from_session_key", params_json).await
    }
}
