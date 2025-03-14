const VERSION: &str = "2015-01-09";
const ENDPOINT: &str = "alidns.cn-hangzhou.aliyuncs.com";

pub mod describe_domains;
pub use describe_domain_records::{DescribeDomainRecords,DescribeDomainRecordsResponse};

pub mod set_domain_record_status;
pub use set_domain_record_status::{SetDomainRecordStatus,SetDomainRecordStatusResponse};

pub mod describe_domain_records;
pub use describe_domains::{DescribeDomains,DescribeDomainsResponse};

pub mod add_domain_record;
pub use add_domain_record::{AddDomainRecord,AddDomainRecordResponse};



