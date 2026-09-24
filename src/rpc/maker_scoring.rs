use crate::{models::*, types::ClientError, ws_client::WsClient};
pub struct MakerScoringNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> MakerScoringNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn get_maker_program_scores(
        &self,
        params: GetMakerProgramScoresParams,
    ) -> Result<GetMakerProgramScoresResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_maker_program_scores", params_json).await
    }
    pub async fn get_maker_programs(
        &self,
        params: GetMakerProgramsParams,
    ) -> Result<Vec<ProgramResponse>, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_maker_programs", params_json).await
    }
}
