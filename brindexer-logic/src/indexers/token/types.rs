use sea_orm::prelude::BigDecimal;

#[derive(Debug, Clone)]
pub struct TokenData {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: Option<BigDecimal>,
    pub total_supply: Option<BigDecimal>,
    #[allow(dead_code)]
    pub contract_uri: Option<String>,
    pub skip_metadata: bool,
}

impl TokenData {
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.symbol.is_none()
            && self.decimals.is_none()
            && self.total_supply.is_none()
    }
}
