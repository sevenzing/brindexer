use super::{Error, RpcClient};
use crate::rpc::constants;
use alloy::{providers::Provider, rpc::client::BatchRequest, sol_types::SolCall};
use alloy_primitives::Address;
use alloy_rpc_client::Waiter;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
struct EthCallParam {
    to: Address,
    data: blockscout_display_bytes::Bytes,
}

impl RpcClient {
    pub async fn batch<C>(
        &self,
        contracts_and_methods: Vec<(Address, C)>,
    ) -> Result<Vec<Result<C::Return, Error>>, Error>
    where
        C: SolCall,
    {
        let mut batch = BatchRequest::new(self.provider().client());

        let futures = contracts_and_methods
            .into_iter()
            .map(|(addr, m)| {
                let encoded = m.abi_encode();
                let param = EthCallParam {
                    to: addr.to_owned(),
                    data: encoded.into(),
                };
                let waiter: Waiter<String> =
                    batch.add_call(constants::ETH_CALL, &serde_json::json!([param]))?;

                Ok(waiter)
            })
            .collect::<Result<Vec<_>, Error>>()?;

        batch.send().await?;

        let results = join_all(futures)
            .await
            .into_iter()
            .map(|r| {
                r.map_err(Error::Rpc)
                    .and_then(|s| {
                        blockscout_display_bytes::Bytes::from_str(&s).map_err(|e| {
                            Error::Internal(anyhow::anyhow!("hex decode error: {}", e))
                        })
                    })
                    .and_then(|s| {
                        C::abi_decode_returns(&s, true).map_err(|e| {
                            Error::Internal(anyhow::anyhow!("abi decode error: {}", e))
                        })
                    })
            })
            .collect::<Vec<_>>();

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rpc::abi;
    use pretty_assertions::assert_eq;
    use rstest::*;
    const DEFAULT_RPC: &str = "https://eth-sepolia.public.blastapi.io";

    #[rstest]
    #[tokio::test]
    #[case(
        &[
            "0x54FA517F05e11Ffa87f4b22AE87d91Cec0C2D7E1",
            "0xdcF5D3E08c5007deCECDb34808C49331bD82a247",
            "0x489c5CB7fD158B0A9E7975076D758268a756C025",
        ],
        &[
            "RareTron.io",
            "VanityTrx.org",
            "VanityTron.io",
        ]
    )]
    async fn test_batch(#[case] addresses: &[&str], #[case] expected: &[&str]) {
        let client = RpcClient::new(DEFAULT_RPC.parse().unwrap());
        let results = client
            .batch(
                addresses
                    .iter()
                    .map(|a| (a.parse().unwrap(), abi::TokenData::nameCall {}))
                    .collect(),
            )
            .await
            .unwrap()
            .into_iter()
            .map(|r| r.unwrap()._0)
            .collect::<Vec<_>>();
        assert_eq!(expected, results);
    }
}
