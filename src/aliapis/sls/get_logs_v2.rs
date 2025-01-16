use crate::Api;
use crate::RequestHeader;
use ordermap::OrderMap;
use reqwest::Error;
use reqwest::Response;
use serde::Deserialize;
use serde::Serialize;
use std::future::Future;

#[derive(Debug)]
pub struct GetLogsV2 {
    uri: String,
    query_params: OrderMap<String, String>,
    body: serde_json::Map<String, serde_json::Value>,
}

impl GetLogsV2 {
    ///project 名称。
    pub fn set_project(mut self, project: impl Into<String>) -> Self {
        self.query_params
            .insert_sorted("project".into(), project.into());
        self
    }
    ///logstore 名称。
    pub fn set_logstore(mut self, logstore: impl Into<String>) -> Self {
        let uri = format!("/logstores/{}/logs", logstore.into());
        self.uri = uri;
        self
    }
    ///开始时间，timestamp，精确到秒。
    pub fn set_from(mut self, from: i64) -> Self {
        self.body.insert(
            "from".to_owned(),
            serde_json::Value::String(from.to_string()),
        );
        self
    }
    ///结束时间，timestamp，精确到秒。
    pub fn set_to(mut self, to: i64) -> Self {
        self.body
            .insert("to".to_owned(), serde_json::Value::String(to.to_string()));
        self
    }
    ///仅当 query 参数为查询语句时，该参数有效，表示请求返回的最大日志条数。最小值为 0，最大值为 100，默认值为 100。
    pub fn set_line(mut self, line: i64) -> Self {
        self.body.insert(
            "line".to_owned(),
            serde_json::Value::String(line.to_string()),
        );
        self
    }
    ///仅当 query 参数为查询语句时，该参数有效，表示查询开始行。默认值为 0。
    pub fn set_offset(mut self, offset: i64) -> Self {
        self.body.insert(
            "offset".to_owned(),
            serde_json::Value::String(offset.to_string()),
        );
        self
    }
    ///用于指定返回结果是否按日志时间戳降序返回日志，精确到分钟级别。
    pub fn set_reverse(mut self, reverse: bool) -> Self {
        self.body.insert(
            "reverse".to_owned(),
            serde_json::Value::String(reverse.to_string()),
        );
        self
    }
    ///是否开启增强 sql，默认关闭。
    pub fn set_power_sql(mut self, power_sql: bool) -> Self {
        self.body.insert(
            "powerSql".to_owned(),
            serde_json::Value::String(power_sql.to_string()),
        );
        self
    }
    ///查询参数.
    pub fn set_session(mut self, session: impl Into<String>) -> Self {
        self.body.insert(
            "session".to_owned(),
            serde_json::Value::String(session.into()),
        );
        self
    }
    ///日志主题。默认值为双引号（""）。
    pub fn set_topic(mut self, topic: impl Into<String>) -> Self {
        self.body
            .insert("topic".to_owned(), serde_json::Value::String(topic.into()));
        self
    }
    ///查询语句或者分析语句。
    pub fn set_query(mut self, query: impl Into<String>) -> Self {
        self.body
            .insert("query".to_owned(), serde_json::Value::String(query.into()));
        self
    }
    ///scan 或短语查询表示是否向前或向后翻页
    pub fn set_forward(mut self, forward: bool) -> Self {
        self.body.insert(
            "forward".to_owned(),
            serde_json::Value::String(forward.to_string()),
        );
        self
    }
    ///是否高亮
    pub fn set_highlight(mut self, highlight: bool) -> Self {
        self.body.insert(
            "highlight".to_owned(),
            serde_json::Value::String(highlight.to_string()),
        );
        self
    }
}

impl Api for GetLogsV2 {
    fn new() -> Self {
        let query_params = OrderMap::new();
        let body = serde_json::Map::new();
        Self {
            uri: "/logstores/{logstore}/logs".to_string(),
            query_params,
            body,
        }
    }

    fn name(&self) -> String {
        "GetLogsV2".to_string()
    }

