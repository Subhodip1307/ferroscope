use crate::AppState;
use crate::user_views::LatestCpu;
use crate::user_views::LatestRam;
use chrono::Utc;
use ferroscope_server::global::structure::{NotificationData,BGRulesData,NotificationChannel};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};
use std::env;
use tokio::sync::mpsc;
use tokio::time::{Duration, interval};


// there will be two worker one for mailsending (any) and one for webhook
// there will only one worker who will collect differnt reporting and notify user according the set up rules


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
                if let Some(cpu_listender) = app_data.cpu_strem.get(cpu_ket) {
                    let _ = cpu_listender.send(LatestCpu {
                        value: -100.0,
                        date_time: Utc::now(),
                    });
                    // removes the key also
                    drop(cpu_listender); //dropping the lock
                    app_data.cpu_strem.remove(cpu_ket);
                };
                // remove the ram stream and cpu stream
                let ram_key = &format!("node_ram_strem_{}", key);
                if let Some(ram_listender) = app_data.ram_strem.get(ram_key) {
                    let _ = ram_listender.send(LatestRam {
                        free: String::from("STOP"),
                        total: String::new(),
                        timestamp: Utc::now(),
                    });
                    drop(ram_listender);
                    app_data.ram_strem.remove(ram_key);
                };
                //  Send Notification
                let _ = app_state
                    .notifier
                    .send(NotificationData {
                        category: "NODE".to_string(),
                        sujbect: "Node Status unreachable".to_string(),
                        unique_id: format!("{}", key),
                    })
                    .await;
            } //end for
        }
    });
}


pub async fn notifier_worker(pg_pool:sqlx::Pool<sqlx::Postgres>){
    tokio::spawn(async move{
        let data:Vec<BGRulesData>=
        sqlx::query_as("select condition_json,action_json where  is_active=TRUE AND event_type='NODE' ")
        .fetch_all(&pg_pool).await.unwrap();
        for i in data{
           let action = i.action_json.0;
            match  action.channel {
                NotificationChannel::Email=>println!("email"),
                NotificationChannel::Webhook=>println!("Webhook"),
            }
        }
    });
}


pub async fn mail_sender_worker(
    mut receiver: mpsc::Receiver<NotificationData>,
) {
    tokio::spawn(async move {
        let username = env::var("EMAIL_HOST_USER");
        let password = env::var("EMAIL_HOST_PASSWORD");
        let smtp_server = env::var("EMAIL_HOST");
        let dev_mail = env::var("DEVMAIL");
        let (user, pass, smtp_server, dev) = match (username, password, smtp_server, dev_mail) {
            (Ok(v1), Ok(v2), Ok(v3), Ok(v4)) => (v1, v2, v3, v4),
            _ => {
                print!("EMAIL Set-up  Incomplete");
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
            let email = Message::builder()
            .from(form.parse().unwrap())
            .to(dev.parse().unwrap())
            .subject(&msg.sujbect)
            .body(msg.get_message())
            .unwrap();
            let _ = mailer.send(email).await;
            println!("Done");
        }
    });
}

