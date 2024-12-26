use crate::aliapis::sign::Api;
use crate::aliapis::sign::RequestHeader;
use ordermap::OrderMap;
use serde::Deserialize;
use serde::Serialize;

use super::ENDPOINT;
use super::VERSION;

#[derive(Debug,Clone)]
pub struct QueryDPUtilizationDetail(OrderMap<String, String>);

impl QueryDPUtilizationDetail {
    ///筛选查询的实例 ID，为空时返回所有的使用的实例明细。
    pub fn set_instance_id(mut self, instance_id: &str) -> Self {
        self.0
            .insert_sorted("InstanceId".into(), instance_id.into());
        self
    }
    ///实例对应的规格。
    pub fn set_instance_spec(mut self, instance_spec: &str) -> Self {
        self.0
            .insert_sorted("InstanceSpec".into(), instance_spec.into());
        self
    }
    ///商品 code，如 ecsRi、scu_bag 等。如果填写该字段，则 prodCode 字段不生效
    pub fn set_commodity_code(mut self, commodity_code: &str) -> Self {
        self.0
            .insert_sorted("CommodityCode".into(), commodity_code.into());
        self
    }
    ///被抵扣实例的 ID，为空时返回所有实例的明细数据。
    pub fn set_deducted_instance_id(mut self, deducted_instance_id: &str) -> Self {
        self.0
            .insert_sorted("DeductedInstanceId".into(), deducted_instance_id.into());
        self
    }
    ///开始时间，格式：YYYY-MM-dd HH:mm:ss。
    pub fn set_start_time(mut self, start_time: &str) -> Self {
        self.0.insert_sorted("StartTime".into(), start_time.into());
        self
    }

    ///结束时间，格式：YYYY-MM-dd HH:mm:ss。
    pub fn set_end_time(mut self, end_time: &str) -> Self {
        self.0.insert_sorted("EndTime".into(), end_time.into());
        self
    }
    ///查询从 LastToken 之后开始返回。第一次查询填 null，之后的从结果 NextToken 获取
    pub fn set_last_token(mut self, last_token: &str) -> Self {
        self.0.insert_sorted("LastToken".into(), last_token.into());
        self
    }
    ///每次查询条数，默认为 20，最大值为 300。
    pub fn set_limit(mut self, limit: &str) -> Self {
        self.0.insert_sorted("Limit".into(), limit.into());
        self
    }
    ///是否包含财务云子账号的资源包用量
    ///true：包含
    ///false：不包含
    pub fn set_include_share(mut self, include_share: &str) -> Self {
        self.0
            .insert_sorted("IncludeShare".into(), include_share.into());
        self
    }
    ///产品 code，如：ecs
    pub fn set_prod_code(mut self, prod_code: &str) -> Self {
        self.0.insert_sorted("ProdCode".into(), prod_code.into());
        self
    }
}

impl Api for QueryDPUtilizationDetail {
    fn new() -> Self {
        let parameters = OrderMap::new();

        QueryDPUtilizationDetail(parameters)
    }

    fn name(&self) -> String {
        "QueryDPUtilizationDetail".to_string()
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryDPUtilizationDetailResponse {
    pub message: String,
    pub request_id: String,
    pub data: Data,
    pub code: String,
    pub success: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub next_token: String,
    pub detail_list: DetailList,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetailList {
    pub detail_list: Vec<DetailList2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DetailList2 {
    pub res_code: String,
    pub uid: i64,
    pub deduct_date: String,
    pub instance_spec: String,
    pub share_uid: i64,
    pub deduct_measure: f64,
    pub instance_id: String,
    pub deduct_factor_total: f64,
    pub deducted_product_detail: String,
    pub deducted_commodity_code: String,
    pub deducted_instance_id: String,
    pub deduct_quantity: f64,
}

#[cfg(test)]
mod tests {
    use chrono::Datelike;

    use crate::aliapis::{
        billing::query_dp_utilization_detail::QueryDPUtilizationDetail, sign::Api,
    };

    #[tokio::test]
    async fn works() -> anyhow::Result<()> {
        let now = chrono::Local::now();
        let current_year = now.year();
        let current_month = now.month();
        let previous_month = now.month0();
        let start_time = format!("{}-{}-1 00:00:00", current_year, previous_month);
        let end_time = format!("{}-{}-0 00:00:00", current_year, current_month);
        let test_api = QueryDPUtilizationDetail::new()
            .set_include_share("true")
            .set_start_time(&start_time)
            .set_end_time(&end_time);
        let response = test_api.canonical_request().sign().send().await?;

        assert_eq!(response.status(), 200);
        Ok(())
    }
}
