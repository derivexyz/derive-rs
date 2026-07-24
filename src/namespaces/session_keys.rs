use crate::{
    actions::{ActionData, ModuleType, TradeData, session_key::{CreateSessionKeyArgs, CreateSessionKeyData}}, models::openapi::{Direction, OrderCreatedResponse, PrivateCreateSessionKeyResponse}, types::ClientError, ws_client::WsClient,
};
pub struct SessionKeys<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> SessionKeys<'a> {
    pub async fn create(&self, session_key_args: CreateSessionKeyArgs) -> Result<PrivateCreateSessionKeyResponse, ClientError> {
        let signer = self.ws_client.wallet.clone().expect("Must have set wallet to create session key");
        let subaccount_id = self.ws_client.subaccount_id.expect("Must have set subaccount id to create session key");
        let scw_address = self.ws_client.smart_contract_wallet_address.clone().expect("Must have set smart contract wallet address to create session key");


        let create_session_key_data: CreateSessionKeyData = session_key_args.clone().into();
        let action = ActionData::new(
            create_session_key_data,
            subaccount_id,
            signer.address(),
            &scw_address.parse().expect("Couldnt unwrap."),
            &self.ws_client.environment,
            ModuleType::CreateSessionKey,
        )?;
        let params = action.populate_create_session_key_params(
            &signer,
            session_key_args,
            &self.ws_client.environment,
            scw_address,
        )?;
        // let trade_data = TradeData::new(
        //     &instrument,
        //     // ticker,
        //     subaccount_id,
        //     order_args.limit_price.clone(),
        //     order_args.amount.clone(),
        //     order_args.direction == Direction::Buy,
        // )?;

        // let order_action = ActionData::new(
        //     trade_data,
        //     subaccount_id,
        //     signer.address(),
        //     &self
        //         .ws_client
        //         .smart_contract_wallet_address
        //         .clone()
        //         .unwrap()
        //         .parse()
        //         .expect("Couldnt unwrap."),
        //     &self.ws_client.environment,
        //     ModuleType::Trade,
        // )?;
        // let params = order_action.populate_order_params(
        //     &signer,
        //     order_args.clone(),
        //     &self.ws_client.environment,
        // )?;
        // // let params_json = serde_json::to_value(&params)?;

        // // we pretty print the params for debugging purposes
        // println!("Order params: {:?}", params);

        self.ws_client.rpc().session_keys().create_session_key(params).await
        // Ok(SignableRequest::new())
        // panic!("Session key creation is not yet implemented. This is a placeholder for the actual implementation.");
    }

}