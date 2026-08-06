use crate::{models::*, types::ClientError, ws_client::WsClient};
pub struct OrderbookNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> OrderbookNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn cancel(
        &self,
        params: CancelOrderRequest,
    ) -> Result<Order, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/cancel", params_json).await
    }
    pub async fn cancel_algo_order(
        &self,
        params: CancelAlgoOrderRequest,
    ) -> Result<Order, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/cancel_algo_order", params_json).await
    }
    pub async fn cancel_all(
        &self,
        params: CancelAllRequest,
    ) -> Result<CancelAllResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/cancel_all", params_json).await
    }
    pub async fn cancel_all_algo_orders(
        &self,
        params: CancelAllAlgoOrdersRequest,
    ) -> Result<CancelAllAlgoOrdersResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/cancel_all_algo_orders", params_json).await
    }
    pub async fn cancel_all_trigger_orders(
        &self,
        params: CancelAllTriggerOrdersRequest,
    ) -> Result<CancelAllTriggerOrdersResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/cancel_all_trigger_orders", params_json).await
    }
    pub async fn cancel_by_instrument(
        &self,
        params: CancelByInstrumentRequest,
    ) -> Result<CancelByInstrumentResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/cancel_by_instrument", params_json).await
    }
    pub async fn cancel_by_label(
        &self,
        params: CancelByLabelRequest,
    ) -> Result<CancelByLabelResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/cancel_by_label", params_json).await
    }
    pub async fn cancel_by_nonce(
        &self,
        params: CancelByNonceRequest,
    ) -> Result<CancelByNonceResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/cancel_by_nonce", params_json).await
    }
    pub async fn cancel_trigger_order(
        &self,
        params: CancelTriggerOrderRequest,
    ) -> Result<Order, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/cancel_trigger_order", params_json).await
    }
    pub async fn get_algo_orders(
        &self,
        params: GetAlgoOrdersRequest,
    ) -> Result<Vec<Order>, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_algo_orders", params_json).await
    }
    pub async fn get_open_orders(
        &self,
        params: GetOpenOrdersRequest,
    ) -> Result<AggregatedOrdersResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_open_orders", params_json).await
    }
    pub async fn get_order(
        &self,
        params: GetOrderRequest,
    ) -> Result<Order, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_order", params_json).await
    }
    pub async fn get_trigger_orders(
        &self,
        params: GetTriggerOrdersRequest,
    ) -> Result<AggregatedTriggerOrdersResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_trigger_orders", params_json).await
    }
    pub async fn order(
        &self,
        params: CreateOrderRequest,
    ) -> Result<OrderCreatedResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/order", params_json).await
    }
    pub async fn order_debug(
        &self,
        params: CreateOrderRequest,
    ) -> Result<OrderDebugResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/order_debug", params_json).await
    }
    pub async fn order_quote(
        &self,
        params: OrderQuoteRequest,
    ) -> Result<OrderQuoteResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/order_quote", params_json).await
    }
    pub async fn replace(
        &self,
        params: ReplaceOrderRequest,
    ) -> Result<ReplaceOrderResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/replace", params_json).await
    }
    pub async fn public_order_quote(
        &self,
        params: OrderQuoteRequest,
    ) -> Result<OrderQuoteResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/order_quote", params_json).await
    }
}
