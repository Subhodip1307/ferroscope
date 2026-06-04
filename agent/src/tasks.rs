use serde_json::json;
use tokio::time::{Duration, interval};
use crate:: {system::logic,Payload,set_up::BaseConFig,jobs,set_up,system};
use std::{sync::Arc};
use reqwest::StatusCode;
use tokio::sync::mpsc;
use reqwest::Client;

pub async fn spawn_info_sender(conf:Arc<BaseConFig>,tx:tokio::sync::mpsc::Sender<Payload>,api_client:Arc<Client>){
    // sening the systeminfo first
    {
        let sys = logic::systeminfo();
        let send_data = format!("{}/send_systeminfo", conf.get_server_url());
        let _=tx.send(Payload { endpoint: send_data, body: json!(&sys) }).await;
    }

    {
        let jobs_api = tx.clone();
        let conf1 = Arc::clone(&conf);
        tokio::spawn(async move { jobs::executor::run(jobs_api, conf1).await });
    }
    // cpu
    {
        let system_conf: Arc<set_up::BaseConFig> = conf.clone();
        let system_api_sender = tx.clone();
        tokio::spawn(async move {
            let mut tick = interval(Duration::from_secs(system_conf.get_cpu_interval()));
            loop {
                system::send_cpu(system_conf.clone(), system_api_sender.clone()).await;
                tick.tick().await;
            }
        });
    }
    // Ram
    {
        let system_conf: Arc<set_up::BaseConFig> = conf.clone();
        let memory_api_sender= tx.clone();
        tokio::spawn(async move {
            let mut tick = interval(Duration::from_secs(system_conf.get_ram_interval()));
            loop {
                system::send_memory(system_conf.clone(), memory_api_sender.clone()).await;
                tick.tick().await;
            }
        });
    }
    
    // uptime
    {
    let system_conf: Arc<set_up::BaseConFig> = conf.clone();
    let system_api_sender= tx.clone();

    tokio::spawn(async move{
        let mut tick = interval(Duration::from_secs(system_conf.get_uptime_interval()));
        loop {
        system::send_uptime(system_conf.clone(), system_api_sender.clone()).await;
        tick.tick().await;
        }
    });
    }

    // disk io
    {
        let disk_conf: Arc<set_up::BaseConFig> = conf.clone();
        let system_api_sender= tx.clone();
        tokio::spawn(async move{
        let base_url = format!("{}/send_disk_io", disk_conf.get_server_url());

        let mut tick = interval(Duration::from_secs(disk_conf.get_disk_io_interval()));
        loop {
        // let data=HashMap::from([("data",)]);
        system_api_sender.send(Payload { endpoint:base_url.clone() , body: json!(logic::get_disk_io().await) }).await.unwrap();
        tick.tick().await;
        }
        });

    }
    // helth api
    let system_conf: Arc<set_up::BaseConFig> = conf.clone();
    let mut tick = interval(Duration::from_secs(10));
    loop {
            let _=api_client.get(&format!("{}/helth_check",system_conf.get_server_url())).send().await;
            tick.tick().await;
    }
    

}



pub async fn queue(api_client:Arc<Client>,mut receiver:mpsc::Receiver<Payload>){
    loop {
        while let Some(payload)=receiver.recv().await{
            // println!("send data for {:?}",payload);
            match api_client.post(&payload.endpoint).json(&payload.body).send().await{
                Ok(res)=>{
                    println!("the status code is {}",res.status());
                if res.status() == StatusCode::UNAUTHORIZED {
                    println!("Invalid Access Token");
                    std::process::exit(1);
                }
                },
                Err(e)=>{println!("getting error {e}, going to sleep");
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                // doing helth check
                for i in 1..6{
                    if api_client.post(&payload.endpoint).json(&payload.body).send().await.is_ok() {
                        println!("Server is reachble again");
                        break;
                   }//endif
                println!("Retrying after {} sec ",i*60);
                tokio::time::sleep(std::time::Duration::from_secs(i*60)).await
                }//endfor  
            }
            }
        }
    }
}