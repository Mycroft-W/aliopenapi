# 阿里云openapi的简单实现调用

根据官方文档（[V3版本](https://help.aliyun.com/zh/sdk/product-overview/v3-request-structure-and-signature?spm=a2c4g.11186623.0.0.4bd02addteCnVx)）实现签名,目前仅实现了少量api

阿里云AK/SK 通过读取环境变量获得
* ALI_CLOUD_ACCESSKEY_ID
* ALI_CLOUD_ACCESSKEY_SECRET

## 调用方式
```
// 实例化api,使用set_xxx() 设置参数
// api参数参考官方文档，https://api.aliyun.com/document?spm=api-workbench.home.0.0.5f4d42503kTM2t
let api = QuerySendStatistics::new()
            .set_start_date(&date)
            .set_end_date(&date)
            .set_is_globe("1");

// send() 会先调用 canonical_request() 规范化请求头，然后调用 sign() 使用AK/SK签名
let response = api.send().await?;
```