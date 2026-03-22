// Web Executor
use super::structures:: Web;
use reqwest::Client;
use serde_json::json;
use std::sync::Arc;
use crate::set_up::BaseConFig;
use tokio::time::{Duration};
use super::config_reader::{file_name_list, load_config};
use super::structures::{BaseFormat};
use std::sync::LazyLock;
use std::env;
use rustls::pki_types::{ServerName};
use rustls::{ClientConfig};
use std::error::Error;
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use x509_parser::parse_x509_certificate;
use rustls::{ RootCertStore};
use rustls_native_certs::load_native_certs;
use crate::Payload;

static CONFDIR: LazyLock<String> =
    LazyLock::new(|| env::var("CONF_DIR").unwrap_or("/etc/ferroscope_agent".to_string()));


async fn web_check(web: &Web, client: &Client) -> (String, u16) {
    match client.head(web.get_url()).send().await {
        Ok(value) => ("Success".to_string(), value.status().as_u16()),
        Err(e) => {println!("\n web error {}\n",e);  (e.to_string(), 1)},
    }
}


pub(super) async fn web_runner(api_queue:tokio::sync::mpsc::Sender<Payload>, config: Arc<BaseConFig>) {
    let all_files = match file_name_list(&format!("{}/Web", *CONFDIR)).await {
        Ok(value) => value,
        Err(e) => {
            println!("no config founnd err: {}", e);
            Vec::new()
        }
    };
    let baseapi = config.get_service_url();

    let _client = Client::builder()
        .timeout(Duration::from_secs(5)) //get this value from config
        .build()
        .unwrap();
    for file in all_files {
        let a: config::Config = match load_config(file).await {
            Ok(value) => value,
            Err(e) => {
                println!("error is {}", e);
                continue;
            }
        };

        let value: Web = match a.try_deserialize() {
            Ok(value) => value,
            Err(e) => {
                println!("error is {}", e);
                continue;
            }
        };
        let (_, code) = web_check(&value, &_client).await;
        // ssl expiry
        let ssl_datetime=match  get_https_expiry(value.get_url()).await {
            Ok(e)=>Some(e),
            Err(e)=>{println!("error in ssl is {}",e) ;None}
        };


        let data=match value.match_status(code) {
            true=>BaseFormat {
                    service_name: value.name,
                    status: "up".to_string(),
                    error_msg: "".to_string(),
                    category: "Web".to_string(),
                    ssl_exp: ssl_datetime,
                },
            false=>BaseFormat {
                    service_name: value.name,
                    status: "down".to_string(),
                    error_msg: format!("The status code is {}", code),
                    category: "Web".to_string(),
                    ssl_exp: ssl_datetime,
                }
            
        };
        let _=api_queue.send(Payload { endpoint: baseapi.clone(), body: json!(data) }).await;
    } //endfor
}




async fn get_https_expiry(domain: &str) -> Result<time::OffsetDateTime, Box<dyn Error>> {
      
    let ssl_domain = domain
    .strip_prefix("https://")
    .ok_or("don't have https")?.trim_end_matches("/");


    println!("the domain is {}",ssl_domain);

    let mut root_store = RootCertStore::empty();

    for cert in load_native_certs()? {
    root_store.add(cert)?;
    }

    let config = ClientConfig::builder()
        // .dangerous()
        // .with_custom_certificate_verifier(Arc::new(NoVerifier))
        .with_root_certificates(Arc::new(root_store))
        .with_no_client_auth();
    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect((ssl_domain, 443)).await?;
    let server_name = ServerName::try_from(ssl_domain.to_string())?;
    let tls_stream = connector.connect(server_name, stream).await?;
    let certs = tls_stream
        .get_ref()
        .1
        .peer_certificates()
        .ok_or("No certificate found")?;
    let cert = &certs[0];
    let (_, parsed_cert) = parse_x509_certificate(cert.as_ref())?;
    Ok(parsed_cert.validity().not_after.to_datetime())
}




