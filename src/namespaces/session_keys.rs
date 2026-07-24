use crate::{
    actions::{
        ActionData, ModuleType,
        session_key::{CreateSessionKeyArgs, CreateSessionKeyData},
    },
    models::openapi::PrivateCreateSessionKeyResponse,
    types::ClientError,
    ws_client::WsClient,
};
pub struct SessionKeys<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> SessionKeys<'a> {
    pub async fn create(
        &self,
        session_key_args: CreateSessionKeyArgs,
    ) -> Result<PrivateCreateSessionKeyResponse, ClientError> {
        let signer = self
            .ws_client
            .wallet
            .clone()
            .expect("Must have set wallet to create session key");
        let scw_address = self
            .ws_client
            .smart_contract_wallet_address
            .clone()
            .expect("Must have set smart contract wallet address to create session key");

        let create_session_key_data: CreateSessionKeyData = session_key_args.clone().into();
        let action = ActionData::new(
            create_session_key_data,
            0,
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
        self.ws_client
            .rpc()
            .session_keys()
            .create_session_key(params)
            .await
    }
}
