use tokio::time::{Duration, interval};
mod system;
use system::logic;
mod jobs;
use reqwest::{Client, header};
use std::sync::Arc;
mod set_up;

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
    // sening the systeminfo first
    {
        let sys = logic::systeminfo();
        // let conf1=Arc::clone(&conf);
        let send_data = format!("{}/send_systeminfo", conf.get_server_url());
        let _ = api_client.post(send_data).json(&sys).send().await.unwrap();
        // println!("{:?}",a);
    }

    {
        let jobs_api: Arc<Client> = Arc::clone(&api_client);
        let conf1 = Arc::clone(&conf);
        tokio::spawn(async move { jobs::executor::run(jobs_api, conf1).await });
    }
    // cpu
    {
        let system_conf: Arc<set_up::BaseConFig> = conf.clone();
        let system_api_client: Arc<Client> = api_client.clone();
        tokio::spawn(async move {
            let mut tick = interval(Duration::from_secs(system_conf.get_cpu_interval()));
            loop {
                system::send_cpu(system_conf.clone(), system_api_client.clone()).await;
                println!("cpu send");
                tick.tick().await;
            }
        });
    }
    // Ram
    {
        let system_conf: Arc<set_up::BaseConFig> = conf.clone();
        let system_api_client: Arc<Client> = api_client.clone();
        tokio::spawn(async move {
            let mut tick = interval(Duration::from_secs(system_conf.get_ram_interval()));
            loop {
                system::send_memory(system_conf.clone(), system_api_client.clone()).await;
                tick.tick().await;
            }
        });
    }
    // uptime
    let system_conf: Arc<set_up::BaseConFig> = conf.clone();
    let system_api_client: Arc<Client> = api_client.clone();

    let mut tick = interval(Duration::from_secs(system_conf.get_uptime_interval()));
    loop {
        system::send_uptime(system_conf.clone(), system_api_client.clone()).await;
        tick.tick().await;
    }
}
