use crate::{
    models::{PrivateGetAllPortfoliosParamsSchema, PrivateGetSubaccountResultSchema},
    types::ClientError,
    ws_client::WsClient,
};

pub struct AccountNamespace<'a> {
    pub ws_client: &'a WsClient,
}

impl<'a> AccountNamespace<'a> {
    pub async fn get_all_portfolios(
        &self,
    ) -> Result<Vec<PrivateGetSubaccountResultSchema>, ClientError> {
        let wallet = self
            .ws_client
            .smart_contract_wallet_address
            .map(|addr| ethers::utils::to_checksum(&addr, None).to_string())
            .ok_or_else(|| "No LightAccount wallet address set. Please login first.".to_string())
            .unwrap();

        let params = PrivateGetAllPortfoliosParamsSchema { wallet };

        let params_json = serde_json::to_value(&params)?;

        self.ws_client
            .send_rpc("private/get_all_portfolios", params_json)
            .await
    }
}
