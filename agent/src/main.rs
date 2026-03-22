use serde_json::json;
use tokio::time::{Duration, interval};
mod system;
use system::logic;
mod jobs;
use reqwest::{Client, header};
use std::sync::Arc;
mod set_up;
use tokio::sync::mpsc;
use reqwest::StatusCode;

#[derive(Debug)]
struct Payload {
    endpoint: String,
    body: serde_json::Value,
}


#[tokio::main]
async fn main() {
    // set-up
    let conf = {
        let service_setup = set_up::ConfSetUp::new();
        service_setup.set_up().await;
        println!("runing next");
        Arc::new(service_setup.get_config().unwrap())
    };

    let mut __headers = header::HeaderMap::new();
    __headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(conf.get_auth_token())
            .expect("something went wrong in Header"),
    );
    let api_client = Arc::new(
        Client::builder()
            .default_headers(__headers)
            .timeout(std::time::Duration::from_secs(conf.get_api_time_out()))
            .build()
            .expect("errr"),
    );
    let (tx, rx) = mpsc::channel::<Payload>(10);

    // starting queue
    let jobs_api: Arc<Client> = Arc::clone(&api_client);
    tokio::spawn(async move {
        queue(jobs_api, rx).await;
    });

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
                println!("cpu send");
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
    let system_conf: Arc<set_up::BaseConFig> = conf.clone();
    let system_api_sender= tx.clone();

    let mut tick = interval(Duration::from_secs(system_conf.get_uptime_interval()));
    loop {
        system::send_uptime(system_conf.clone(), system_api_sender.clone()).await;
        tick.tick().await;
    }
}


async fn queue(api_client:Arc<Client>,mut receiver:mpsc::Receiver<Payload>){
    loop {
        while let Some(payload)=receiver.recv().await{
            // println!("send data for {:?}",payload);
            match api_client.post(&payload.endpoint).json(&payload.body).send().await{
                Ok(res)=>{println!("the status code is {}",res.status());
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

