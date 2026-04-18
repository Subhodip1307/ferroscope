use crate::AppState;
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

            for key in &key_vec {
                // remove the key
                app_data.helth_check.remove(key);

                let cpu_ket = &format!("node_cpu_strem_{}", key);
                if let Some(cpu_listender) = app_data.cpu_strem.get(cpu_ket) {
                    let _ = cpu_listender.send(LatestCpu {
                        value: -100.0,
                        date_time: Utc::now(),
                    });
                    // removes the key also
                    drop(cpu_listender); //dropping the lock
                    app_data.cpu_strem.remove(cpu_ket);
                };
                // remove the ram stream and cpu stream
                let ram_key = &format!("node_ram_strem_{}", key);
                if let Some(ram_listender) = app_data.ram_strem.get(ram_key) {
                    let _ = ram_listender.send(LatestRam {
                        free: String::from("STOP"),
                        total: String::new(),
                        timestamp: Utc::now(),
                    });
                    drop(ram_listender);
                    app_data.ram_strem.remove(ram_key);
                };
                //  Send Notification
                let _ = app_state
                    .notifier
                    .send(NotificationData {
                        category: EventType::NODE,
                        sujbect: "Node Status unreachable".to_string(),
                        unique_id: format!("{}", key),
                    })
                    .await;
            } //end for
        }
    });
}
