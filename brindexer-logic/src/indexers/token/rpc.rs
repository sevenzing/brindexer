use super::types::TokenData;
use crate::{abi, IndexerJobError, RpcClient};
use alloy::sol_types::SolCall;
use alloy_primitives::Address;
use blockscout_db::entity::tokens;
use futures::StreamExt;
use sea_orm::prelude::BigDecimal;
use std::str::FromStr;

const MAX_CONCURRENT_REQUESTS: usize = 5;

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

    let futures = tokens
        .into_iter()
        .zip(names)
        .zip(symbols)
        .zip(decimals)
        .zip(total_supplies)
        .zip(contract_uries)
        .map(
            |(((((token, name), symbol), decimal), total_supply), contract_uri)| {
                let token_data = TokenData {
                    name: name.map(|n| n._0).ok(),
                    symbol: symbol.map(|s| s._0).ok(),
                    decimals: decimal.map(|d| d._0.into()).ok(),
                    total_supply: total_supply
                        .map(|ts| BigDecimal::from_str(&ts._0.to_string()).unwrap())
                        .ok(),
                    contract_uri: contract_uri.map(|s| s._0).ok(),
                };
                (token, token_data)
            },
        )
        .map(|(token, token_data)| async { (token, token_data.populate_with_contract_uri().await) })
        .collect::<Vec<_>>();

    let data = futures::stream::iter(futures)
        .buffered(MAX_CONCURRENT_REQUESTS)
        .collect::<Vec<_>>()
        .await;

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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::*;

    #[rstest]
    #[tokio::test]
    #[case(
        &[
            "0x54FA517F05e11Ffa87f4b22AE87d91Cec0C2D7E1",
            "0xdcF5D3E08c5007deCECDb34808C49331bD82a247",
            "0x489c5CB7fD158B0A9E7975076D758268a756C025",
        ],
        &
            [
     TokenData {
        name: Some("RareTron.io".into()),
        symbol: Some("RareTron.io".into()),
        decimals: Some(BigDecimal::from_str("0").unwrap()),
        total_supply: Some(BigDecimal::from_str("6666666666").unwrap()),
        contract_uri: None,
    },
    TokenData {
        name: Some("VanityTrx.org".into()),
        symbol: Some("VanityTrx.org".into()),
        decimals: Some(BigDecimal::from_str("0").unwrap()),
        total_supply: Some(BigDecimal::from_str("9999999999").unwrap()),
        contract_uri: None,
    },
    TokenData {
        name: Some("VanityTron.io".into()),
        symbol: Some("VanityTron.io".into()),
        decimals: Some(BigDecimal::from_str("0").unwrap()),
        total_supply: Some(BigDecimal::from_str("66666666666").unwrap()),         
        contract_uri: None},
        ]
    )]
    async fn test_get_all_token_data_from_rpc(
        #[case] token_contract_address_hashes: &[&str],
        #[case] expected: &[TokenData],
    ) {
        let rpc = RpcClient::from_url("https://eth-sepolia.public.blastapi.io".parse().unwrap());
        let tokens = token_contract_address_hashes
            .iter()
            .map(|contract_address_hashes| tokens::Model {
                contract_address_hash: blockscout_display_bytes::Bytes::from_str(
                    contract_address_hashes,
                )
                .unwrap()
                .to_vec(),
                name: None,
                symbol: None,
                total_supply: None,
                decimals: None,
                r#type: Default::default(),
                cataloged: None,
                inserted_at: Default::default(),
                updated_at: Default::default(),
                holder_count: None,
                skip_metadata: None,
                fiat_value: None,
                circulating_market_cap: None,
                total_supply_updated_at_block: None,
                icon_url: None,
                is_verified_via_admin_panel: None,
                volume_24h: None,
            })
            .collect();
        let result: Vec<_> = get_all_token_data_from_rpc(&rpc, tokens)
            .await
            .unwrap()
            .into_iter()
            .map(|(_, t)| t)
            .collect();

        assert_eq!(&result, expected);
    }
}
