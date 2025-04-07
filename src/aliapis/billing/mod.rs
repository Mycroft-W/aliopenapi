const VERSION: &str = "2017-12-14";
const ENDPOINT: &str = "business.aliyuncs.com";

pub mod describe_instance_bill;
pub use describe_instance_bill::{DescribeInstanceBill,DescribeInstanceBillResponse};

pub mod query_account_balance;
pub use query_account_balance::{QueryAccountBalance,QueryAccountBalanceResponse};

pub mod query_dp_utilization_detail;
pub use query_dp_utilization_detail::{QueryDPUtilizationDetail,QueryDPUtilizationDetailResponse};

pub mod query_resource_package_instances;
pub use query_resource_package_instances::{QueryResourcePackageInstances,QueryResourcePackageInstancesResponse};

pub mod query_orders;
pub use query_orders::{QueryOrders,QueryOrdersResponse};

pub mod get_order_detail;
pub use get_order_detail::{GetOrderDetail,GetOrderDetailResponse};
