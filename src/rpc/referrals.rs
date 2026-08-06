use crate::{models::*, types::ClientError, ws_client::WsClient};
pub struct ReferralsNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> ReferralsNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn get_all_referral_codes(
        &self,
        params: GetAllReferralCodesParams,
    ) -> Result<Vec<Referrer>, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_all_referral_codes", params_json).await
    }
    pub async fn get_referral_performance(
        &self,
        params: GetReferralPerformanceParams,
    ) -> Result<GetReferralPerformanceResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_referral_performance", params_json).await
    }
}
