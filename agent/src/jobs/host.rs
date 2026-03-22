use serde_json::json;
// Executor
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};
use super::config_reader::{file_name_list, load_config};
use super::structures::{BaseFormat, Host};
use std::sync::Arc;
use crate::set_up::BaseConFig;
use crate::Payload;
use std::sync::LazyLock;
use std::env;

static CONFDIR: LazyLock<String> =
    LazyLock::new(|| env::var("CONF_DIR").unwrap_or("/etc/ferroscope_agent".to_string()));

async fn host_check(host: &Host) -> bool {
    timeout(Duration::from_secs(2), TcpStream::connect(host.addr()))
        .await
        .is_ok()
}

pub(super) async fn host_runner(api_queue:tokio::sync::mpsc::Sender<Payload>, config: Arc<BaseConFig>) {
    let all_files = match file_name_list(&format!("{}/Host", *CONFDIR)).await {
        Ok(value) => value,
        Err(e) => {
            println!("no config founnd err: {}", e);
            Vec::new()
        }
    };
    let baseapi = config.get_service_url();
    for file in all_files {
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
        let _=api_queue.send(Payload { endpoint: baseapi.clone(), body:json!(BaseFormat {
                service_name: value.name,
                category: "Host".to_string(),
                ssl_exp: None,
                status: if host_status {
                    "up".to_string()
                } else {
                    "down".to_string()
                },
                error_msg: "".to_string(),
            }) }).await;
    } //endfor
}