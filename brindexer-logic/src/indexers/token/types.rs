use sea_orm::prelude::BigDecimal;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenData {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: Option<BigDecimal>,
    pub total_supply: Option<BigDecimal>,
    #[allow(dead_code)]
    pub contract_uri: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContractURIData {
    pub name: String,
    pub symbol: Option<String>,
}

impl TokenData {
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.symbol.is_none()
            && self.decimals.is_none()
            && self.total_supply.is_none()
    }

    pub fn is_complete(&self) -> bool {
        self.name.is_some()
            && self.symbol.is_some()
            && self.decimals.is_some()
            && self.total_supply.is_some()
    }

    pub async fn populate_with_contract_uri(mut self) -> TokenData {
        if self.is_complete() {
            return self;
        }

        let contract_uri_data = match self.process_contract_uri().await {
            Some(data) => data,
            None => return self,
        };

        if self.name.is_none() {
            self.name = Some(contract_uri_data.name.clone());
        }
        if self.symbol.is_none() {
            self.symbol = contract_uri_data.symbol.clone();
        }

        self
    }

    async fn process_contract_uri(&self) -> Option<ContractURIData> {
        let contract_uri = self.contract_uri.as_deref()?;

        if let Ok(contract_uri) = Url::parse(contract_uri) {
            reqwest::get(contract_uri).await.ok()?.json().await.ok()
        } else {
            serde_json::from_str(contract_uri).ok()
        }
    }
}
