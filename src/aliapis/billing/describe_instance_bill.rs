use api_derive::Api;
use ordermap::OrderMap;
use serde::Deserialize;
use serde::Serialize;

///查询用户某个账期内所有商品实例或计费项的消费汇总
#[derive(Debug, Clone, Api)]
pub struct DescribeInstanceBill(OrderMap<String, String>);


impl DescribeInstanceBill {
    /// 必选，账期 YYYY－MM。仅支持最近 18 个月账期。实例：2024-11
    pub fn set_billing_cycle(mut self, billing_cycle: &str) -> Self {
        self.0
            .insert_sorted("BillingCycle".into(), billing_cycle.into());
        self
    }
    /// 产品代码
    pub fn set_product_code(mut self, product_code: &str) -> Self {
        self.0
            .insert_sorted("ProductCode".into(), product_code.into());
        self
    }
    /// 产品类型
    pub fn set_product_type(mut self, product_type: &str) -> Self {
        self.0
            .insert_sorted("ProductType".into(), product_type.into());
        self
    }
    ///订阅类型。取值：
    ///Subscription：预付费。
    ///PayAsYouGo：后付费。
    pub fn set_subscription_type(mut self, subscription_type: &str) -> Self {
        self.0
            .insert_sorted("SubscriptionType".into(), subscription_type.into());
        self
    }
    ///是否按照计费项维度拉取数据。
    ///false。与费用中心-费用账单-账单明细-实例账单一致。
    ///true。与费用中心-费用账单-账单明细-计费项账单一致。
    pub fn set_is_billing_item(mut self, is_billing_item: &str) -> Self {
        self.0
            .insert_sorted("IsBillingItem".into(), is_billing_item.into());
        self
    }
    ///根据原价（PretaxGrossAmount）和应付（PretaxAmount）是否都为 0 做过滤。取值：
    ///false。
    ///true。
    pub fn set_is_hide_zero_charge(mut self, is_hide_zero_charge: &str) -> Self {
        self.0
            .insert_sorted("IsHideZeroCharge".into(), is_hide_zero_charge.into());
        self
    }
    ///账单日期，仅当 Granularity 为 DAILY 时必填，格式为 YYYY-MM-DD。
    pub fn set_billing_date(mut self, billing_date: &str) -> Self {
        self.0
            .insert_sorted("BillingDate".into(), billing_date.into());
        self
    }
    ///查询账单的颗粒度。取值如下：
    ///MONTHLY：月。与费用中心-费用账单-账单明细-账期账单一致。
    ///DAILY：日。与费用中心-费用账单-账单明细-按天账单一致。
    pub fn set_granularity(mut self, granularity: &str) -> Self {
        self.0
            .insert_sorted("Granularity".into(), granularity.into());
        self
    }
    ///资源归属账号 ID，资源归属账号是实际使用资源的账号。
    pub fn set_bill_owner_id(mut self, bill_owner_id: &str) -> Self {
        self.0
            .insert_sorted("BillOwnerId".into(), bill_owner_id.into());
        self
    }
    ///实例 ID。
    pub fn set_instance_id(mut self, instance_id: &str) -> Self {
        self.0
            .insert_sorted("InstanceID".into(), instance_id.into());
        self
    }
    ///产品 Code，与费用中心账单产品 Code 一致。
    pub fn set_pip_code(mut self, pip_code: &str) -> Self {
        self.0.insert_sorted("PipCode".into(), pip_code.into());
        self
    }
    ///用来表示当前调用开始读取的位置，参数值必须为空或者使用返回结果中的 NextToken 设值，否则会报错。空代表从头开始读取。
    pub fn set_next_token(mut self, next_token: &str) -> Self {
        self.0.insert_sorted("NextToken".into(), next_token.into());
        self
    }
    ///本次读取的最大数据记录数量。默认值：300，最大值：300。
    pub fn set_max_resutls(mut self, max_resutls: &str) -> Self {
        self.0
            .insert_sorted("MaxResults".into(), max_resutls.into());
        self
    }
}
/* 
impl Api for DescribeInstanceBill {
    fn new() -> Self {
        let mut parameters: OrderMap<String, String> = OrderMap::new();
        parameters.insert_sorted("MaxResults".into(), "300".into());

        DescribeInstanceBill(parameters)
    }

    fn name(&self) -> String {
        "DescribeInstanceBill".to_string()
    }

    fn send(self) -> impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>> {
        RequestHeader::new(
            super::ENDPOINT.to_string(),
            self.name(),
            super::VERSION.to_string(),
            self.0,
        )
        .sign()
        .send()
    }
}
*/

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeInstanceBillResponse {
    pub message: String,
    pub request_id: String,
    pub data: Data,
    pub code: String,
    pub success: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub billing_cycle: String,
    pub total_count: f64,
    #[serde(rename = "AccountID")]
    pub account_id: String,
    pub max_results: f64,
    pub items: Vec<Item>,
    pub account_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    pub instance_spec: String,
    pub product_name: String,
    #[serde(rename = "InstanceID")]
    pub instance_id: String,
    #[serde(rename = "BillAccountID")]
    pub bill_account_id: String,
    pub deducted_by_cash_coupons: f64,
    pub billing_date: String,
    pub list_price_unit: String,
    pub payment_amount: f64,
    pub list_price: String,
    pub deducted_by_prepaid_card: f64,
    pub invoice_discount: f64,
    pub item: String,
    pub subscription_type: String,
    pub pretax_gross_amount: f64,
    pub instance_config: String,
    pub currency: String,
    pub commodity_code: String,
    pub item_name: String,
    pub cost_unit: String,
    pub resource_group: String,
    pub adjust_amount: f64,
    pub billing_type: String,
    pub deducted_by_coupons: f64,
    pub usage: String,
    pub product_detail: String,
    pub product_code: String,
    pub zone: String,
    pub product_type: String,
    pub outstanding_amount: f64,
    pub biz_type: String,
    pub billing_item: String,
    pub nick_name: String,
    pub pip_code: String,
    #[serde(rename = "IntranetIP")]
    pub intranet_ip: String,
    pub service_period_unit: String,
    pub service_period: String,
    pub deducted_by_resource_package: String,
    pub usage_unit: String,
    #[serde(rename = "InternetIP")]
    pub internet_ip: String,
    pub pretax_amount: f64,
    #[serde(rename = "OwnerID")]
    pub owner_id: String,
    pub bill_account_name: String,
    pub region: String,
    pub tag: String,
    pub cash_amount: f64,
}

#[cfg(test)]
mod tests {
    use crate::{aliapis::billing::describe_instance_bill::DescribeInstanceBill, Api};

    #[tokio::test]
    async fn works() -> anyhow::Result<()> {
        let billing_cycle = chrono::Local::now().format("%Y-%m").to_string();
        let test_api = DescribeInstanceBill::new().set_billing_cycle(&billing_cycle);
        let response = test_api.send().await?;

        assert_eq!(response.status(), 200);
        Ok(())
    }
}
