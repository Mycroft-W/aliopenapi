use ordermap::OrderMap;
use serde::Deserialize;
use serde::Serialize;

use crate::Api;
use crate::RequestHeader;

#[derive(Debug, Clone)]
pub struct DescribeDomainRecords(OrderMap<String, String>);

impl DescribeDomainRecords {
    pub fn set_lang(mut self, lang: &str) -> Self {
        self.0.insert_sorted("Lang".into(), lang.into());
        self
    }
    pub fn set_domain_name(mut self, domain_name: &str) -> Self {
        self.0
            .insert_sorted("DomainName".into(), domain_name.into());
        self
    }
    pub fn set_key_word(mut self, key_word: &str) -> Self {
        self.0.insert_sorted("KeyWord".into(), key_word.into());
        self
    }
    pub fn set_rr_key_word(mut self, rr_key_word: &str) -> Self {
        self.0.insert_sorted("RRKeyWord".into(), rr_key_word.into());
        self
    }
    pub fn set_type_key_word(mut self, type_key_word: &str) -> Self {
        self.0
            .insert_sorted("TypeKeyWord".into(), type_key_word.into());
        self
    }
    pub fn set_value_key_word(mut self, value_key_word: &str) -> Self {
        self.0
            .insert_sorted("ValueKeyWord".into(), value_key_word.into());
        self
    }
    pub fn set_order_by(mut self, order_by: &str) -> Self {
        self.0.insert_sorted("OrderBy".into(), order_by.into());
        self
    }
    pub fn set_direction(mut self, direction: &str) -> Self {
        self.0.insert_sorted("Direction".into(), direction.into());
        self
    }
    pub fn set_search_mode(mut self, search_mode: &str) -> Self {
        self.0
            .insert_sorted("SearchMode".into(), search_mode.into());
        self
    }
    pub fn set_group_id(mut self, group_id: &str) -> Self {
        self.0.insert_sorted("GroupId".into(), group_id.into());
        self
    }
    pub fn set_record_type(mut self, type_: &str) -> Self {
        self.0.insert_sorted("Type".into(), type_.into());
        self
    }
    pub fn set_line(mut self, line: &str) -> Self {
        self.0.insert_sorted("Line".into(), line.into());
        self
    }
    pub fn set_status(mut self, status: &str) -> Self {
        self.0.insert_sorted("Status".into(), status.into());
        self
    }
    pub fn set_page_number(mut self, page_number: i64) -> Self {
        self.0
            .insert_sorted("PageNumber".into(), page_number.to_string());
        self
    }
    pub fn set_page_size(mut self, page_size: i64) -> Self {
        self.0
            .insert_sorted("PageSize".into(), page_size.to_string());
        self
    }
}

impl Api for DescribeDomainRecords {
    fn new() -> Self {
        let mut parameters = OrderMap::new();
        parameters.insert_sorted("PageSize".into(), 500.to_string());
        DescribeDomainRecords(parameters)
    }

    fn name(&self) -> String {
        "DescribeDomainRecords".to_string()
    }

    fn send(self) -> impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>> {
        RequestHeader::new(
            super::ENDPOINT.to_string(),
            self.name(),
            super::VERSION.to_string(),
            self.0,
        )
        .set_method("GET")
        .sign()
        .send()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeDomainRecordsResponse {
    pub total_count: i64,
    pub page_size: i64,
    pub request_id: String,
    pub domain_records: DomainRecords,
    pub page_number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DomainRecords {
    pub record: Vec<Record>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record {
    pub status: String,
    #[serde(rename = "RR")]
    pub rr: String,
    pub line: String,
    pub locked: bool,
    #[serde(rename = "Type")]
    pub type_field: String,
    pub domain_name: String,
    pub value: String,
    pub record_id: String,
    #[serde(rename = "TTL")]
    pub ttl: i64,
    pub create_timestamp: i64,
    pub weight: i64,
    pub update_timestamp: Option<i64>,
}

#[cfg(test)]
mod tests {
    use crate::aliapis::{dns::describe_domain_records::DescribeDomainRecords, sign::Api};

    #[tokio::test]
    async fn works() -> anyhow::Result<()> {
        let test_api = DescribeDomainRecords::new().set_domain_name("123pan.cn");
        let response = test_api.send().await?;
        assert_eq!(response.status(), 200);

        Ok(())
    }
}
