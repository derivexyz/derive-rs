use crate::{
    models::{PrivateGetCollateralsParamsSchema, PrivateGetCollateralsResultSchema},
    types::ClientError,
    ws_client::WsClient,
};

pub struct CollateralsNamespace<'a> {
    pub ws_client: &'a WsClient,
}

impl<'a> CollateralsNamespace<'a> {
    pub async fn get(&self) -> Result<PrivateGetCollateralsResultSchema, ClientError> {
        let subaccount_id = self
            .ws_client
            .subaccount_id
            .ok_or_else(|| "No subaccount_id set. Please login first.".to_string())
            .unwrap();

        let params = PrivateGetCollateralsParamsSchema { subaccount_id };

        let params_json = serde_json::to_value(&params)?;

        self.ws_client
            .send_rpc("private/get_collaterals", params_json)
            .await
    }
}
