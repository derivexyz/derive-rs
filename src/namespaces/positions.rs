use crate::{
    models::{PrivateGetPositionsParamsSchema, PrivateGetPositionsResultSchema},
    types::ClientError,
    ws_client::WsClient,
};

pub struct PositionsNamespace<'a> {
    pub ws_client: &'a WsClient,
}

impl<'a> PositionsNamespace<'a> {
    /// Get current positions for the active subaccount
    pub async fn get(&self) -> Result<PrivateGetPositionsResultSchema, ClientError> {
        let subaccount_id = self
            .ws_client
            .subaccount_id
            .ok_or_else(|| "No subaccount_id set. Please login first.".to_string())
            .unwrap();

        let params = PrivateGetPositionsParamsSchema { subaccount_id };

        let params_json = serde_json::to_value(&params)?;

        self.ws_client
            .send_rpc("private/get_positions", params_json)
            .await
    }
}
