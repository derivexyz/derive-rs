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
    pub async fn liquidate(
        &self,
        params: PrivateLiquidateRequest,
    ) -> Result<PrivateLiquidateResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/liquidate", params_json).await
    }
    pub async fn get_maker_program_scores(
        &self,
        params: GetMakerProgramScoresParams,
    ) -> Result<GetMakerProgramScoresResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_maker_program_scores", params_json).await
    }
    pub async fn get_margin(
        &self,
        params: PublicGetMarginRequest,
    ) -> Result<GetMarginResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_margin", params_json).await
    }
    pub async fn set_socialization_feed_data(
        &self,
        params: PublicSetSocializationFeedDataRequest,
    ) -> Result<OperationAckResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/set_socialization_feed_data", params_json).await
    }
    pub async fn start_auction(
        &self,
        params: PublicStartAuctionRequest,
    ) -> Result<PublicStartAuctionResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/start_auction", params_json).await
    }
}
