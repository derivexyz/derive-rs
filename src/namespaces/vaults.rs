use tracing::debug;

use crate::{
    actions::{
        ActionData, BurnVaultSharesArgs, BurnVaultSharesData, CancelAllVaultRequestsArgs,
        CancelAllVaultRequestsData, CreateVaultArgs, CreateVaultData, DepositVaultArgs,
        DepositVaultData, MintVaultSharesArgs, MintVaultSharesData, ModuleType, WithdrawVaultArgs,
        WithdrawVaultData,
    },
    models::{
        VaultCancelResponse, VaultCreateResponse, VaultRequestAckResponse, VaultSettleResponse,
    },
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

        debug!(
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

        debug!(
            "Deposit Vault Params: {}",
            serde_json::to_string_pretty(&params).unwrap()
        );

        self.ws_client
            .rpc()
            .vault_shareholders()
            .request_vault_deposit(params)
            .await
    }

    pub async fn withdraw(
        &self,
        withdraw_args: WithdrawVaultArgs,
    ) -> Result<VaultRequestAckResponse, ClientError> {
        let module_data = WithdrawVaultData::from_args(withdraw_args.clone());

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

        let params = action_data.populate_withdraw_vault_params(
            &self.ws_client.wallet.clone().unwrap(),
            withdraw_args.clone(),
            &self.ws_client.environment,
        )?;

        debug!(
            "Withdraw Vault Params: {}",
            serde_json::to_string_pretty(&params).unwrap()
        );

        self.ws_client
            .rpc()
            .vault_shareholders()
            .request_vault_withdraw(params)
            .await
    }

    pub async fn mint_shares(
        &self,
        mint_shares_args: MintVaultSharesArgs,
    ) -> Result<VaultSettleResponse, ClientError> {
        let module_data = MintVaultSharesData::from_args(mint_shares_args.clone());
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

        let params = action_data.populate_mint_vault_shares_params(
            &self.ws_client.wallet.clone().unwrap(),
            mint_shares_args.clone(),
            &self.ws_client.environment,
        )?;

        debug!(
            "Mint Vault Shares Params: {}",
            serde_json::to_string_pretty(&params).unwrap()
        );

        self.ws_client
            .rpc()
            .vault_curators()
            .mint_vault_shares(params)
            .await
    }

    pub async fn burn_shares(
        &self,
        burn_shares_args: BurnVaultSharesArgs,
    ) -> Result<VaultSettleResponse, ClientError> {
        let module_data = BurnVaultSharesData::from_args(burn_shares_args.clone());
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

        let params = action_data.populate_burn_vault_shares_params(
            &self.ws_client.wallet.clone().unwrap(),
            burn_shares_args.clone(),
            &self.ws_client.environment,
        )?;

        debug!(
            "Burn Vault Shares Params: {}",
            serde_json::to_string_pretty(&params).unwrap()
        );

        self.ws_client
            .rpc()
            .vault_curators()
            .burn_vault_shares(params)
            .await
    }

    pub async fn cancel_all_vault_requests(
        &self,
        args: CancelAllVaultRequestsArgs,
    ) -> Result<VaultCancelResponse, ClientError> {
        let module_data = CancelAllVaultRequestsData::from_args(args.clone());
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

        let params = action_data.populate_cancel_all_vault_requests_params(
            &self.ws_client.wallet.clone().unwrap(),
            args.clone(),
            &self.ws_client.environment,
        )?;

        debug!(
            "Cancel All Vault Requests Params: {}",
            serde_json::to_string_pretty(&params).unwrap()
        );

        self.ws_client
            .rpc()
            .vault_shareholders()
            .cancel_all_vault_requests(params)
            .await
    }
}
