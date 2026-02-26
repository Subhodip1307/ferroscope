use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub (super) struct UserToken{
    pub token: String,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub(super) struct NodesList {
    id: i64,
    name: String,
}

#[derive(sqlx::FromRow,Debug,Serialize)]
// same strcut being used in user view for res and db query
pub(super) struct SysInfo {
    pub system_name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub uptime: i64,
    pub cpu_threads: i16,
    pub cpu_vendor:String,
}



#[derive(sqlx::FromRow, Debug, Serialize)]
pub(super) struct LatestCpu {
    pub value: f64,
    pub date_time: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub(super) struct LatestRam {
    pub free: String,
    pub total: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub(super) struct ServiceList {
    service_name: String,
}

#[derive(Debug, Serialize)]
pub(super) struct SingleServiceStatus {
    pub status: String,
    pub service_status: String,
    pub error_msg: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub(super) struct ServiceStatus {
    pub error_msg: String,
    pub status: String,
    pub service_status: String,
}