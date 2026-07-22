use crate::{models::openapi::*, types::ClientError, ws_client::WsClient};
pub struct OnchainActionsNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> OnchainActionsNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn get_onchain_action_history(
        &self,
        params: GetOnchainActionHistoryParams,
    ) -> Result<GetOnchainActionHistoryResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_onchain_action_history", params_json).await
    }
    pub async fn get_pending_deposits(
        &self,
        params: GetPendingDepositsParams,
    ) -> Result<GetPendingDepositsResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_pending_deposits", params_json).await
    }
    pub async fn register_deposit_address(
        &self,
        params: RegisterDepositAddressParams,
    ) -> Result<RegisterDepositAddressResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/register_deposit_address", params_json).await
    }
}
