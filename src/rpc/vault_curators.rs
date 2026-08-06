use crate::{models::*, types::ClientError, ws_client::WsClient};
pub struct VaultCuratorsNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> VaultCuratorsNamespace<'a> {
    pub fn new(ws_client: &'a WsClient) -> Self {
        Self { ws_client }
    }
    pub async fn burn_vault_shares(
        &self,
        params: BurnSharesRequest,
    ) -> Result<VaultSettleResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/burn_vault_shares", params_json).await
    }
    pub async fn create_vault(
        &self,
        params: CreateVaultRequest,
    ) -> Result<VaultCreateResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/create_vault", params_json).await
    }
    pub async fn force_burn(
        &self,
        params: ForceBurnRequest,
    ) -> Result<VaultForceBurnResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/force_burn", params_json).await
    }
    pub async fn get_curated_vaults(
        &self,
        params: GetCuratedVaultsRequest,
    ) -> Result<VaultIdsResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_curated_vaults", params_json).await
    }
    pub async fn get_live_burn_requests(
        &self,
        params: GetLiveBurnRequestsRequest,
    ) -> Result<MultipleVaultRequestsResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_live_burn_requests", params_json).await
    }
    pub async fn get_live_mint_requests(
        &self,
        params: GetLiveMintRequestsRequest,
    ) -> Result<MultipleVaultRequestsResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/get_live_mint_requests", params_json).await
    }
    pub async fn mint_vault_shares(
        &self,
        params: MintSharesRequest,
    ) -> Result<VaultSettleResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/mint_vault_shares", params_json).await
    }
    pub async fn reject_deposit_request(
        &self,
        params: RejectDepositRequestRequest,
    ) -> Result<VaultRequestAckResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/reject_deposit_request", params_json).await
    }
    pub async fn update_vault_info(
        &self,
        params: UpdateVaultInfoRequest,
    ) -> Result<OffchainAckResponse, ClientError> {
        let params_json = serde_json::to_value(&params)?;
        self.ws_client.send_rpc("private/update_vault_info", params_json).await
    }
}
