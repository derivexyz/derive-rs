use crate::{
    actions::{ActionData, ModuleType, WithdrawArgs, WithdrawData},
    models::openapi::PrivateWithdrawResponse,
    types::ClientError,
    ws_client::WsClient,
};
pub struct FundMovementsNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> FundMovementsNamespace<'a> {
    pub async fn withdraw(
        &self,
        withdraw_args: WithdrawArgs,
    ) -> Result<PrivateWithdrawResponse, ClientError> {
        let subaccount_id = self.ws_client.subaccount_id.unwrap();
        let signer = self.ws_client.wallet.clone().unwrap();

        let wallet = self
            .ws_client
            .smart_contract_wallet_address
            .clone()
            .unwrap();
        let env = &self.ws_client.environment;

        let erc20_details = self.ws_client
            .erc20_cache
            .get(&withdraw_args.asset)
            .expect("ERC20 asset details not found in cache. Please ensure the asset is supported and cached.")
            .clone();

        let data = WithdrawData::from_args(withdraw_args.clone(), erc20_details.clone())?;
        let action = ActionData::new(
            data,
            subaccount_id,
            signer.address(),
            &wallet.parse().expect("Couldnt parse wallet address"),
            &self.ws_client.environment,
            ModuleType::Withdraw,
        )?;

        let params =
            action.populate_withdraw_params(&signer, withdraw_args.clone(), env, subaccount_id)?;

        println!("Withdrawal params: {:?}", params);

        self.ws_client
            .rpc()
            .transfers_withdrawals()
            .withdraw(params)
            .await
        // Ok(SignableRequest::new())
    }
}
