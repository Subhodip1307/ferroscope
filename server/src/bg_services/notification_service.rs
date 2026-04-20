use ferroscope_server::global::structure::{BGRulesData, NotificationChannel, NotificationData,EventType};
use sqlx::Row;
use std::{sync::Arc};
use tokio::sync::mpsc;
use super::structures::EmailBody;
use super::notification_medium::mail_sender_worker;

// there will be two worker one for mailsending (any) and one for webhook
// there will only one worker who will collect differnt reporting and notify user according the set up rules
// there will be different worker for each type of event RAM CPU NODE SERVICES etc.

pub async fn notification_service(
    pg_pool: sqlx::Pool<sqlx::Postgres>,
    receiver: mpsc::Receiver<NotificationData>,
) {
    let (rx, tx) = mpsc::channel::<EmailBody>(10);

    tokio::spawn(async move {
        mail_sender_worker(tx).await;
    });

    notifier_worker(pg_pool, rx, receiver).await;
}

async fn notifier_worker(
    pg_pool: sqlx::Pool<sqlx::Postgres>,
    mail_sender: mpsc::Sender<EmailBody>,
    mut receiver: mpsc::Receiver<NotificationData>,
) {
    tokio::spawn(async move {
        while let Some(msg) = receiver.recv().await {
            
            match msg.category {
                EventType::NODE | EventType::SERVICE =>service_node_notifier(msg,&pg_pool,&mail_sender).await,
                _=>println!("Not Ready for other types"),
            };
        } //end while of mpsc recv
    }); //end tokio::spwn
}

async fn service_node_notifier(
    msg: NotificationData,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    mail_sender: &mpsc::Sender<EmailBody>,
) {
    let data:Vec<BGRulesData>=
        sqlx::query_as("select name,condition_json,action_json from rules where  is_active=TRUE AND event_type=$1 ")
        .bind(msg.get_event_type())
        .fetch_all(pg_pool).await.unwrap();

    let notifcation_message=match msg.category {
        EventType::NODE=>{
            let data=sqlx::query("select name from nodes where id=$1")
            .bind(msg.unique_id)
            .fetch_optional(pg_pool).await.unwrap();
            let node_name=data.map(|row|row.get("name")).unwrap_or_else(||String::from("<Node name not found>"));
            Arc::new(format!("{}: Node is down",node_name))
            
        },
        EventType::SERVICE=>{
             let data=sqlx::query("select service_name from service_monitor where id=$1")
            .bind(msg.unique_id)
            .fetch_optional(pg_pool).await.unwrap();
            let service_name=data.map(|row|row.get("service_name")).unwrap_or_else(||String::from("<Node name not found>"));
            Arc::new(format!("{}: Service  is down",service_name))
        },
        _=>{Arc::new(String::from("<Something Went Wrong>"))}
    };

    println!("{}",notifcation_message);

    for i in data {
        let action = i.action_json.0;
        if matches!(action.channel, NotificationChannel::Email) {
            println!("Sending mail");
            let _ = mail_sender
                .send(EmailBody {
                    email_to: action.to,
                    msg: notifcation_message.clone(),
                    subject: "Ferroscope Notification".to_string(),
                })
                .await;
        }
        if matches!(action.channel, NotificationChannel::Webhook) {
            println!("Webhook");
        }
    } //endfor

}