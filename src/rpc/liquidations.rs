use crate::{models::*, types::ClientError, ws_client::WsClient};
pub struct LiquidationsNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> LiquidationsNamespace<'a> {
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
    pub async fn get_liquidation_history(
        &self,
        params: GetLiquidationHistoryRequest,
    ) -> Result<LiquidationHistoryResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_liquidation_history", params_json).await
    }
    pub async fn get_live_auctions(
        &self,
    ) -> Result<PublicGetLiveAuctionsResponse, ClientError> {
        self.ws_client.send_rpc("public/get_live_auctions", serde_json::json!({})).await
    }
    pub async fn margin_watch(
        &self,
        params: MarginWatchRequest,
    ) -> Result<PublicMarginWatchResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/margin_watch", params_json).await
    }
    pub async fn start_auction(
        &self,
        params: PublicStartAuctionRequest,
    ) -> Result<PublicStartAuctionResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/start_auction", params_json).await
    }
}
