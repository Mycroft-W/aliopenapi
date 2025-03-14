use ordermap::OrderMap;
use serde::Deserialize;
use serde::Serialize;

use crate::Api;
use crate::RequestHeader;

pub struct AddDomainRecord(OrderMap<String, String>);

impl AddDomainRecord {
    pub fn set_lang(mut self, lang: &str) -> Self{
        self.0.insert_sorted("Lang".into(), lang.into());
        self
    }
    pub fn set_user_client_ip(mut self, user_client_ip: &str) -> Self{
        self.0.insert_sorted("UserClientIp".into(), user_client_ip.into());
        self
    }
    pub fn set_domain_name(mut self, domain_name: &str) -> Self{
        self.0.insert_sorted("DomainName".into(), domain_name.into());
        self
    }
    pub fn set_rr(mut self, rr: &str) -> Self{
        self.0.insert_sorted("RR".into(), rr.into());
        self
    }
    pub fn set_type(mut self, type_: &str) -> Self{
        self.0.insert_sorted("Type".into(), type_.to_uppercase());
        self
    }
    pub fn set_value(mut self, value: &str) -> Self{
        self.0.insert_sorted("Value".into(), value.into());
        self
    }
    pub fn set_ttl(mut self, ttl: &str) -> Self{
        self.0.insert_sorted("TTL".into(), ttl.into());
        self
    }
    pub fn set_priority(mut self, priority: &str) -> Self{
        self.0.insert_sorted("Priority".into(), priority.into());
        self
    }
    pub fn set_line(mut self, line: &str) -> Self{
        self.0.insert_sorted("Line".into(), line.into());
        self
    }
}

impl Api for AddDomainRecord {
    fn new() -> Self {
        let mut parameters = OrderMap::new();
        parameters.insert_sorted("PageSize".into(), 100.to_string());
        AddDomainRecord(parameters)
    }

    fn name(&self) -> String {
        "AddDomainRecord".to_string()
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
pub struct AddDomainRecordResponse {
    pub status: String,
    pub request_id: String,
    pub record_id: String,
}

#[cfg(test)]
mod tests {
    use crate::{Api, AddDomainRecord};



    #[tokio::test]
    async fn works() -> anyhow::Result<()> {
        dotenv::dotenv().ok();
        let test_api = AddDomainRecord::new().set_domain_name("123clouddrive.cn").set_rr("test").set_type("cname").set_value("www.123pan.com");
        let response = test_api.send().await?;

        //println!("{:?}", response.text().await?);

        assert_eq!(response.status(), 200);

        Ok(())
    }
}
