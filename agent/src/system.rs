pub mod logic;
pub mod structures;
use crate::set_up;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use crate::Payload;

pub async fn send_cpu(conf: Arc<set_up::BaseConFig>, api_queue:tokio::sync::mpsc::Sender<Payload>) {
    let send_data = format!("{}/send_cpu", conf.get_server_url());
    let cpu_usage: HashMap<&str, f64> =
        HashMap::from([("cpu", (logic::total_cpu_usage().await.unwrap() * 100.0).round())]);
    api_queue.send(Payload { endpoint: send_data, body: json!(cpu_usage) }).await.unwrap();
}

pub async fn send_uptime(conf: Arc<set_up::BaseConFig>, api_queue:tokio::sync::mpsc::Sender<Payload>) {
    let uptime: HashMap<&str, u64> = HashMap::from([("uptime_sec", logic::get_uptime().unwrap())]);
    let send_data = format!("{}/send_uptime", conf.get_server_url());
    api_queue.send(Payload { endpoint: send_data, body: json!(uptime) }).await.unwrap();
}

pub async fn send_memory(conf: Arc<set_up::BaseConFig>, api_queue:tokio::sync::mpsc::Sender<Payload>) {
    let send_data = format!("{}/send_memory", conf.get_server_url());
    let memory = logic::memory_usage().unwrap();
    memory.get_total();
    let data: HashMap<&str, &str> =
        HashMap::from([("free", memory.get_free()), ("total", memory.get_total())]);
    api_queue.send(Payload { endpoint: send_data, body: json!(data) }).await.unwrap();
}
