use tokio::{time::{Duration, interval}};
mod system;
use std::{collections::HashMap};
use system::logic;
const BASEAPI:&str="http://localhost:8000";

mod jobs;
use reqwest::{Client,header};
use std::sync::Arc;


#[tokio::main]
async fn main() {
    let mut __headers=header::HeaderMap::new();
    __headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static("ABCD"));
    let api_client=Arc::new(
        Client::builder().default_headers(__headers).timeout(std::time::Duration::from_secs(5)).build().expect("errr")
    );
    // sening the systeminfo first
    {
    let sys=logic::systeminfo();
    let send_data=format!("{}/send_systeminfo",BASEAPI);
    let a=api_client.post(send_data)
    .json(&sys)
    .send().await.unwrap();
    println!("{:?}",a);
    }

    let jobs_api=Arc::clone(&api_client);
    tokio::spawn(async move {
        jobs::executor::run(jobs_api).await  
    });
    
    let mut ticker=interval(Duration::from_secs(10));
    loop {
    // sening cpu usage
    {
    let send_data=format!("{}/send_cpu",BASEAPI);
    let  cpu_usage: HashMap<&str, f64>=HashMap::from([
        ("cpu",(logic::total_cpu_usage().unwrap() * 100.0).round())
    ]);
    let a= api_client.post(send_data)
    .json(&cpu_usage)
    .send().await.unwrap();
    println!("{:?}",a);
    }
    // uptime
    {
    let  uptime: HashMap<&str, u64>=HashMap::from([
        ("uptime",logic::get_uptime().unwrap())
    ]);
     reqwest::Client::new() 
    .post(BASEAPI)
    .form(&uptime)
    .send()
    .await.unwrap();    
    }

    // send memory
    {
    let send_data=format!("{}/send_memory",BASEAPI);
    let memory=logic::memory_usage().unwrap();
    memory.get_total();
    let  data: HashMap<&str, &str>=HashMap::from([
        ("free",memory.get_free()),
        ("total",memory.get_total()),
    ]);
    let a= api_client.post(send_data)
    .json(&data)
    .send().await.unwrap(); 
    println!("Memory Priented {:?}",memory);
            println!("{:?}",a);
    }
    ticker.tick().await;
}

}
