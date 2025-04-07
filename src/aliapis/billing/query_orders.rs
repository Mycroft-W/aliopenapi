use api_derive::Api;

use ordermap::OrderMap;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Api)]
pub struct QueryOrders(OrderMap<String, String>);

impl QueryOrders {
    pub fn set_create_time_end(mut self, create_time_end: &str) -> Self {
        self.0
            .insert_sorted("CreateTimeEnd".into(), create_time_end.into());
        self
    }
    pub fn set_prod_code(mut self, prod_code: &str) -> Self {
        self.0
            .insert_sorted("ProdCode".into(), prod_code.into());
        self
    }
    pub fn set_create_time_start(mut self, create_time_start: &str) -> Self {
        self.0
            .insert_sorted("CreateTimeStart".into(), create_time_start.into());
        self
    }
    pub fn set_page_num(mut self, page_num: &str) -> Self {
        self.0
            .insert_sorted("PageNum".into(), page_num.into());
        self
    }
    pub fn set_page_size(mut self, page_size: &str) -> Self {
        self.0
            .insert_sorted("PageSize".into(), page_size.into());
        self
    }
    pub fn set_product_type(mut self, product_type: &str) -> Self {
        self.0
            .insert_sorted("ProductType".into(), product_type.into());
        self
    }
    pub fn set_subscription_type(mut self, subscription_type: &str) -> Self {
        self.0
            .insert_sorted("SubscriptionType".into(), subscription_type.into());
        self
    }
    pub fn set_payment_status(mut self, payment_status: &str) -> Self {
        self.0
            .insert_sorted("PaymentStatus".into(), payment_status.into());
        self
    }
    pub fn set_order_type(mut self, order_type: &str) -> Self {
        self.0
            .insert_sorted("OrderType".into(), order_type.into());
        self
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryOrdersResponse {
    pub message: String,
    pub request_id: String,
    pub data: Data,
    pub code: String,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub total_count: f64,
    pub page_num: i64,
    pub page_size: i64,
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
    pub subscription_type: String,
    pub pretax_gross_amount: f64,
    pub order_type: String,
    pub pretax_amount: f64,
    pub product_code: String,
    pub create_time: String,
    pub currency: String,
    pub commodity_code: String,
    pub product_type: Option<String>,
    pub payment_time: Option<String>,
    pub order_id: String,
    pub payment_status: String,
}



#[cfg(test)]
mod tests {
    use crate::Api;

    use super::QueryOrders;

    #[tokio::test]
    async fn works() -> anyhow::Result<()> {
        let billing_cycle = chrono::Local::now().format("%Y-%m").to_string();
        let test_api = QueryOrders::new().set_create_time_end("2024-01-01T00:00:00Z").set_page_size("300");
        let response = test_api.send().await?;

        assert_eq!(response.status(), 200);
        Ok(())
    }
} 