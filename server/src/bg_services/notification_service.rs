use super::notification_medium::{mail_sender_worker, webhook_sender_worker};
use super::structures::{
    __MailMessageBody, __MessageBody, __MessageNode, __MessageService, __WebHookMEssageBody,
};
use ferroscope_server::global::structure::{
    BGRulesData, EventType, NotificationChannel, NotificationData,
};
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn notification_service(
    pg_pool: sqlx::Pool<sqlx::Postgres>,
    receiver: mpsc::Receiver<NotificationData>,
) {
    let (email_rx, email_tx) = mpsc::channel::<__MailMessageBody>(10);
    let (webhook_rx, webhook_tx) = mpsc::channel::<__WebHookMEssageBody>(10);

    tokio::spawn(async move { mail_sender_worker(email_tx).await });
    tokio::spawn(async move { webhook_sender_worker(webhook_tx).await });
    notifier_worker(pg_pool, email_rx, webhook_rx, receiver).await;
}

async fn notifier_worker(
    pg_pool: sqlx::Pool<sqlx::Postgres>,
    mail_sender: mpsc::Sender<__MailMessageBody>,
    webhook_sender: mpsc::Sender<__WebHookMEssageBody>,
    mut receiver: mpsc::Receiver<NotificationData>,
) {
    tokio::spawn(async move {
        while let Some(msg) = receiver.recv().await {
            let data:Vec<BGRulesData>=
                sqlx::query_as("select name,condition_json,action_json from rules where  is_active=TRUE AND event_type=$1 ")
                .bind(msg.get_event_type())
                .fetch_all(&pg_pool).await.unwrap();

            match msg.category {
                EventType::NODE | EventType::SERVICE => {
                    service_node_notifier(msg, &pg_pool, &mail_sender, &webhook_sender, data).await
                }
                _ => println!("Not Ready for other types"),
            };
        } //end while of mpsc recv
    }); //end tokio::spwn
}

async fn service_node_notifier(
    msg: NotificationData,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    mail_sender: &mpsc::Sender<__MailMessageBody>,
    webhook_sender: &mpsc::Sender<__WebHookMEssageBody>,
    data: Vec<BGRulesData>,
) {
    let notifcation_meta_data = match msg.category {
        EventType::NODE => {
            let data = sqlx::query("select name from nodes where id=$1")
                .bind(msg.unique_id)
                .fetch_optional(pg_pool)
                .await
                .unwrap();
            let node_name = data
                .map(|row| row.get("name"))
                .unwrap_or_else(|| String::from("<Node name not found>"));
            __MessageBody::Node(__MessageNode {
                unique_id: msg.unique_id,
                msg: format!("{}: Node is down", node_name),
                name: node_name,
            })
        }
        EventType::SERVICE => {
            let data=sqlx::query("SELECT sm.service_name,n.name as node_name FROM service_monitor sm JOIN nodes n on n.id=sm.node_id WHERE sm.id=$1")
            .bind(msg.unique_id)
            .fetch_optional(pg_pool).await.unwrap();
            let service_name_node_name = data
                .map(|row| (row.get("service_name"), row.get("node_name")))
                .unwrap_or_else(|| (String::from("<Node name not found>"), String::new()));
            __MessageBody::Service(__MessageService {
                unique_id: msg.unique_id,
                msg: format!(
                    "Service: {} of node: {} is down",
                    service_name_node_name.0, service_name_node_name.1
                ),
                node_name: service_name_node_name.1,
                service_name: service_name_node_name.0,
            })
        }
        _ => return,
    };

    let notification_message = Arc::new(notifcation_meta_data.get_message());
    for i in data {
        let action = i.action_json.0;

        match action.channel {
            NotificationChannel::Email => {
                let _ = mail_sender
                    .send(__MailMessageBody {
                        destination: action.to.clone(),
                        msg: notification_message.clone(),
                        subject: "Ferroscope Notification".to_string(),
                    })
                    .await;
            }
            NotificationChannel::Webhook => {
                if matches!(notifcation_meta_data, __MessageBody::Node(_)) {
                    let _ = webhook_sender
                        .send(__WebHookMEssageBody {
                            destination: action.to,
                            value: serde_json::json!({
                                "unique_id":notifcation_meta_data.get_unique_id(),
                                "msg": notification_message.clone(),
                                "name":notifcation_meta_data.get_node_name(),
                            }),
                        })
                        .await;
                } else {
                    //service down
                    let _ = webhook_sender
                        .send(__WebHookMEssageBody {
                            destination: action.to,
                            value: serde_json::json!({
                                "unique_id":notifcation_meta_data.get_unique_id(),
                                "msg": notification_message.clone(),
                                "node_name":notifcation_meta_data.get_node_name(),
                                "Service_name":notifcation_meta_data.get_service_name()
                            }),
                        })
                        .await;
                }
                //endif
            }
        } //end match
    } //endfor
}
