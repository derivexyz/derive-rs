use crate::{models::openapi::*, types::ClientError, ws_client::WsClient};
pub struct SubaccountsNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> SubaccountsNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn change_subaccount_label(
        &self,
        params: ChangeSubaccountLabelRequest,
    ) -> Result<PrivateChangeSubaccountLabelResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/change_subaccount_label", params_json).await
    }
    pub async fn get_all_portfolios(
        &self,
        params: GetAllPortfoliosRequest,
    ) -> Result<Vec<Subaccount>, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_all_portfolios", params_json).await
    }
    pub async fn get_collaterals(
        &self,
        params: GetCollateralsRequest,
    ) -> Result<PrivateGetCollateralsResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_collaterals", params_json).await
    }
    pub async fn get_positions(
        &self,
        params: GetPositionsRequest,
    ) -> Result<PrivateGetPositionsResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_positions", params_json).await
    }
    pub async fn get_subaccount(
        &self,
        params: GetSubaccountRequest,
    ) -> Result<Subaccount, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_subaccount", params_json).await
    }
    pub async fn get_subaccounts(
        &self,
        params: GetSubaccountsRequest,
    ) -> Result<PrivateGetSubaccountsResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_subaccounts", params_json).await
    }
}
