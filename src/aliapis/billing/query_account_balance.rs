use ordermap::OrderMap;
use serde::Deserialize;
use serde::Serialize;
use crate::aliapis::sign::Api;
use crate::aliapis::sign::RequestHeader;

use super::ENDPOINT;
use super::VERSION;

///查询用户账户余额信息
pub struct QueryAccountBalance;

impl Api for QueryAccountBalance {
    fn new() -> Self {
        QueryAccountBalance
    }

    fn name(&self) -> String {
        "QueryAccountBalance".to_string()
    }

    fn canonical_request(self) -> RequestHeader {
        let params: OrderMap<String, String> = OrderMap::new();

        RequestHeader::new(
            ENDPOINT.to_string(),
            self.name(),
            VERSION.to_string(),
            params,
        )
    }
}

// Response Struct
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryAccountBalanceResponse {
    pub message: String,
    pub request_id: String,
    pub data: Data,
    pub code: String,
    pub success: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub available_cash_amount: String,
    pub mybank_credit_amount: String,
    pub currency: String,
    pub available_amount: String,
    pub credit_amount: String,
    pub quota_limit: String,
}

#[cfg(test)]
mod tests {
    use crate::aliapis::{billing::query_account_balance::QueryAccountBalance, sign::Api};

    #[tokio::test]
    async fn works() -> anyhow::Result<()> {
        let test_api = QueryAccountBalance::new();
        let response = test_api.canonical_request().sign().send().await?;

        assert_eq!(response.status(), 200);
        Ok(())
    }
}
