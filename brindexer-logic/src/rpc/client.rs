use alloy::{
    providers::ProviderBuilder,
    transports::http::{Client, Http},
};
use derive_new::new;
use url::Url;

pub type HttpProvider = alloy::providers::RootProvider<Http<Client>>;

#[derive(Debug, Clone, new)]
pub struct RpcClient {
    provider: Box<HttpProvider>,
}

impl RpcClient {
    pub fn from_url(url: Url) -> Self {
        let provider: Box<alloy::providers::RootProvider<Http<Client>>> =
            Box::new(ProviderBuilder::new().on_http(url));

        Self::new(provider)
    }

    pub fn provider(&self) -> &HttpProvider {
        &self.provider
    }
}
