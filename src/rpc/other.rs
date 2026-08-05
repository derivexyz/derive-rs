use crate::{models::openapi::*, types::ClientError, ws_client::WsClient};
pub struct OtherNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> OtherNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn liquidate(
        &self,
        params: PrivateLiquidateRequest,
    ) -> Result<PrivateLiquidateResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/liquidate", params_json).await
    }
    pub async fn set_session_key(
        &self,
        params: SetSessionKeyRequest,
    ) -> Result<PrivateSetSessionKeyResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/set_session_key", params_json).await
    }
    pub async fn start_auction(
        &self,
        params: PublicStartAuctionRequest,
    ) -> Result<PublicStartAuctionResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/start_auction", params_json).await
    }
}
