// receive
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct CpuStats {
    pub cpu: f64,
}

#[derive(Deserialize, Debug)]
pub struct MemoryStats {
    pub free: String,
    pub total: String,
}

#[derive(Deserialize, Debug)]
pub struct ServiceMonitor {
    pub service_name: String,
    pub status: String, //runing or dead
    pub error_msg: Option<String>,
}

#[derive(Deserialize,Debug)]
// same strcut being used in user view for res and db query
pub(super) struct SysInfo {
    pub system_name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub uptime: i64,
    pub cpu_threads: i16,
    pub cpu_vendor:String,
}
