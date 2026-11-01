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
use tokio::time::{Duration, interval,sleep};
use tokio::sync::mpsc;
use super::structures::Recovery;
use sha2::{Digest, Sha256};
use tokio::process::Command;

static CONFDIR: LazyLock<String> =
    LazyLock::new(|| env::var("CONF_DIR").unwrap_or("/etc/ferroscope_agent".to_string()));

pub async fn run(api_queue: mpsc::Sender<Payload>, config: Arc<BaseConFig>) {
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

    let (fall_back_sender,fall_back_receiver)=mpsc::channel::<Recovery>(100);
    fall_back_executor(fall_back_receiver).await;

    let web_fallback_sender=fall_back_sender.clone();
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(web_config.get_web_interval()));
        loop {
            println!("the conf file is {:?}", web_config_files.clone());
            web_runner(
                api_queue2.clone(),
                Arc::clone(&web_config),
                web_config_files.clone(),web_fallback_sender.clone()
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
            host_configs_files.clone(),fall_back_sender.clone()
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
// TODO write a function that will run and executed the bash scripts 
async  fn fall_back_executor(mut fall_back_receiver:mpsc::Receiver<Recovery>){
    tokio::spawn(async move {
    let timeout = Duration::from_secs(120);
    while let Some(data)=fall_back_receiver.recv().await{
        println!("got first value");
        // checking hash of that file
        if let Ok(content)=tokio::fs::read(&data.file).await{
            let hex = hex::encode(Sha256::digest(&content));
            if hex ==data.hash {
                // hash match done now run the bash

                tokio::select! {
                result = Command::new("bash").arg(data.file).output() => {
                    let output = result.expect("failed to run script");
                    println!("status: {}", output.status);
                    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                }
                _ = sleep(timeout) => {
                    println!("script killed: exceeded time limit");
                }
            }
    
            }
        }
    }
    });
}