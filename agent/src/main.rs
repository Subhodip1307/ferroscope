use tokio::{time::{Duration, interval}};
mod system;
use std::{collections::HashMap};
use system::logic;
mod jobs;
use reqwest::{Client,header};
use std::sync::Arc;
use std::env;
use std::sync::LazyLock;
static BASEAPI:LazyLock<String>=LazyLock::new(||env::var("BASEAPI").unwrap_or("http://localhost:8000".to_string()));

#[tokio::main]
async fn main() {
    let mut __headers=header::HeaderMap::new();
    __headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(
        env::var("AUTH").unwrap().as_str()
    ).expect("something went wrong in Header"));
    let api_client=Arc::new(
        Client::builder().default_headers(__headers).timeout(std::time::Duration::from_secs(5)).build().expect("errr")
    );
    // sening the systeminfo first
    {
    let sys=logic::systeminfo();
    let send_data=format!("{}/send_systeminfo",*BASEAPI);
    let _=api_client.post(send_data)
    .json(&sys)
    .send().await.unwrap();
    // println!("{:?}",a);
    }

    let jobs_api=Arc::clone(&api_client);
    tokio::spawn(async move {
        jobs::executor::run(jobs_api).await  
    });
    
    let mut ticker=interval(Duration::from_secs(60));
    loop {
    // sening cpu usage
    {
    let send_data=format!("{}/send_cpu",*BASEAPI);
    let  cpu_usage: HashMap<&str, f64>=HashMap::from([
        ("cpu",(logic::total_cpu_usage().unwrap() * 100.0).round())
    ]);
    let _= api_client.post(send_data)
    .json(&cpu_usage)
    .send().await.unwrap();
    // println!("{:?}",a);
    }
    // uptime
    {
    let  uptime: HashMap<&str, u64>=HashMap::from([
        ("uptime_sec",logic::get_uptime().unwrap())
    ]);
    let send_data=format!("{}/send_uptime",*BASEAPI);

     reqwest::Client::new() 
    .post(send_data)
    .form(&uptime)
    .send()
    .await.unwrap();    
    }

    // send memory
    {
    let send_data=format!("{}/send_memory",*BASEAPI);
    let memory=logic::memory_usage().unwrap();
    memory.get_total();
    let  data: HashMap<&str, &str>=HashMap::from([
        ("free",memory.get_free()),
        ("total",memory.get_total()),
    ]);
    let _= api_client.post(send_data)
    .json(&data)
    .send().await.unwrap(); 
    // println!("Memory Priented {:?}",memory);
            // println!("{:?}",a);
    }
    ticker.tick().await;
}

}
