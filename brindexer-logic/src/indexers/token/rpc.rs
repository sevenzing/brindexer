use super::types::TokenData;
use crate::{abi, IndexerJobError, RpcClient};
use alloy::sol_types::SolCall;
use alloy_primitives::Address;
use blockscout_db::entity::tokens;
use sea_orm::prelude::BigDecimal;
use std::str::FromStr;

pub async fn get_all_token_data_from_rpc<'a>(
    rpc: &RpcClient,
    tokens: Vec<tokens::Model>,
) -> Result<Vec<(tokens::Model, TokenData)>, IndexerJobError> {
    let names = get_batch_contracts_and_methods(rpc, abi::TokenData::nameCall {}, &tokens).await?;
    let symbols =
        get_batch_contracts_and_methods(rpc, abi::TokenData::symbolCall {}, &tokens).await?;
    let decimals =
        get_batch_contracts_and_methods(rpc, abi::TokenData::decimalsCall {}, &tokens).await?;
    let total_supplies =
        get_batch_contracts_and_methods(rpc, abi::TokenData::totalSupplyCall {}, &tokens).await?;
    let contract_uries =
        get_batch_contracts_and_methods(rpc, abi::TokenData::contractURICall {}, &tokens).await?;

    let data = tokens
        .into_iter()
        .zip(names)
        .zip(symbols)
        .zip(decimals)
        .zip(total_supplies)
        .zip(contract_uries)
        .map(
            |(((((token, name), symbol), decimal), total_supply), contract_uri)| {
                let mut token_data = TokenData {
                    name: name.map(|n| n._0).ok(),
                    symbol: symbol.map(|s| s._0).ok(),
                    decimals: decimal.map(|d| d._0.into()).ok(),
                    total_supply: total_supply
                        .map(|ts| BigDecimal::from_str(&ts._0.to_string()).unwrap())
                        .ok(),
                    contract_uri: contract_uri.map(|s| s._0).ok(),
                    skip_metadata: false,
                };
                if token_data.is_empty() {
                    token_data.skip_metadata = true;
                }
                (token, token_data)
            },
        )
        .collect();

    Ok(data)
}

async fn get_batch_contracts_and_methods<S: SolCall + Clone>(
    rpc: &RpcClient,
    method: S,
    tokens: &[tokens::Model],
) -> Result<Vec<Result<<S as SolCall>::Return, IndexerJobError>>, IndexerJobError> {
    let calls = tokens
        .iter()
        .map(|token| {
            (
                Address::from_slice(&token.contract_address_hash),
                method.clone(),
            )
        })
        .collect();

    let result = rpc
        .batch(calls)
        .await?
        .into_iter()
        .map(|r| r.map_err(IndexerJobError::RpcError))
        .collect::<Vec<_>>();
    Ok(result)
}