    fn send(self) -> impl Future<Output = Result<Response, Error>> {
        RequestHeader::new(
            super::ENDPOINT.to_string(),
            self.name(),
            super::VERSION.to_string(),
            self.query_params,
        )
        .set_method("POST")
        .set_accept_encoding("gzip")
        .set_uri(&self.uri)
        .set_body(serde_json::Value::Object(self.body))
        .sign()
        .send()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetLogsV2Response {
    #[serde(rename = "meta")]
    pub meta: Meta,
    #[serde(rename = "data")]
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Meta {
    #[serde(rename = "count")]
    pub count: i64,
    #[serde(rename = "progress")]
    pub progress: String,
    #[serde(rename = "processedRows")]
    pub processed_rows: i64,
    #[serde(rename = "processedBytes")]
    pub processed_bytes: i64,
    #[serde(rename = "elapsedMillisecond")]
    pub elapsed_millisecond: i64,
    #[serde(rename = "hasSQL")]
    pub has_sql: bool,
    #[serde(rename = "telementryType")]
    pub telementry_type: String,
    #[serde(rename = "telemetryType")]
    pub telemetry_type: String,
    #[serde(rename = "whereQuery")]
    pub where_query: String,
    #[serde(rename = "aggQuery")]
    pub agg_query: String,
    #[serde(rename = "keys")]
    pub keys: Vec<String>,
    #[serde(rename = "terms")]
    pub terms: Vec<Term>,
    #[serde(rename = "isAccurate")]
    pub is_accurate: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Term {
    #[serde(rename = "term")]
    pub term: String,
    #[serde(rename = "key")]
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Daum {
    #[serde(rename = "access_time")]
    pub access_time: String,
    #[serde(rename = "app_version")]
    pub app_version: String,
    #[serde(rename = "body_bytes_sent")]
    pub body_bytes_sent: String,
    #[serde(rename = "cdn_user_real_ip")]
    pub cdn_user_real_ip: String,
    #[serde(rename = "content-type")]
    pub content_type: String,
    #[serde(rename = "http_referrer")]
    pub http_referrer: String,
    #[serde(rename = "http_user_agent")]
    pub http_user_agent: String,
    #[serde(rename = "http_x_forwarded_for")]
    pub http_x_forwarded_for: String,
    #[serde(rename = "jwt_uid")]
    pub jwt_uid: String,
    #[serde(rename = "jwt_username")]
    pub jwt_username: String,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "project_tag")]
    pub project_tag: String,
    #[serde(rename = "remote_addr")]
    pub remote_addr: String,
    #[serde(rename = "request_body")]
    pub request_body: String,
    #[serde(rename = "request_length")]
    pub request_length: String,
    #[serde(rename = "request_method")]
    pub request_method: String,
    #[serde(rename = "request_time")]
    pub request_time: String,
    #[serde(rename = "request_uri")]
    pub request_uri: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "trace_id")]
    pub trace_id: String,
    #[serde(rename = "upstream_addr")]
    pub upstream_addr: String,
    #[serde(rename = "upstream_response_time")]
    pub upstream_response_time: String,
    #[serde(rename = "upstream_status")]
    pub upstream_status: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "__topic__")]
    pub topic: String,
    #[serde(rename = "__source__")]
    pub source: String,
    #[serde(rename = "__tag__:__hostname__")]
    pub tag_hostname: String,
    #[serde(rename = "__tag__:__path__")]
    pub tag_path: String,
    #[serde(rename = "__tag__:__user_defined_id__")]
    pub tag_user_defined_id: String,
    #[serde(rename = "__tag__:__pack_id__")]
    pub tag_pack_id: String,
    #[serde(rename = "__tag__:__client_ip__")]
    pub tag_client_ip: String,
    #[serde(rename = "__tag__:__receive_time__")]
    pub tag_receive_time: String,
    #[serde(rename = "__time__")]
    pub time: String,
}

#[cfg(test)]
mod tests {
    use crate::{aliapis::sls::get_logs_v2::GetLogsV2, Api};

    #[tokio::test]
    async fn works() -> anyhow::Result<()> {
        let date = chrono::Local::now().timestamp();
        let test_api = GetLogsV2::new()
            .set_logstore("123pan-gateway")
            .set_from(date - 3600)
            .set_to(date);
        println!("{:#?}", test_api);
        let response = test_api.send().await?;
        println!("{:#?}", response.text().await?);
        //assert_eq!(response.status(), 200);

        Ok(()) 
    }
}
