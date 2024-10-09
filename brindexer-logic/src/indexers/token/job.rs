use std::{collections::HashMap, hash::Hash};

use crate::{
    abi,
    indexers::runtime::{IndexerJob, IndexerJobContext, IndexerJobError},
    repository, RpcClient,
};
use alloy::sol_types::SolCall;
use alloy_primitives::Address;
use blockscout_db::entity::tokens;
use derive_new::new;
use sea_orm::ConnectionTrait;

#[derive(Debug, Clone, new)]
pub struct TokenDataJob {
    batch_size: u64,
}

#[async_trait::async_trait]
impl IndexerJob for TokenDataJob {
    fn name(&self) -> &'static str {
        "token_data_job"
    }

    fn schedule(&self) -> &'static str {
        "every 10 seconds"
    }

    async fn execute(&self, ctx: &IndexerJobContext) -> Result<(), IndexerJobError> {
        tracing::info!("run token data job with retries: {}", ctx.retries);
        let tokens =
            repository::tokens::fetch_uncataloged_tokens(ctx.db.as_ref(), self.batch_size).await?;
        process_uncataloged_tokens(ctx.rpc.as_ref(), ctx.db.as_ref(), tokens).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct TokenData {
    name: Option<String>,
    symbol: Option<String>,
    decimals: Option<u8>,
    total_supply: Option<String>,
}

async fn process_uncataloged_tokens<C: ConnectionTrait>(
    rpc: &RpcClient,
    db: &C,
    tokens: Vec<tokens::Model>,
) -> Result<(), IndexerJobError> {
    let tokens_data = get_all_token_data(rpc, db, &tokens).await?;

    dbg!(&tokens_data);
    todo!()
}

async fn get_all_token_data<C: ConnectionTrait>(
    rpc: &RpcClient,
    db: &C,
    tokens: &[tokens::Model],
) -> Result<HashMap<Address, TokenData>, IndexerJobError> {
    let names = get_batch_contracts_and_methods(rpc, abi::TokenData::nameCall {}, tokens).await?;
    let symbols =
        get_batch_contracts_and_methods(rpc, abi::TokenData::symbolCall {}, tokens).await?;
    let decimals =
        get_batch_contracts_and_methods(rpc, abi::TokenData::decimalsCall {}, tokens).await?;
    let total_supply =
        get_batch_contracts_and_methods(rpc, abi::TokenData::totalSupplyCall {}, tokens).await?;

    let mut result = HashMap::new();
    for ((((token, name), symbol), decimal), total_supply) in tokens
        .iter()
        .zip(names)
        .zip(symbols)
        .zip(decimals)
        .zip(total_supply)
    {
        let token_data = TokenData {
            name: name.map(|n| n._0).ok(),
            symbol: symbol.map(|s| s._0).ok(),
            decimals: decimal.map(|d| d._0).ok(),
            total_supply: total_supply.map(|ts| ts._0.to_string()).ok(),
        };
        let address = Address::from_slice(&token.contract_address_hash);
        result.insert(address, token_data);
    }
    Ok(result)
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
