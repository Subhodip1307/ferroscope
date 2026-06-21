use serde_json::json;
// Executor
use super::config_reader::load_config;
use super::structures::{BaseFormat, Host};
use crate::Payload;
use crate::set_up::BaseConFig;
use arc_swap::ArcSwap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};

async fn host_check(host: &Host) -> bool {
    timeout(Duration::from_secs(2), TcpStream::connect(host.addr()))
        .await
        .is_ok()
}

pub(super) async fn host_runner(
    api_queue: tokio::sync::mpsc::Sender<Payload>,
    config: Arc<BaseConFig>,
    all_files: Arc<ArcSwap<Vec<String>>>,
) {
    let baseapi = config.get_service_url();
    let all_files_snapshorts = all_files.load();
    for file in all_files_snapshorts.iter() {
        let a: config::Config = match load_config(file).await {
            Ok(value) => value,
            Err(e) => {
                println!("error is {}", e);
                continue;
            }
        };

        let value: Host = match a.try_deserialize() {
            Ok(value) => value,
            Err(e) => {
                println!("error is {}", e);
                continue;
            }
        };
        let host_status = host_check(&value).await;
        api_queue
            .send(Payload {
                endpoint: baseapi.clone(),
                body: json!(BaseFormat {
                    service_name: value.name,
                    category: "Host".to_string(),
                    ssl_exp: None,
                    status: if host_status {
                        "up".to_string()
                    } else {
                        "down".to_string()
                    },
                    error_msg: "".to_string(),
                }),
            })
            .await
            .unwrap();
    } //endfor
}
