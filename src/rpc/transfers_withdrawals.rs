use crate::{models::openapi::*, types::ClientError, ws_client::WsClient};
pub struct TransfersWithdrawalsNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> TransfersWithdrawalsNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn transfer_positions(
        &self,
        params: TransferPositionsRequest,
    ) -> Result<TransferPositionsResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/transfer_positions", params_json).await
    }
    pub async fn transfer_spot(
        &self,
        params: PrivateTransferSpotRequest,
    ) -> Result<PrivateTransferSpotResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/transfer_spot", params_json).await
    }
    pub async fn transfer_spot_external(
        &self,
        params: PrivateTransferSpotExternalRequest,
    ) -> Result<PrivateTransferSpotExternalResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/transfer_spot_external", params_json).await
    }
    pub async fn update_whitelisted_recipients(
        &self,
        params: UpdateWhitelistedRecipientsRequest,
    ) -> Result<UpdateWhitelistedRecipientsResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client
            .send_rpc("private/update_whitelisted_recipients", params_json)
            .await
    }
    pub async fn withdraw(
        &self,
        params: PrivateWithdrawRequest,
    ) -> Result<PrivateWithdrawResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/withdraw", params_json).await
    }
}
