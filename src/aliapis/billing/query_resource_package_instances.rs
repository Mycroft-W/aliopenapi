use crate::aliapis::sign::Api;
use crate::aliapis::sign::RequestHeader;
use ordermap::OrderMap;
use serde::Deserialize;
use serde::Serialize;

use super::ENDPOINT;
use super::VERSION;

pub struct QueryResourcePackageInstances(OrderMap<String, String>);

impl QueryResourcePackageInstances {
    ///产品代码。
    pub fn set_product_code(mut self, product_code: &str) -> Self {
        self.0
            .insert_sorted("ProductCode".into(), product_code.into());
        self
    }
    ///失效起始时间。日期格式按照 ISO8601 标准表示，并需要使用 UTC 时间。格式为：yyyy-MM-ddTHH:mm:ssZ。
    pub fn set_expiry_time_start(mut self, expiry_time_start: &str) -> Self {
        self.0
            .insert_sorted("ExpiryTimeStart".into(), expiry_time_start.into());
        self
    }
    ///失效结束时间。日期格式按照 ISO8601 标准表示，并需要使用 UTC 时间。格式为：yyyy-MM-ddTHH:mm:ssZ。
    pub fn set_expiry_time_end(mut self, expiry_time_end: &str) -> Self {
        self.0
            .insert_sorted("ExpiryTimeEnd".into(), expiry_time_end.into());
        self
    }
    ///是否包含合作伙伴。
    pub fn set_include_partner(mut self, include_partner: &str) -> Self {
        self.0
            .insert_sorted("IncludePartner".into(), include_partner.into());
        self
    }
    ///页码，默认为 1。
    pub fn set_page_num(mut self, page_num: &str) -> Self {
        self.0.insert_sorted("PageNum".into(), page_num.into());
        self
    }
    ///每页条数，默认值 20，最大 300。
    pub fn set_page_size(mut self, page_size: &str) -> Self {
        self.0.insert_sorted("PageSize".into(), page_size.into());
        self
    }
}

impl Api for QueryResourcePackageInstances {
    fn new() -> Self {
        let mut parameters = OrderMap::new();
        parameters.insert_sorted("PageSize".to_string(), "300".to_string());

        QueryResourcePackageInstances(parameters)
    }

    fn name(&self) -> String {
        "QueryResourcePackageInstances".to_string()
    }

    fn canonical_request(self) -> crate::aliapis::sign::RequestHeader {
        RequestHeader::new(
            ENDPOINT.to_string(),
            self.name(),
            VERSION.to_string(),
            self.0,
        )
    }
}

// Response Struct
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryResourcePackageInstancesResponse {
    pub request_id: String,
    pub message: String,
    pub data: Data,
    pub code: String,
    pub success: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub instances: Instances,
    pub total_count: i64,
    pub page_num: i64,
    pub page_size: i64,
    pub host_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Instances {
    pub instance: Vec<Instance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Instance {
    pub status: String,
    pub instance_id: String,
    pub effective_time: String,
    pub expiry_time: String,
    pub total_amount: String,
    pub remark: String,
    pub remaining_amount_unit: String,
    pub package_type: String,
    pub deduct_type: String,
    pub total_amount_unit: String,
    pub commodity_code: String,
    pub region: String,
    pub applicable_products: ApplicableProducts,
    pub remaining_amount: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApplicableProducts {
    pub product: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::aliapis::{
        billing::query_resource_package_instances::QueryResourcePackageInstances, sign::Api,
    };

    #[tokio::test]
    async fn works() -> anyhow::Result<()> {
        let test_api = QueryResourcePackageInstances::new();
        let response = test_api.canonical_request().sign().send().await?;

        assert_eq!(response.status(), 200);
        Ok(())
    }
}
