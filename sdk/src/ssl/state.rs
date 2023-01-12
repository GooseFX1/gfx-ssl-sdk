use solana_program::pubkey::Pubkey;
use crate::error::Result;
use gfx_ssl_sdk::{LiquidityAccount, Pair, SSL};

#[cfg(not(target_arch = "wasm32"))]
use solana_client::nonblocking::rpc_client::RpcClient;
#[cfg(target_arch = "wasm32")]
use solana_client_wasm::nonblocking::rpc_client::WasmClient as RpcClient;

use crate::utils::get_state;

pub async fn get_liquidity_account(address: &Pubkey, client: &RpcClient) -> Result<LiquidityAccount> {
    get_state(address, client, "LiquidityAccount").await
}

pub async fn get_pair(address: &Pubkey, client: &RpcClient) -> Result<Pair> {
    get_state(address, client, "Pair").await
}

pub async fn get_ssl(address: &Pubkey, client: &RpcClient) -> Result<SSL> {
    get_state(address, client, "SSL").await
}
