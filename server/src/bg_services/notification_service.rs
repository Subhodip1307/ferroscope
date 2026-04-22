use ferroscope_server::global::structure::{BGRulesData, NotificationChannel, NotificationData,EventType};
use sqlx::Row;
use std::{sync::Arc};
use tokio::sync::mpsc;
use super::structures::__MessageBody;
use super::notification_medium::{mail_sender_worker,webhook_sender_worker};

// there will be two worker one for mailsending (any) and one for webhook
// there will only one worker who will collect differnt reporting and notify user according the set up rules
// there will be different worker for each type of event RAM CPU etc.

pub async fn notification_service(
    pg_pool: sqlx::Pool<sqlx::Postgres>,
    receiver: mpsc::Receiver<NotificationData>,
) {
    let (email_rx, email_tx) = mpsc::channel::<__MessageBody>(10);
    let (webhook_rx, webhook_tx) = mpsc::channel::<__MessageBody>(10);

    tokio::spawn(async move {
        tokio::join!(mail_sender_worker(email_tx),webhook_sender_worker(webhook_tx))
    });

    notifier_worker(pg_pool, email_rx, webhook_rx,receiver).await;
}

async fn notifier_worker(
    pg_pool: sqlx::Pool<sqlx::Postgres>,
    mail_sender: mpsc::Sender<__MessageBody>,
    webhook_sender: mpsc::Sender<__MessageBody>,
    mut receiver: mpsc::Receiver<NotificationData>,
) {
    tokio::spawn(async move {
        while let Some(msg) = receiver.recv().await {
            
            match msg.category {
                EventType::NODE | EventType::SERVICE =>service_node_notifier(msg,&pg_pool,&mail_sender,&webhook_sender).await,
                _=>println!("Not Ready for other types"),
            };
        } //end while of mpsc recv
    }); //end tokio::spwn
}

async fn service_node_notifier(
    msg: NotificationData,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    mail_sender: &mpsc::Sender<__MessageBody>,
    webhook_sender: &mpsc::Sender<__MessageBody>
    
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
             let data=sqlx::query("SELECT sm.service_name,n.name as node_name FROM service_monitor sm JOIN nodes n on n.id=sm.node_id WHERE sm.id=$1")
            .bind(msg.unique_id)
            .fetch_optional(pg_pool).await.unwrap();
            let service_name=data.map(|row|(row.get("service_name"),row.get("node_name"))).unwrap_or_else(||(String::from("<Node name not found>"),String::new()));
            Arc::new(format!("Service: {} of node: {} is down",service_name.0,service_name.1))
        },
        _=>{Arc::new(String::from("<Something Went Wrong>"))}
    };

    println!("{}",notifcation_message);

    for i in data {
        let action = i.action_json.0;
        if matches!(action.channel, NotificationChannel::Email) {
            println!("Sending mail");
            let _ = mail_sender
                .send(__MessageBody {
                    destination: action.to.clone(),
                    msg: notifcation_message.clone(),
                    subject: "Ferroscope Notification".to_string(),
                })
                .await;
        }
        if matches!(action.channel, NotificationChannel::Webhook) {
            let _=webhook_sender.send(
                __MessageBody {
                    destination: action.to,
                    msg: notifcation_message.clone(),
                    subject: "Ferroscope Notification".to_string(),
                }
            ).await;
        }
    } //endfor

}