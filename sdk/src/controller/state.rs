use solana_program::pubkey::Pubkey;
use gfx_controller_interface::{Controller, StakingAccount};
use crate::error::Result;

#[cfg(not(target_arch = "wasm32"))]
use solana_client::nonblocking::rpc_client::RpcClient;
#[cfg(target_arch = "wasm32")]
use solana_client_wasm::nonblocking::rpc_client::WasmClient as RpcClient;

use crate::utils::get_state;

pub async fn get_controller(address: &Pubkey, client: &RpcClient) -> Result<Controller> {
    get_state(address, client, "Controller").await
}

pub async fn get_staking_account(address: &Pubkey, client: &RpcClient) -> Result<StakingAccount> {
    get_state(address, client, "StakingAccount").await
}
