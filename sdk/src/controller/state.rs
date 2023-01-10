
use solana_program::pubkey::Pubkey;
use gfx_controller_sdk::Controller;
use crate::error::{GfxSdkError, Result};

#[cfg(not(target_arch = "wasm32"))]
use solana_client::nonblocking::rpc_client::RpcClient;
#[cfg(target_arch = "wasm32")]
use solana_client_wasm::nonblocking::rpc_client::WasmClient as RpcClient;


pub async fn get_controller(address: &Pubkey, client: &RpcClient) -> Result<Controller> {
    let mut data = client.get_account_data(address).await
        .map_err(|_| GfxSdkError::AccountNotFound(address.clone()))?;
    let controller = Controller::try_deserialize(&mut data.as_slice())
        .map_err(|_| GfxSdkError::DeserializeFailure(
            address.clone(), "Controller".to_string())
        )?;
    Ok(controller)
}