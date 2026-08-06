use crate::{models::*, types::ClientError, ws_client::WsClient};
pub struct MarketMakerProtectionNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> MarketMakerProtectionNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn get_mmp_config(
        &self,
        params: MmpScopeRequest,
    ) -> Result<Vec<MmpConfigResult>, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_mmp_config", params_json).await
    }
    pub async fn reset_mmp(
        &self,
        params: MmpScopeRequest,
    ) -> Result<ResetMmpResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/reset_mmp", params_json).await
    }
    pub async fn set_mmp_config(
        &self,
        params: SetMmpConfigRequest,
    ) -> Result<SetMmpConfigResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/set_mmp_config", params_json).await
    }
}
