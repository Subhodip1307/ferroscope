// execute the logics
use super::config_reader::file_name_list;
use super::host::host_runner;
use super::web::web_runner;
use crate::Payload;
use crate::set_up::BaseConFig;
use arc_swap::ArcSwap;
use std::env;
use std::sync::{Arc, LazyLock};
use tokio::signal::unix::{SignalKind, signal};
use tokio::time::{Duration, interval};

static CONFDIR: LazyLock<String> =
    LazyLock::new(|| env::var("CONF_DIR").unwrap_or("/etc/ferroscope_agent".to_string()));

pub async fn run(api_queue: tokio::sync::mpsc::Sender<Payload>, config: Arc<BaseConFig>) {
    let api_queue2 = api_queue.clone();
    let web_config = Arc::clone(&config);
    let web_config_files = Arc::new(ArcSwap::from_pointee(
        file_name_list(&format!("{}/Web", *CONFDIR))
            .await
            .unwrap_or_default(),
    ));
    let host_configs_files = Arc::new(ArcSwap::from_pointee(
        file_name_list(&format!("{}/Host", *CONFDIR))
            .await
            .unwrap_or_default(),
    ));

    {
        //start
        let web = web_config_files.clone();
        let host = host_configs_files.clone();
        tokio::spawn(async move {
            reload_conf(web.clone(), host).await;
        });
    } //end

    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(web_config.get_web_interval()));
        loop {
            println!("the conf file is {:?}", web_config_files.clone());
            web_runner(
                api_queue2.clone(),
                Arc::clone(&web_config),
                web_config_files.clone(),
            )
            .await;
            ticker.tick().await;
            // TODO: handel error and if get error then exit the loop
        }
    });
    let mut ticker = interval(Duration::from_secs(config.get_host_interval()));
    loop {
        host_runner(
            api_queue.clone(),
            Arc::clone(&config),
            host_configs_files.clone(),
        )
        .await;
        ticker.tick().await;
    }
}

type ConfigType = Arc<ArcSwap<Vec<String>>>;

async fn reload_conf(web_configs: ConfigType, host_configs: ConfigType) {
    // will move code letter from where
    let mut reload_sig = signal(SignalKind::hangup()).unwrap();
    loop {
        reload_sig.recv().await;
        println!("Reloading the files");
        web_configs.store(Arc::new(
            file_name_list(&format!("{}/Web", *CONFDIR))
                .await
                .unwrap_or_default(),
                
        ));
        host_configs.store(Arc::new(
            file_name_list(&format!("{}/Host", *CONFDIR))
                .await
                .unwrap_or_default(),
                
        ));
    }
}
