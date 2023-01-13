use crate::error::Result;
use gfx_ssl_sdk::{LiquidityAccount, Pair, SSL};
use solana_program::pubkey::Pubkey;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_client;

use crate::utils::{get_state, get_state_blocking};

pub async fn get_liquidity_account(
    address: &Pubkey,
    client: &RpcClient,
) -> Result<LiquidityAccount> {
    get_state(address, client, "LiquidityAccount").await
}

pub async fn get_pair(address: &Pubkey, client: &RpcClient) -> Result<Pair> {
    get_state(address, client, "Pair").await
}

pub async fn get_ssl(address: &Pubkey, client: &RpcClient) -> Result<SSL> {
    get_state(address, client, "SSL").await
}

pub fn get_liquidity_account_blocking(
    address: &Pubkey,
    client: &rpc_client::RpcClient,
) -> Result<LiquidityAccount> {
    get_state_blocking(address, client, "LiquidityAccount")
}

pub fn get_pair_blocking(address: &Pubkey, client: &rpc_client::RpcClient) -> Result<Pair> {
    get_state_blocking(address, client, "Pair")
}

pub fn get_ssl_blocking(address: &Pubkey, client: &rpc_client::RpcClient) -> Result<SSL> {
    get_state_blocking(address, client, "SSL")
}
