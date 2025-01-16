use ordermap::OrderMap;
use serde::Deserialize;
use serde::Serialize;

use crate::Api;
use crate::RequestHeader;

pub struct SetDomainRecordStatus(OrderMap<String, String>);

impl SetDomainRecordStatus {
    pub fn set_lang(mut self, lang: &str) -> Self{
        self.0.insert_sorted("Lang".into(), lang.into());
        self
    }
    pub fn set_user_client_ip(mut self, user_client_ip: &str) -> Self{
        self.0.insert_sorted("UserClientIp".into(), user_client_ip.into());
        self
    }
    pub fn set_record_id(mut self, record_id: &str) -> Self{
        self.0.insert_sorted("RecordId".into(), record_id.into());
        self
    }
    pub fn set_status(mut self, status: &str) -> Self{
        self.0.insert_sorted("Status".into(), status.into());
        self
    }
}

impl Api for SetDomainRecordStatus {
    fn new() -> Self {
        let mut parameters = OrderMap::new();
        parameters.insert_sorted("PageSize".into(), 100.to_string());
        SetDomainRecordStatus(parameters)
    }

    fn name(&self) -> String {
        "SetDomainRecordStatus".to_string()
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
pub struct SetDomainRecordStatusResponse {
    pub status: String,
    pub request_id: String,
    pub record_id: String,
}

#[cfg(test)]
mod tests {
    use crate::{Api, SetDomainRecordStatus};



    #[tokio::test]
    async fn works() -> anyhow::Result<()> {
        let test_api = SetDomainRecordStatus::new().set_record_id("802585887865966592").set_status("Enable");
        let response = test_api.send().await?;
        assert_eq!(response.status(), 403);

        Ok(())
    }
}
