use std::env;

use tokio::sync::mpsc;

use crate::AppState;
use crate::user_views::LatestCpu;
use crate::user_views::LatestRam;
use chrono::Utc;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};
use tokio::time::{Duration, interval};
use ferroscope_server::global::structure::NotificationData;

pub async fn node_status_check(app_state: AppState) {
    // runing backgrond services
    tokio::spawn(async move {
        let app_data = app_state.clone();

        #[cfg(debug_assertions)]
        let timeout = 10_000; //test
        #[cfg(not(debug_assertions))]
        let timeout = 30_000; //production

        let mut tick = interval(Duration::from_secs(30));
        loop {
            tick.tick().await;
            let mut key_vec: Vec<i64> = Vec::with_capacity(app_data.helth_check.len());
            for entry in app_data.helth_check.iter() {
                let key = entry.key();
                let value = *entry.value();

                let current = ferroscope_server::global::utils_functions::current_time();

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
            //  Send Notification
            let _=app_state.notifier.send(NotificationData{category:"NODE".to_string(),message:format!("Node Offline {}",key)}).await;

        
            } //end for
        }
    });
}

pub async fn send_notification_mail(mut receiver: mpsc::Receiver<ferroscope_server::global::structure::NotificationData>) {
    tokio::spawn(async move {
        let username = env::var("EMAIL_HOST_USER");
        let password = env::var("EMAIL_HOST_PASSWORD");
        let smtp_server = env::var("EMAIL_HOST");
        let dev_mail = env::var("DEVMAIL");
        let (user, pass,smtp_server,dev) = match (username, password,smtp_server,dev_mail) {
            (Ok(v1), Ok(v2),Ok(v3),Ok(v4)) => {
                (v1, v2,v3,v4)
            }
            _ => {
                print!("EMAIL Set-up  Incomplete");
                return;
            }
        };
        let form=&format!("Ferroscope <{}>",user);
        let creds = Credentials::new(user, pass);

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_server)
            .unwrap()
            .credentials(creds)
            .pool_config(lettre::transport::smtp::PoolConfig::new().max_size(1))
            .build();

        while let Some(msg) = receiver.recv().await {
            println!("sending message ");
            if msg.category == "NODE" {
                let email=node_message(form,msg.message,&dev);
                let _ = mailer.send(email).await;
                println!("Done");
            }else{
                println!("unknown command {}",msg.category)
            }
            
        }
    });
}



fn node_message(form:&str,msg:String,to_user:&str)->lettre::Message{
     Message::builder()
        .from(form.parse().unwrap())
        .to(to_user.parse().unwrap())
        .subject("Node Status unreachable")
        .body(msg)
        .unwrap()
}