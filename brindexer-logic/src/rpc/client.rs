use alloy::{
    providers::ProviderBuilder,
    transports::http::{Client, Http},
};
use url::Url;

pub type HttpProvider = alloy::providers::RootProvider<Http<Client>>;

pub struct RpcClient {
    provider: Box<HttpProvider>,
}

impl RpcClient {
    pub fn new(url: Url) -> Self {
        let provider: Box<alloy::providers::RootProvider<Http<Client>>> =
            Box::new(ProviderBuilder::new().on_http(url));

        Self { provider }
    }

    pub fn provider(&self) -> &HttpProvider {
        &self.provider
    }
}
