use alloy::{network::TransactionBuilder, providers::Provider, rpc::{client::ClientBuilder, types::TransactionRequest}};
use super::{abi, RpcClient};



impl RpcClient {
    // pub async fn batch_contract_calls(&self, calls: Vec<ContractCall>) -> Result<Vec<ContractCallResponse>, Error> {
    //     let mut tasks = Vec::new();
    //     for call in calls {
    //         let task = self.provider.call(call);
    //         tasks.push(task);
    //     }
    //     let results = futures::future::join_all(tasks).await;
    //     Ok(results)
    // }

    pub async fn batch(&self) {
        let c = ClientBuilder::default().http("http://localhost:8545".parse().unwrap());
        let b = c.new_batch();
        
        
        let contract = abi::TokenData::TokenDataInstance::new("0xdAC17F958D2ee523a2206206994597C13D831ec7".parse().unwrap(), self.provider());
        let req = contract.name().calldata();


        b.add_call("eth_call", &serde_json::json!({
            "to": "0xdAC17F958D2ee523a2206206994597C13D831ec7",
            "data": req.to_string()
        }));

        
    }
}


