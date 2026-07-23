use crate::{models::openapi::*, types::ClientError, ws_client::WsClient};
use crate::models::ticker_slim_schema::TickerSlimSchema;
pub struct MarketDataNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> MarketDataNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn get_all_currencies(&self) -> Result<Vec<Currency>, ClientError> {
        self.ws_client
            .send_rpc("public/get_all_currencies", serde_json::Value::Null)
            .await
    }
    pub async fn get_all_instruments(
        &self,
        params: GetAllInstrumentsRequest,
    ) -> Result<GetAllInstrumentsResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_all_instruments", params_json).await
    }
    pub async fn get_all_live_instruments(&self) -> Result<Vec<String>, ClientError> {
        self.ws_client
            .send_rpc("public/get_all_live_instruments", serde_json::Value::Null)
            .await
    }
    pub async fn get_assets(
        &self,
        params: GetAssetsRequest,
    ) -> Result<Vec<Asset>, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_assets", params_json).await
    }
    pub async fn get_currency(
        &self,
        params: GetCurrencyRequest,
    ) -> Result<Currency, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_currency", params_json).await
    }
    pub async fn get_funding_rate_history(
        &self,
        params: GetFundingRateHistoryRequest,
    ) -> Result<FundingRateHistoryResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_funding_rate_history", params_json).await
    }
    pub async fn get_index_chart_data(
        &self,
        params: GetIndexChartDataRequest,
    ) -> Result<Vec<IndexCandle>, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_index_chart_data", params_json).await
    }
    pub async fn get_instrument(
        &self,
        params: GetInstrumentRequest,
    ) -> Result<Instrument, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_instrument", params_json).await
    }
    pub async fn get_interest_rate_history(
        &self,
        params: GetInterestRateHistoryRequest,
    ) -> Result<InterestRateHistoryResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_interest_rate_history", params_json).await
    }
    pub async fn get_latest_signed_feeds(
        &self,
        params: GetLatestSignedFeedsRequest,
    ) -> Result<GetLatestSignedFeedsResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_latest_signed_feeds", params_json).await
    }
    pub async fn get_option_settlement_prices(
        &self,
        params: GetOptionSettlementPricesRequest,
    ) -> Result<OptionSettlementPricesResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_option_settlement_prices", params_json).await
    }
    pub async fn get_risk_universes(&self) -> Result<Vec<RiskUniverse>, ClientError> {
        self.ws_client
            .send_rpc("public/get_risk_universes", serde_json::Value::Null)
            .await
    }
    pub async fn get_ticker(
        &self,
        params: GetTickerRequest,
    ) -> Result<TickerSlimSchema, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_ticker", params_json).await
    }
    pub async fn get_tickers(
        &self,
        params: GetTickersRequest,
    ) -> Result<GetTickersResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_tickers", params_json).await
    }
    pub async fn get_trade_history(
        &self,
        params: GetPublicTradeHistoryRequest,
    ) -> Result<PublicTradesResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_trade_history", params_json).await
    }
    pub async fn get_tradingview_chart_data(
        &self,
        params: GetTradingviewChartDataRequest,
    ) -> Result<Vec<TradingviewCandle>, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_tradingview_chart_data", params_json).await
    }
}
