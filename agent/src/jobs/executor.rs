// execute the logics
use super::host::host_runner;
use super::web::web_runner;
use crate::set_up::BaseConFig;
use reqwest::Client;
use std::sync::Arc;
use tokio::time::{Duration, interval};



pub async fn run(api_client: Arc<Client>, config: Arc<BaseConFig>) {
    let web_client = Arc::clone(&api_client);
    let web_config = Arc::clone(&config);
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(web_config.get_web_interval()));
        loop {
            web_runner(Arc::clone(&web_client), Arc::clone(&web_config)).await;
            ticker.tick().await;
            // TODO: handel error and if get error then exit the loop
        }
    });
    let mut ticker = interval(Duration::from_secs(config.get_host_interval()));
    loop {
        host_runner(Arc::clone(&api_client), Arc::clone(&config)).await;
        ticker.tick().await;
    }
}




