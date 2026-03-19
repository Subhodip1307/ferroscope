// Web Executor
use super::structures:: Web;
use reqwest::Client;
use std::sync::Arc;
use crate::set_up::BaseConFig;
use tokio::time::{Duration};
use super::config_reader::{file_name_list, load_config};
use super::structures::{BaseFormat};
use std::sync::LazyLock;
use std::env;
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{ClientConfig, DigitallySignedStruct, SignatureScheme};
use std::error::Error;
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use x509_parser::parse_x509_certificate;

static CONFDIR: LazyLock<String> =
    LazyLock::new(|| env::var("CONF_DIR").unwrap_or("/etc/ferroscope_agent".to_string()));


async fn web_check(web: &Web, client: &Client) -> (String, u16) {
    match client.head(web.get_url()).send().await {
        Ok(value) => ("Success".to_string(), value.status().as_u16()),
        Err(e) => {println!("\n web error {}\n",e);  (e.to_string(), 1)},
    }
}


pub(super) async fn web_runner(api_client: Arc<Client>, config: Arc<BaseConFig>) {
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
        let (status, code) = web_check(&value, &_client).await;
        // ssl expiry
        let ssl_datetime=match  get_https_expiry(value.get_url()).await {
            Ok(e)=>Some(e),
            Err(e)=>{println!("error in ssl is {}",e) ;None}
        };

        if status == "Success" && value.match_status(code) {
            let res = api_client
                .post(&baseapi)
                .json(&BaseFormat {
                    service_name: value.name,
                    status: "up".to_string(),
                    error_msg: "".to_string(),
                    category: "Web".to_string(),
                    ssl_exp: ssl_datetime,
                })
                .send()
                .await
                .unwrap();
            println!("the res is {:?}", res);
        } else {
            let res = api_client
                .post(&baseapi)
                .json(&BaseFormat {
                    service_name: value.name,
                    status: "down".to_string(),
                    error_msg: format!("The status code is {}", code),
                    category: "Web".to_string(),
                    ssl_exp: ssl_datetime,
                })
                .send()
                .await
                .unwrap();
            println!("the res is {:?}", res);
        }
    } //endfor
}

// ssl timing

#[derive(Debug)]
struct NoVerifier;
impl ServerCertVerifier for NoVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer,
        _intermediates: &[CertificateDer],
        _server_name: &ServerName,
        _ocsp: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }
    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }
    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }
    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        vec![
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::ED25519,
        ]
    }
}
async fn get_https_expiry(domain: &str) -> Result<time::OffsetDateTime, Box<dyn Error>> {
      
    let ssl_domain = domain
    .strip_prefix("https://")
    .ok_or("don't have https")?;


    let config = ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(NoVerifier))
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




