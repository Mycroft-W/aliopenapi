use std::fmt::Debug;

use ordermap::OrderMap;
use serde::Deserialize;
use serde::Serialize;

use crate::aliapis::sign::Api;
use crate::aliapis::sign::RequestHeader;

///查询短信发送统计详情，包括短信发送时间、短信发送成功条数、接收回执条数等
#[derive(Debug, Clone)]
pub struct QuerySendStatistics(OrderMap<String, String>);

impl QuerySendStatistics {
    ///短信发送范围。取值：
    ///1：国内短信发送记录。
    ///2：国际/港澳台短信发送记录。
    pub fn set_is_globe(mut self, is_globe: &str) -> Self {
        self.0.insert_sorted("IsGlobe".into(), is_globe.into());
        self
    }
    ///开始日期，格式为 yyyyMMdd，例如 20181225。
    pub fn set_start_date(mut self, start_date: &str) -> Self {
        self.0.insert_sorted("StartDate".into(), start_date.into());
        self
    }
    ///结束日期，格式为 yyyyMMdd，例如 20181225。
    pub fn set_end_date(mut self, end_date: &str) -> Self {
        self.0.insert_sorted("EndDate".into(), end_date.into());
        self
    }
    ///模板类型。取值：
    ///0：验证码。
    ///1：通知短信。
    ///2：推广短信。（仅支持企业客户）
    ///3：国际/港澳台消息。（仅支持企业客户）
    ///7：数字短信。
    pub fn set_template_type(mut self, template_type: &str) -> Self {
        self.0
            .insert_sorted("TemplateType".into(), template_type.into());
        self
    }
    ///签名名称。
    pub fn set_sign_name(mut self, sign_name: &str) -> Self {
        self.0.insert_sorted("SignName".into(), sign_name.into());
        self
    }
    ///当前页码。默认取值为 1。
    pub fn set_page_index(mut self, page_index: &str) -> Self {
        self.0.insert_sorted("PageIndex".into(), page_index.into());
        self
    }
    ///每页显示的条数。取值范围：1~50。
    pub fn set_page_size(mut self, page_size: &str) -> Self {
        self.0.insert_sorted("PageSize".into(), page_size.into());
        self
    }
}

impl Api for QuerySendStatistics {
    fn new() -> Self {
        let mut parameters = OrderMap::new();
        parameters.insert_sorted("PageIndex".to_string(), 1.to_string());
        parameters.insert_sorted("PageSize".to_string(), 50.to_string());

        QuerySendStatistics(parameters)
    }

    fn name(&self) -> String {
        "QuerySendStatistics".into()
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
pub struct QuerySendStatisticsResponse {
    pub request_id: String,
    pub data: Data,
    pub code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub target_list: Vec<TargetList>,
    pub total_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TargetList {
    pub total_count: i64,
    pub no_responded_count: i64,
    pub send_date: String,
    pub responded_fail_count: i64,
    pub responded_success_count: i64,
}

#[cfg(test)]
mod tests {
    use crate::aliapis::{sign::Api, sms::query_send_statistics::QuerySendStatistics};

    #[tokio::test]
    async fn works() -> anyhow::Result<()> {
        let date = chrono::Local::now().format("%Y%m%d").to_string();
        let test_api = QuerySendStatistics::new()
            .set_start_date(&date)
            .set_end_date(&date)
            .set_is_globe("1");
        let response = test_api.send().await?;
        assert_eq!(response.status(), 200);

        Ok(())
    }
}
