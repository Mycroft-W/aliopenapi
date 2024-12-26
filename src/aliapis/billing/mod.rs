pub mod describe_instance_bill;
pub mod query_resource_package_instances;
pub mod query_account_balance;
pub mod query_dp_utilization_detail;

pub use describe_instance_bill::{DescribeInstanceBill,DescribeInstanceBillResponse};
pub use query_account_balance::{QueryAccountBalance,QueryAccountBalanceResponse};
pub use query_dp_utilization_detail::{QueryDPUtilizationDetail,QueryDPUtilizationDetailResponse};
pub use query_resource_package_instances::{QueryResourcePackageInstances,QueryResourcePackageInstancesResponse};

const VERSION: &str = "2017-12-14";
const ENDPOINT: &str = "business.aliyuncs.com";
