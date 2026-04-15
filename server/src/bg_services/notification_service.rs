use ferroscope_server::global::structure::{NotificationData,BGRulesData,NotificationChannel,EventType};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};
use std::env;
use tokio::sync::mpsc;


struct EmailBody{
    email_to:Vec<String>,
    msg:String,
    subject:String
}


// there will be two worker one for mailsending (any) and one for webhook
// there will only one worker who will collect differnt reporting and notify user according the set up rules
// there will be different worker for each type of event RAM CPU NODE SERVICES etc.

pub async fn notification_service(pg_pool:sqlx::Pool<sqlx::Postgres>,receiver: mpsc::Receiver<NotificationData>){
    let (rx,tx)=mpsc::channel::<EmailBody>(10);
    
    tokio::spawn(async move {
        mail_sender_worker(tx).await;
    });

    node_notifier_worker(pg_pool,rx,receiver).await;

}






async fn node_notifier_worker(pg_pool:sqlx::Pool<sqlx::Postgres>,mail_sender:mpsc::Sender<EmailBody> ,mut receiver: mpsc::Receiver<NotificationData>){
    tokio::spawn(async move{
        while let Some(msg) = receiver.recv().await {

            let data:Vec<BGRulesData>=
            sqlx::query_as("select condition_json,action_json where  is_active=TRUE AND event_type=$1 ")
            .bind(msg.get_event_type())
            .fetch_all(&pg_pool).await.unwrap();
            for i in data{
            let action = i.action_json.0;
                match  action.channel {
                    NotificationChannel::Email=>{
                       let _= mail_sender.send(EmailBody { email_to: action.to, msg: action.message, subject: "".to_string() }).await;
                    }
                    NotificationChannel::Webhook=>println!("Webhook"),
                }
            }//endfor

        }//end while of mpsc recv
    });//end tokio::spwn
}


async fn mail_sender_worker(
    mut receiver: mpsc::Receiver<EmailBody>,
) {
        let username = env::var("EMAIL_HOST_USER");
        let password = env::var("EMAIL_HOST_PASSWORD");
        let smtp_server = env::var("EMAIL_HOST");
        let (user, pass, smtp_server) = match (username, password, smtp_server) {
            (Ok(v1), Ok(v2), Ok(v3)) => (v1, v2, v3),
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
            for recipient in msg.email_to {
                let email = Message::builder()
                .from(form.parse().unwrap())
                .to(recipient.parse().unwrap())
                .subject(&msg.subject)
                .body(msg.msg.clone())
                .unwrap();
                let _ = mailer.send(email).await;
                println!("Done");
            }
        }
}