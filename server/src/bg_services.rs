use std::env;

use tokio::sync::mpsc;

use crate::AppState;
use crate::user_views::LatestCpu;
use crate::user_views::LatestRam;
use chrono::Utc;
use tokio::time::{Duration, interval};
use lettre::{
    message::Mailbox,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};



pub async fn node_status_check(app_state: AppState) {
    // runing backgrond services
    tokio::spawn(async move {
        let app_data = app_state.clone();
        let timeout = 30_000;
        let mut tick = interval(Duration::from_secs(30));
        loop {
            tick.tick().await;
            let mut key_vec: Vec<i64> = Vec::with_capacity(app_data.helth_check.len());
            for entry in app_data.helth_check.iter() {
                let key = entry.key();
                let value = *entry.value();

                let current = ferroscope_server::current_time();

                if current - value > timeout {
                    println!("helth check failed");
                    key_vec.push(*key);
                }
            } //end for remove dashmap lock

            for key in &key_vec {
                // remove the key
                app_data.helth_check.remove(key);

                let cpu_ket = &format!("node_cpu_strem_{}", key);
                match app_data.cpu_strem.get(cpu_ket) {
                    Some(cpu_listender) => {
                        let _ = cpu_listender.send(LatestCpu {
                            value: -100.0,
                            date_time: Utc::now(),
                        });
                        // removes the key also
                        drop(cpu_listender); //dropping the lock
                        app_data.cpu_strem.remove(cpu_ket);
                    }
                    None => {}
                };
                // remove the ram stream and cpu stream
                let ram_key = &format!("node_ram_strem_{}", key);
                match app_data.ram_strem.get(ram_key) {
                    Some(ram_listender) => {
                        let _ = ram_listender.send(LatestRam {
                            free: String::from("STOP"),
                            total: String::new(),
                            timestamp: Utc::now(),
                        });
                        drop(ram_listender);
                        app_data.ram_strem.remove(ram_key);
                    }
                    None => {}
                };
            } //end
        }
    });
}


pub async fn send_notification_mail(mut receiver:mpsc::Receiver<String>){
    println!("runing mee");
    tokio::spawn(async move {
             let username=env::var("EmailUser");
            let password=env::var("EmailPassword");
            let (user,pass)=  match (username,password)  {
                (Ok(v1),Ok(v2))=>{print!("EMAIL Usere & Password Found"); (v1,v2)},
                _=>{print!("EMAIL Usere & Password not Found"); return ; }
            };

            let creds = Credentials::new(
            user,
            pass,
        );

        let mailer= AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .pool_config(
                lettre::transport::smtp::PoolConfig::new()
                    .max_size(1)
            )
            .build();
        
        while let Some(msg)=receiver.recv().await {
            let email = Message::builder()
            .from("Your Name <your@gmail.com>".parse().unwrap())
            .to(msg.parse().unwrap())
            .subject("Hello")
            .body(msg)
            .unwrap();
            let _ = mailer.send(email).await;
            
        }

    });


}