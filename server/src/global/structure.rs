use serde::{Deserialize,Serialize};
use sqlx::types::Json;

pub struct NotificationData {
    pub category: String,
    pub sujbect: String,
    pub unique_id: String,
}
impl NotificationData {
    pub fn get_message(&self) -> String {
        if self.category == "NODE" {
            return format!("Node Offline {}", self.unique_id);
        } else if self.category == "SERVICE" {
            return format!("Service Offline {}", self.unique_id); //need to imporve
        }
        "".to_string()
    }
}



#[derive(Serialize,Debug,Deserialize)]
enum ConditionField {
    Status,//node status
    Value,//check certain values
}

#[derive(Serialize,Deserialize,Debug,sqlx::FromRow)]
pub struct Condition {
    field: Json<ConditionField>,
    operator: String,
    value: i32,//0 for down 1 for up
}

#[derive(Serialize,Deserialize,Debug)]
pub enum NotificationChannel{
    Webhook,
    Email
}




#[derive(Deserialize,Debug,Serialize,sqlx::FromRow)]
pub struct BGNotify{
    pub channel:NotificationChannel,
    to:Vec<String>,
    message:String
}


#[derive(Deserialize,Debug,sqlx::FromRow)]
pub  struct BGRulesData {
    pub name:String,
    pub condition_json:Json<Condition>,
    pub action_json:Json<BGNotify>

}