use crate::{models::*, types::ClientError, ws_client::WsClient};
pub struct HistoryNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> HistoryNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn get_deposit_history(
        &self,
        params: GetDepositHistoryRequest,
    ) -> Result<DepositHistoryResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_deposit_history", params_json).await
    }
    pub async fn get_erc20_transfer_history(
        &self,
        params: GetErc20TransferHistoryRequest,
    ) -> Result<TransferHistoryResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_erc20_transfer_history", params_json).await
    }
    pub async fn get_funding_history(
        &self,
        params: GetFundingHistoryRequest,
    ) -> Result<PerpSettlementHistoryResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_funding_history", params_json).await
    }
    pub async fn get_interest_history(
        &self,
        params: GetInterestHistoryRequest,
    ) -> Result<InterestHistoryResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_interest_history", params_json).await
    }
    pub async fn get_option_settlement_history(
        &self,
        params: GetOptionSettlementHistoryParams,
    ) -> Result<OptionSettlementHistoryResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client
            .send_rpc("private/get_option_settlement_history", params_json)
            .await
    }
    pub async fn get_order_history(
        &self,
        params: GetOrderHistoryRequest,
    ) -> Result<PaginatedOrdersResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_order_history", params_json).await
    }
    pub async fn get_trade_history(
        &self,
        params: GetTradeHistoryRequest,
    ) -> Result<PaginatedTradesResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_trade_history", params_json).await
    }
    pub async fn get_withdrawal_history(
        &self,
        params: GetWithdrawalHistoryRequest,
    ) -> Result<WithdrawalHistoryResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_withdrawal_history", params_json).await
    }
    pub async fn get_liquidation_history(
        &self,
        params: GetLiquidationHistoryRequest,
    ) -> Result<LiquidationHistoryResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_liquidation_history", params_json).await
    }
}
