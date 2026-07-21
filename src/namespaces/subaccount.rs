use crate::{
    models::{PrivateGetFundingHistoryParamsSchema, PrivateGetFundingHistoryResultSchema},
    types::ClientError,
    ws_client::WsClient,
};

pub struct SubaccountNamespace<'a> {
    pub ws_client: &'a WsClient,
}

impl<'a> SubaccountNamespace<'a> {
    pub async fn get_funding_history(
        &self,
        end_timestamp: Option<i64>,
        start_timestamp: Option<i64>,
        instrument_name: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<PrivateGetFundingHistoryResultSchema, ClientError> {
        let subaccount_id = self.ws_client.subaccount_id.unwrap();

        let params = PrivateGetFundingHistoryParamsSchema {
            end_timestamp,
            start_timestamp,
            subaccount_id: Some(Some(subaccount_id)),
            instrument_name: Some(instrument_name),
            page,
            page_size,
            wallet: None,
        };

        let params_json = serde_json::to_value(&params)?;

        let result: PrivateGetFundingHistoryResultSchema = self
            .ws_client
            .send_rpc("private/get_funding_history", params_json)
            .await?;

        Ok(result)
    }
}
