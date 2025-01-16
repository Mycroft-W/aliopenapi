pub mod describe_domains;
pub mod set_domain_record_status;
pub mod describe_domain_records;
pub use describe_domains::{DescribeDomains,DescribeDomainsResponse};
pub use set_domain_record_status::{SetDomainRecordStatus,SetDomainRecordStatusResponse};
pub use describe_domain_records::{DescribeDomainRecords,DescribeDomainRecordsResponse};

const VERSION: &str = "2015-01-09";
const ENDPOINT: &str = "alidns.cn-hangzhou.aliyuncs.com";