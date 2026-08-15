use crate::{
    actions::{
        ActionData, ModuleType,
        session_key::{SetSessionKeyArgs, SetSessionKeyData},
    },
    models::PrivateSetSessionKeyResponse,
    types::ClientError,
    ws_client::WsClient,
};
pub struct SessionKeys<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> SessionKeys<'a> {
    pub async fn create(
        &self,
        session_key_args: SetSessionKeyArgs,
    ) -> Result<PrivateSetSessionKeyResponse, ClientError> {
        let signer = self
            .ws_client
            .wallet
            .clone()
            .expect("Must have set wallet to create session key");
        let scw_address = self
            .ws_client
            .derive_wallet
            .clone()
            .expect("Must have set smart contract wallet address to create session key");

        let set_session_key_data: SetSessionKeyData = session_key_args.clone().into();
        let action = ActionData::new(
            set_session_key_data,
            0,
            signer.address(),
            &scw_address.parse().expect("Couldnt unwrap."),
            &self.ws_client.environment,
            ModuleType::SetSessionKey,
        )?;
        let params = action.populate_set_session_key_params(
            &signer,
            session_key_args,
            &self.ws_client.environment,
            scw_address,
        )?;
        self.ws_client.rpc().other().set_session_key(params).await
    }
}
