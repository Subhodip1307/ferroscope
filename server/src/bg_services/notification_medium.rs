use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};
use std::{env};
use tokio::sync::mpsc;
use super::structures::__MessageBody;
use reqwest::{Client};
use std::collections::HashMap;



pub(super) async fn mail_sender_worker(mut receiver: mpsc::Receiver<__MessageBody>) {
    let username = env::var("EMAIL_HOST_USER");
    let password = env::var("EMAIL_HOST_PASSWORD");
    let smtp_server = env::var("EMAIL_HOST");
    let (user, pass, smtp_server) = match (username, password, smtp_server) {
        (Ok(v1), Ok(v2), Ok(v3)) => (v1, v2, v3),
        _ => {
            println!("EMAIL Set-up  Incomplete");
            return;
        }
    };
    let form = &format!("Ferroscope <{}>", user);
    let creds = Credentials::new(user, pass);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_server)
        .unwrap()
        .credentials(creds)
        .pool_config(lettre::transport::smtp::PoolConfig::new().max_size(1))
        .build();

    while let Some(msg) = receiver.recv().await {
        for recipient in msg.destination {
            let email = Message::builder()
                .from(form.parse().unwrap())
                .to(recipient.parse().unwrap())
                .subject(&msg.subject)
                .body((*msg.msg).clone())
                .unwrap();
            let _ = mailer.send(email).await;
            println!("Done");
        }
    }
}



pub(super) async fn webhook_sender_worker(mut receiver: mpsc::Receiver<__MessageBody>){
    let api_client = 
        Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("errr");
    while let Some(msg) = receiver.recv().await {
        let payload=HashMap::from([("Message",msg.msg)]);
        for recipient in msg.destination {
           if api_client.post(&recipient).json(&payload).send().await.is_err(){
            println!("WebHook API issue: {}",recipient);
           };   
        }
    }
    
}