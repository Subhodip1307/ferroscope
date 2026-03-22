// execute the logics
use super::host::host_runner;
use super::web::web_runner;
use crate::set_up::BaseConFig;
use std::sync::Arc;
use tokio::time::{Duration, interval};
use crate::Payload;


pub async fn run(api_queue:tokio::sync::mpsc::Sender<Payload>, config: Arc<BaseConFig>) {
    let  api_queue2= api_queue.clone();
    let web_config = Arc::clone(&config);
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(web_config.get_web_interval()));
        loop {
            web_runner(api_queue2.clone(), Arc::clone(&web_config)).await;
            ticker.tick().await;
            // TODO: handel error and if get error then exit the loop
        }
    });
    let mut ticker = interval(Duration::from_secs(config.get_host_interval()));
    loop {
        host_runner(api_queue.clone(), Arc::clone(&config)).await;
        ticker.tick().await;
    }
}




