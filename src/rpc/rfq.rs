use crate::{models::*, types::ClientError, ws_client::WsClient};
pub struct RfqNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> RfqNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn cancel_batch_quotes(
        &self,
        params: CancelBatchQuotesRequest,
    ) -> Result<CancelBatchResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/cancel_batch_quotes", params_json).await
    }
    pub async fn cancel_batch_rfqs(
        &self,
        params: CancelBatchRfqsRequest,
    ) -> Result<CancelBatchRfqsResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/cancel_batch_rfqs", params_json).await
    }
    pub async fn cancel_quote(
        &self,
        params: CancelQuoteRequest,
    ) -> Result<Quote, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/cancel_quote", params_json).await
    }
    pub async fn cancel_rfq(
        &self,
        params: CancelRfqRequest,
    ) -> Result<CancelRfqResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/cancel_rfq", params_json).await
    }
    pub async fn execute_quote(
        &self,
        params: ExecuteQuoteRequest,
    ) -> Result<QuoteExecuteResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/execute_quote", params_json).await
    }
    pub async fn get_quotes(
        &self,
        params: GetQuotesRequest,
    ) -> Result<QuoteGetResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_quotes", params_json).await
    }
    pub async fn get_rfqs(
        &self,
        params: GetRfqsRequest,
    ) -> Result<RfqGetResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_rfqs", params_json).await
    }
    pub async fn poll_quotes(
        &self,
        params: PollQuotesRequest,
    ) -> Result<QuotePollResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/poll_quotes", params_json).await
    }
    pub async fn poll_rfqs(
        &self,
        params: PollRfqsRequest,
    ) -> Result<RfqPollResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/poll_rfqs", params_json).await
    }
    pub async fn replace_quote(
        &self,
        params: ReplaceQuoteRequest,
    ) -> Result<QuoteReplaceResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/replace_quote", params_json).await
    }
    pub async fn rfq_get_best_quote(
        &self,
        params: RfqGetBestQuoteRequest,
    ) -> Result<RfqGetBestQuoteResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/rfq_get_best_quote", params_json).await
    }
    pub async fn send_quote(
        &self,
        params: SendQuoteRequest,
    ) -> Result<Quote, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/send_quote", params_json).await
    }
    pub async fn send_rfq(&self, params: SendRfqRequest) -> Result<Rfq, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/send_rfq", params_json).await
    }
    pub async fn execute_quote_debug(
        &self,
        params: PublicExecuteQuoteDebugRequest,
    ) -> Result<QuoteExecuteDebugResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/execute_quote_debug", params_json).await
    }
    pub async fn send_quote_debug(
        &self,
        params: PublicSendQuoteDebugRequest,
    ) -> Result<QuoteSendDebugResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/send_quote_debug", params_json).await
    }
}
