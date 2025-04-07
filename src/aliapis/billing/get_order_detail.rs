use api_derive::Api;

use ordermap::OrderMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Api)]
pub struct GetOrderDetail(OrderMap<String, String>);

impl GetOrderDetail {
    pub fn set_order_id(mut self, order_id: &str) -> Self {
        self.0
            .insert_sorted("OrderId".into(), order_id.into());
        self
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetOrderDetailResponse {
    pub message: String,
    pub request_id: String,
    pub data: Data,
    pub code: String,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub order_list: OrderList,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OrderList {
    pub order: Vec<Order>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Order {
    pub operator: Option<String>,
    pub product_code: String,
    pub config: String,
    pub sub_order_id: String,
    pub create_time: String,
    pub quantity: f64,
    pub payment_time: Option<String>,
    pub order_id: String,
    pub bill_module_config: BillModuleConfig,
    pub original_module_config: OriginalModuleConfig,
    pub usage_end_time: String,
    pub subscription_type: String,
    pub pretax_gross_amount: f64,
    pub order_type: String,
    pub order_sub_type: String,
    pub pretax_amount: f64,
    pub currency: String,
    pub commodity_code: String,
    pub usage_start_time: String,
    pub original_config: String,
    pub instance_i_ds: String,
    pub payment_status: String,
    pub extend_infos: ExtendInfos,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillModuleConfig {
    pub bill_module_config: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExtendInfos {
    pub discount_amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginalModuleConfig {
    pub original_module_config: Vec<OriginalModuleConfigElement>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OriginalModuleConfigElement {
    pub module_properties: ModuleProperties,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleProperties {
    pub module_properties: Vec<ModuleProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleProperty {
    pub value: String,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use crate::Api;


    #[tokio::test]
    async fn works() -> anyhow::Result<()> {
        let billing_cycle = chrono::Local::now().format("%Y-%m").to_string();
        let test_api = super::GetOrderDetail::new().set_order_id("2024010100000001");
        let response = test_api.send().await?;

        assert_eq!(response.status(), 200);
        Ok(())
    }
} 