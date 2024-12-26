use super::{ENDPOINT, VERSION};
use ordermap::OrderMap;
use serde::{Deserialize, Serialize};

use crate::aliapis::sign::{Api, RequestHeader};

#[derive(Debug,Clone)]
pub struct ListSyntheticDetail(OrderMap<String, String>);
impl ListSyntheticDetail {
    ///地域 ID，当前只支持 cn-hangzhou。
    pub fn set_region_id(mut self, region_id: &str) -> Self {
        self.0
            .insert_sorted("RegionID".into(), region_id.to_string());
        self
    }
    ///拨测类型，必填，即时拨测：1，定时拨测：2。
    pub fn set_synthetic_type(mut self, synthetic_type: &str) -> Self {
        self.0
            .insert_sorted("SyntheticType".into(), synthetic_type.to_string());
        self
    }
    ///结果分类，必填，写死：SYNTHETIC。
    pub fn set_category(mut self, category: &str) -> Self {
        self.0
            .insert_sorted("Category".into(), category.to_string());
        self
    }
    ///拨测结果内容，必填，获取不同类型的拨测结果传不同的值：
    ///PING 列表：ICMP_LIST
    ///TCP 列表：TCP_LIST
    ///DNS 列表：DNS_LIST
    ///HTTP(s)列表：HTTP_LIST
    ///网站测速列表：WEBSITE_LIST
    ///文件下载列表：DOWNLOAD_LIST
    ///拨测结果明细：ALL
    pub fn set_detail(mut self, detail: &str) -> Self {
        self.0.insert_sorted("Detail".into(), detail.to_string());
        self
    }
    ///筛选条件，必填。
    ///查任务拨测结果列表：{"taskId":"${taskId}"}
    ///查任务拨测结果详情：{"taskId":"${taskId}","dataId":"${dataId}"}
    pub fn set_filters(mut self, task_id: &str) -> Self {
        //let start = r#"{{\"taskId\": \""#;
        //let end = r#""}"#;
        let filter = format!("{{\"taskId\": \"{}\"}}", task_id);
        self.0.insert_sorted("Filters".into(), filter);
        self
    }

    ///筛选条件数组，必填。
    ///查询拨测结果列表：[{"Key":"taskType","OpType":"in","Value":[任务类型]}]。
    ///查询拨测结果详情：[{"Key":"dataId","OpType":"eq","Value":"dataId"}] ，dataId 为查任务拨测结果列表返回值的 dataId 字段。
    pub fn set_advanced_filters(mut self, key: &str, op: &str, value: &str) -> Self {
        let advanced_filters = format!(
            "[{{\"Key\":\"{}\",\"OpType\":\"{}\",\"Value\":\"{}\"}}]",
            key, op, value
        );
        self.0
            .insert_sorted("".into(), advanced_filters.to_string());
        self
    }
    ///查询起始时间的时间戳，必填，精确到毫秒。
    pub fn set_start_time(mut self, start_time: &str) -> Self {
        self.0
            .insert_sorted("StartTime".into(), start_time.to_string());
        self
    }
    ///查询结束时间的时间戳，必填，精确到毫秒。
    pub fn set_end_time(mut self, end_time: &str) -> Self {
        self.0.insert_sorted("EndTime".into(), end_time.to_string());
        self
    }
    ///排序字段，仅支持：timestamp（拨测发起时间）。
    pub fn set_order_by(mut self, order_by: &str) -> Self {
        self.0.insert_sorted("OrderBy".into(), order_by.to_string());
        self
    }
    ///排序标准。取值：
    ///ASC：升序。
    ///DESC：降序。
    pub fn set_order(mut self, order: &str) -> Self {
        self.0.insert_sorted("Order".into(), order.to_string());
        self
    }
    ///查询页码，从 1 开始。
    pub fn set_page(mut self, page: &str) -> Self {
        self.0.insert_sorted("Page".into(), page.to_string());
        self
    }
    ///分页大小。
    pub fn set_page_size(mut self, page_size: &str) -> Self {
        self.0
            .insert_sorted("PageSize".into(), page_size.to_string());
        self
    }
}

impl Api for ListSyntheticDetail {
    // 使用默认参数实例化
    fn new() -> Self {
        let mut parameters = OrderMap::new();
        parameters.insert_sorted("PageSize".to_string(), "300".to_string());
        parameters.insert_sorted("RegionId".to_string(), "cn-hangzhou".to_string());
        parameters.insert_sorted("Category".to_string(), "SYNTHETIC".to_string());

        ListSyntheticDetail(parameters)
    }

    fn name(&self) -> String {
        "ListSyntheticDetail".into()
    }

    fn canonical_request(self) -> RequestHeader {
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
pub struct ListSyntheticDetailResponse {
    pub request_id: String,
    pub data: Data,
    pub code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub page_size: i64,
    pub total: i64,
    pub page: i64,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    #[serde(rename = "__time__")]
    pub time: i64,
    #[serde(rename = "fileDownload")]
    pub file_download: String,
    #[serde(rename = "ipIsp")]
    pub ip_isp: String,
    #[serde(rename = "downloadSpeed")]
    pub download_speed: String,
    #[serde(rename = "targetCity")]
    pub target_city: String,
    #[serde(rename = "ipCity")]
    pub ip_city: String,
    #[serde(rename = "responseTime")]
    pub response_time: String,
    #[serde(rename = "__source__")]
    pub source: String,
    #[serde(rename = "resultCode")]
    pub result_code: String,
    #[serde(rename = "ipCountry")]
    pub ip_country: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "responseCode")]
    pub response_code: String,
    #[serde(rename = "targetIp")]
    pub target_ip: String,
    #[serde(rename = "targetOperator")]
    pub target_operator: String,
    #[serde(rename = "clientType")]
    pub client_type: String,
    #[serde(rename = "dataId")]
    pub data_id: String,
    #[serde(rename = "fileSize")]
    pub file_size: String,
    #[serde(rename = "clientLastIp")]
    pub client_last_ip: String,
    #[serde(rename = "errName")]
    pub err_name: String,
    #[serde(rename = "ipRegion")]
    pub ip_region: String,
    #[serde(rename = "taskName")]
    pub task_name: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

#[cfg(test)]
mod tests {

    use chrono::*;

    use crate::aliapis::{arms::list_synthetic_detail::ListSyntheticDetail, sign::Api};

    #[tokio::test]
    async fn works() -> anyhow::Result<()> {
        let test_api = ListSyntheticDetail::new()
            .set_end_time(&Local::now().timestamp_millis().to_string())
            .set_start_time(&(Local::now().timestamp_millis() - 24 * 60 * 60 * 1000).to_string())
            .set_synthetic_type("2")
            .set_detail("DOWNLOAD_LIST")
            .set_page("1")
            .set_filters("f124d885bcbc4d78a7fcb3a020b6ad66");
        let response = test_api.canonical_request().sign().send().await?;

        assert_eq!(response.status(), 200);
        Ok(())
    }
}
