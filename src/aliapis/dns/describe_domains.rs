use ordermap::OrderMap;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::aliapis::sign::Api;
use crate::aliapis::sign::RequestHeader;

#[derive(Debug, Clone)]
pub struct DescribeDomains(OrderMap<String, String>);

impl Api for DescribeDomains {
    fn new() -> Self {
        let mut parameters = OrderMap::new();
        parameters.insert_sorted("PageSize".into(), 100.to_string());
        DescribeDomains(parameters)
    }

    fn name(&self) -> String {
        "DescribeDomains".to_string()
    }

    fn send(self) -> impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>> {
        RequestHeader::new(
            super::ENDPOINT.to_string(),
            self.name(),
            super::VERSION.to_string(),
            self.0,
        )
        .set_method("POST")
        .sign()
        .send()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeDomainsResponse {
    pub domains: Domains,
    pub total_count: i64,
    pub page_size: i64,
    pub request_id: String,
    pub page_number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Domains {
    pub domain: Vec<Domain>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Domain {
    pub ali_domain: bool,
    pub resource_group_id: String,
    pub domain_name: String,
    pub create_time: String,
    pub puny_code: String,
    pub dns_servers: DnsServers,
    pub starmark: bool,
    pub domain_logging_switch_status: String,
    pub version_code: String,
    pub domain_id: String,
    pub version_name: String,
    pub record_count: i64,
    pub create_timestamp: i64,
    pub tags: Tags,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DnsServers {
    pub dns_server: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tags {
    pub tag: Vec<Value>,
}

#[cfg(test)]
mod tests {
    use crate::aliapis::{dns::describe_domains::DescribeDomains, sign::Api};

    #[tokio::test]
    async fn works() -> anyhow::Result<()> {
        let test_api = DescribeDomains::new();
        let response = test_api.send().await?;
        assert_eq!(response.status(), 403);

        Ok(())
    }
}
