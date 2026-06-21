mod jobs;
mod system;
use reqwest::{Client, header};
use std::sync::Arc;
mod set_up;
use tokio::sync::mpsc;
mod tasks;

#[derive(Debug)]
struct Payload {
    endpoint: String, //make it arc<String>
    body: serde_json::Value,
}

#[tokio::main]
async fn main() {
    println!("Runing Version : {}", env!("CARGO_PKG_VERSION"));
    // set-up
    let conf = {
        let service_setup = set_up::ConfSetUp::new();
        service_setup.set_up().await;
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
        tasks::queue(jobs_api, rx).await;
    });

    // spawn other task
    tasks::spawn_info_sender(conf, tx, api_client).await;
}
