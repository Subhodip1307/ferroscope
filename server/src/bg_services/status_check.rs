use crate::AppState;
use crate::objects::StreamPayLoad;
use crate::user_views::LatestCpu;
use crate::user_views::LatestRam;
use chrono::Utc;
use ferroscope_server::global::structure::{EventType, NotificationData};
use tokio::time::{Duration, interval};

pub async fn node_status_check(app_state: AppState) {
    // runing backgrond services
    tokio::spawn(async move {
        let app_data = app_state.clone();

        #[cfg(debug_assertions)]
        let timeout = 10_000; //test
        #[cfg(not(debug_assertions))]
        let timeout = 30_000; //production

        let mut tick = interval(Duration::from_secs(30));
        loop {
            tick.tick().await;
            let mut key_vec: Vec<i64> = Vec::with_capacity(app_data.helth_check.len());
            for entry in app_data.helth_check.iter() {
                let key = entry.key();
                let value = *entry.value();

                let current = ferroscope_server::global::utils_functions::current_time();

                if current - value > timeout {
                    println!("helth check failed");
                    key_vec.push(*key);
                }
            } //end for remove dashmap lock

            for key in key_vec {
                // remove the key
                let cpu_ket = &format!("node_cpu_strem_{}", key);
                let ram_key = &format!("node_ram_strem_{}", key);
                let disk_key = &format!("node_diskio_strem_{}", key);
                 app_data.stream_data.remove(cpu_ket);
                 app_data.stream_data.remove(ram_key);
                 app_data.stream_data.remove(disk_key);
            
            } //end for
        }
    });
}
