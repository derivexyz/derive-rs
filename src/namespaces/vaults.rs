use crate::{
    actions::{
        ActionData, CreateVaultArgs, CreateVaultData, DepositVaultArgs, DepositVaultData,
        ModuleType,
    },
    models::{VaultCreateResponse, VaultRequestAckResponse},
    types::ClientError,
    ws_client::WsClient,
};
pub struct VaultsNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> VaultsNamespace<'a> {
    pub async fn create(
        &self,
        create_args: CreateVaultArgs,
    ) -> Result<VaultCreateResponse, ClientError> {
        let module_data = CreateVaultData::from_args(create_args.clone());

        let action_data = ActionData::new(
            module_data,
            self.ws_client.subaccount_id.unwrap(),
            self.ws_client.wallet.clone().unwrap().address(),
            &self
                .ws_client
                .derive_wallet
                .clone()
                .unwrap()
                .parse()
                .expect("Couldnt parse wallet address"),
            &self.ws_client.environment,
            ModuleType::Vault,
        )?;

        let params = action_data.populate_create_vault_params(
            &self.ws_client.wallet.clone().unwrap(),
            create_args.clone(),
            &self.ws_client.environment,
        )?;

        println!(
            "Create Vault Params: {}",
            serde_json::to_string_pretty(&params).unwrap()
        );

        self.ws_client
            .rpc()
            .vault_curators()
            .create_vault(params)
            .await
    }

    pub async fn deposit(
        &self,
        deposit_args: DepositVaultArgs,
    ) -> Result<VaultRequestAckResponse, ClientError> {
        let module_data = DepositVaultData::from_args(deposit_args.clone());

        let action_data = ActionData::new(
            module_data,
            self.ws_client.subaccount_id.unwrap(),
            self.ws_client.wallet.clone().unwrap().address(),
            &self
                .ws_client
                .derive_wallet
                .clone()
                .unwrap()
                .parse()
                .expect("Couldnt parse wallet address"),
            &self.ws_client.environment,
            ModuleType::Vault,
        )?;

        let params = action_data.populate_deposit_vault_params(
            &self.ws_client.wallet.clone().unwrap(),
            deposit_args.clone(),
            &self.ws_client.environment,
        )?;

        println!(
            "Deposit Vault Params: {}",
            serde_json::to_string_pretty(&params).unwrap()
        );

        self.ws_client
            .rpc()
            .vault_shareholders()
            .request_vault_deposit(params)
            .await
    }
}
