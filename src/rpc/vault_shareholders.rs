use crate::{models::*, types::ClientError, ws_client::WsClient};
pub struct VaultShareholdersNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> VaultShareholdersNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn cancel_all_vault_requests(
        &self,
        params: CancelVaultRequestRequest,
    ) -> Result<VaultCancelResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/cancel_all_vault_requests", params_json).await
    }
    pub async fn get_live_vault_requests(
        &self,
        params: GetLiveVaultRequestsRequest,
    ) -> Result<MultipleVaultRequestsResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_live_vault_requests", params_json).await
    }
    pub async fn get_shareholder_vaults(
        &self,
        params: GetShareholderVaultsRequest,
    ) -> Result<VaultIdsResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_shareholder_vaults", params_json).await
    }
    pub async fn get_vault_request_history(
        &self,
        params: GetVaultRequestHistoryRequest,
    ) -> Result<PaginatedVaultRequestHistory, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_vault_request_history", params_json).await
    }
    pub async fn get_vault_shares(
        &self,
        params: GetVaultSharesRequest,
    ) -> Result<VaultSharesResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_vault_shares", params_json).await
    }
    pub async fn request_vault_deposit(
        &self,
        params: RequestVaultDepositRequest,
    ) -> Result<VaultRequestAckResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/request_vault_deposit", params_json).await
    }
    pub async fn request_vault_withdraw(
        &self,
        params: RequestVaultWithdrawRequest,
    ) -> Result<VaultRequestAckResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/request_vault_withdraw", params_json).await
    }
    pub async fn get_vault(
        &self,
        params: GetVaultRequest,
    ) -> Result<Vault, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_vault", params_json).await
    }
    pub async fn get_vault_action_history(
        &self,
        params: GetVaultActionHistoryRequest,
    ) -> Result<PaginatedVaultActionHistory, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_vault_action_history", params_json).await
    }
    pub async fn get_vault_performance_history(
        &self,
        params: GetVaultPerformanceHistoryRequest,
    ) -> Result<VaultPerformanceHistoryResult, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client
            .send_rpc("public/get_vault_performance_history", params_json)
            .await
    }
    pub async fn get_vaults(
        &self,
        params: GetVaultsRequest,
    ) -> Result<VaultsResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("public/get_vaults", params_json).await
    }
}
