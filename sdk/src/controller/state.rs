
#[cfg(not(target_arch = "wasm32"))]
use solana_client::nonblocking::rpc_client::RpcClient;

#[cfg(target_arch = "wasm32")]
use solana_client_wasm::nonblocking::rpc_client::WasmClient as RpcClient;
