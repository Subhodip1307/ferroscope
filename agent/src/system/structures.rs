use serde::Serialize;

#[derive(Debug)]
pub struct __Memory {
    total: String,
    free: String,
}
impl __Memory {
    pub fn new(total: u64, free: Option<u64>) -> Self {
        let total_memory_bytes = total as f64 / (1024.0 * 1024.0 * 1024.0);
        let free_memory_bytes = free.unwrap_or_default() as f64 / (1024.0 * 1024.0 * 1024.0);
        Self {
            total: format!("{:.2} GiB", total_memory_bytes),
            free: format!("{:.2} GiB", free_memory_bytes),
        }
    }
    pub fn get_free(&self) -> &str {
        &self.free
    }

    pub fn get_total(&self) -> &str {
        &self.total
    }
}
// pub struct __Cprocesses {
//     // current process info
//     running_process: Option<u32>,
//     blocked_process: Option<u32>,
//     total_process: u64,
// }
// impl __Cprocesses {
//     pub fn new(
//         running_process: Option<u32>,
//         blocked_process: Option<u32>,
//         total_process: u64,
//     ) -> Self {
//         Self {
//             running_process,
//             blocked_process,
//             total_process,
//         }
//     }
//     pub fn runinng(&self) -> u32 {
//         self.running_process.unwrap_or(0)
//     }

//     pub fn blocked(&self) -> u32 {
//         self.blocked_process.unwrap_or(0)
//     }

//     pub fn total(&self) -> u64 {
//         self.total_process
//     }
// }

#[derive(Serialize)]
pub struct __SysInfo {
    pub system_name: Option<String>,
    pub kernel_version: Option<String>,
    pub os_version: Option<String>,
    pub uptime: u64,
    pub cpu_threads: usize,
    pub cpu_vendor:String,
}
