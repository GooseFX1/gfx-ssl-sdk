use anchor_lang::AccountDeserialize;
use solana_program::pubkey::Pubkey;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_client;
use crate::error;
use crate::error::GfxSdkError;

pub async fn get_state<T: AccountDeserialize>(
    address: &Pubkey,
    client: &RpcClient,
    type_name: &str,
) -> error::Result<T> {
    let data = client.get_account_data(address).await
        .map_err(|_| GfxSdkError::AccountNotFound(address.clone()))?;
    let state = T::try_deserialize(&mut data.as_slice())
        .map_err(|_| GfxSdkError::DeserializeFailure(
            address.clone(), type_name.to_string())
        )?;
    Ok(state)
}

pub fn get_state_blocking<T: AccountDeserialize>(
    address: &Pubkey,
    client: &rpc_client::RpcClient,
    type_name: &str,
) -> error::Result<T> {
    let data = client.get_account_data(address)
        .map_err(|_| GfxSdkError::AccountNotFound(address.clone()))?;
    let state = T::try_deserialize(&mut data.as_slice())
        .map_err(|_| GfxSdkError::DeserializeFailure(
            address.clone(), type_name.to_string())
        )?;
    Ok(state)
}