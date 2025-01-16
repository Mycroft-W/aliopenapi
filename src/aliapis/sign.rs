use chrono::Utc;
use data_encoding::HEXLOWER;
use ordermap::OrderMap;
use reqwest::{header::HeaderMap, Error, Method, Response};
use ring::{
    digest::{self},
    hmac,
};
use std::{env, future::Future, slice::from_raw_parts, str::from_utf8_unchecked};
use tracing::debug;
use uuid::Uuid;

#[derive(Debug)]
pub struct RequestHeader {
    pub http_method: String,
    pub canonical_uri: String,
    pub host: String,
    pub headers: OrderMap<String, String>,
    pub query_param: OrderMap<String, String>,
    pub body: String,
}

impl Default for RequestHeader {
    fn default() -> Self {
        Self {
            http_method: "GET".to_string().to_uppercase(),
            canonical_uri: "/".to_string(),
            host: Default::default(),
            headers: Default::default(),
            query_param: Default::default(),
            body: Default::default(),
        }
    }
}

impl RequestHeader {
    pub fn new(
        host: String,
        x_acs_action: String,
        x_acs_version: String,
        query_param: OrderMap<String, String>,
    ) -> RequestHeader {
        // 使用有顺序的 OrderMap 存储 quey_args
        let mut headers = OrderMap::new();
        headers.insert_sorted("host".to_owned(), host.clone());
        headers.insert_sorted("x-acs-action".to_owned(), x_acs_action.clone());
        headers.insert_sorted("x-acs-version".to_owned(), x_acs_version.clone());
        // use UTC date
        headers.insert_sorted(
            "x-acs-date".to_owned(),
            Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        );
        // use uuid as random string
        headers.insert_sorted(
            "x-acs-signature-nonce".to_owned(),
            Uuid::new_v4().to_string(),
        );
        headers.insert_sorted("Authorization".to_string(), "".to_string());

        RequestHeader {
            host,
            headers,
            query_param,
            ..Default::default()
        }
    }
    ///设置请求方法
    pub fn set_method(mut self, method: &str) -> Self {
        self.http_method = method.to_uppercase();
        self
    }
    ///设置请求路径
    pub fn set_uri(mut self, uri: &str) -> Self {
        self.canonical_uri = uri.to_string();
        self
    }
    ///设置body
    pub fn set_body(mut self, body: serde_json::Value) -> Self {
        self.body = body.to_string();
        self
    }
    /// 设置 accept_encoding
    pub fn set_accept_encoding(mut self, accept_encoding: impl Into<String>) -> Self {
        self.headers
            .insert_sorted("Accept-Encoding".to_owned(), accept_encoding.into());
        self
    }
    ///使用AK/SK 签名请求
    pub fn sign(mut self) -> Self {
        // get AK and SK from env
        let access_key_secret = env::var("ALI_CLOUD_ACCESSKEY_SECRET").expect("can not get SK");
        let access_key_id = env::var("ALI_CLOUD_ACCESSKEY_ID").expect("can not get AK");

        let algorithm = "ACS3-HMAC-SHA256";

        // format the params
        let mut params_list = Vec::new();

        for (k, v) in self.query_param.clone() {
            params_list.push(format!(
                "{}={}",
                urlencoding::encode(&k),
                urlencoding::encode(&v)
            ));
        }
        let canonical_query_string = &params_list
            .join("&")
            .replace("+", "20%")
            .replace("%5F", "_")
            .replace("%2D", "-")
            .replace("%2E", ".")
            .replace("%7E", "~");

        let request_body = self.body.to_string();

        // get the hex encoding from the sha256 digest of payload
        let hashed_request_payload = HEXLOWER
            .encode(ring::digest::digest(&digest::SHA256, request_body.as_bytes()).as_ref());
        self.headers.insert_sorted(
            "x-acs-content-sha256".to_string(),
            hashed_request_payload.clone(),
        );

        // construct the headers
        let mut filtered_headers = Vec::new();
        let mut sign_headers = Vec::new();

        for (k, v) in self.headers.clone() {
            if k.to_lowercase().starts_with("x-acs-")
                || k.to_lowercase().contains("host")
                || k.to_lowercase().contains("content-type")
            {
                filtered_headers.push(format!("{}:{}", k, v));
                sign_headers.push(k);
            }
        }

        let signed_headers = sign_headers.join(";");

        let canonical_headers = filtered_headers.join("\n");

        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n\n{}\n{}",
            self.http_method,
            self.canonical_uri,
            canonical_query_string,
            canonical_headers,
            signed_headers,
            hashed_request_payload
        )
        .to_string();

        #[cfg(debug_assertions)]
        debug!("{:?}", canonical_request);

        // first sha256 digest, then hex encoding
        let hashed_canoical_request = HEXLOWER
            .encode(ring::digest::digest(&digest::SHA256, canonical_request.as_bytes()).as_ref());
        let string_to_sign = format!("{}\n{}", algorithm, hashed_canoical_request);
        #[cfg(debug_assertions)]
        debug!("{}", string_to_sign);

        // sign the string
        let key = hmac::Key::new(hmac::HMAC_SHA256, access_key_secret.as_bytes());
        let signature = HEXLOWER.encode(hmac::sign(&key, string_to_sign.as_bytes()).as_ref());

        let authorization = format!(
            "{} Credential={},SignedHeaders={},Signature={}",
            algorithm, access_key_id, signed_headers, signature
        );

        self.headers
            .entry("Authorization".to_string())
            .and_modify(|auth| {
                *auth = authorization;
            });
        self
    }

    /// 发送请求到aliyun endpoint
    pub fn send(self) -> impl Future<Output = Result<Response, Error>> {
        let client = reqwest::Client::new();

        let url = format!("https://{}{}", self.host, self.canonical_uri);

        let mut hashmap = HeaderMap::new();
        for (k, v) in self.headers {
            hashmap.insert(
                get_str_at_location(k.as_ptr() as usize, k.len()),
                v.parse().unwrap(),
            );
        }
        let mut query_list = Vec::new();
        for (k, v) in self.query_param {
            query_list.push((k, v));
        }

        let method = Method::from_bytes(self.http_method.as_bytes()).unwrap();

        let client = client
            .request(method, url)
            .headers(hashmap)
            .query(&query_list)
            .body(self.body.to_string());

        #[cfg(debug_assertions)]
        debug!("{:#?}", client);

        client.send()
    }
}

// 获取 static str
fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    // 使用裸指针需要 `unsafe{}` 语句块
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

pub trait Api {
    fn new() -> Self;
    fn name(&self) -> String;
    fn send(self) -> impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>;
}
