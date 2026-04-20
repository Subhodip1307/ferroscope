use serde::{Deserialize, Serialize};
use sqlx::types::Json;

#[derive(Deserialize, Debug)]
pub enum EventType {
    CPU,
    RAM,
    SERVICE,
    NODE,
}
impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::CPU => write!(f, "CPU"),
            EventType::RAM => write!(f, "RAM"),
            EventType::SERVICE => write!(f, "SERVICE"),
            EventType::NODE => write!(f, "NODE"),
        }
    }
}

pub struct NotificationData {
    pub category: EventType,
    pub sujbect: String,
    pub unique_id: i64,
}
impl NotificationData {
    pub fn get_message(&self) -> String {
        match self.category {
            EventType::NODE => format!("Node Offline {}", self.unique_id),
            EventType::SERVICE => format!("Service Offline {}", self.unique_id),
            _ => "".to_string(),
        }
    }

    pub fn get_event_type(&self) -> String {
        self.category.to_string()
    }
}

#[derive(Serialize, Debug, Deserialize)]
enum ConditionField {
    Status, //node status
    Value,  //check certain values
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Condition {
    field: Json<ConditionField>,
    operator: String,
    value: i32, //0 for down 1 for up
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NotificationChannel {
    Webhook,
    Email,
}

#[derive(Deserialize, Debug, Serialize, sqlx::FromRow)]
pub struct BGNotify {
    pub channel: NotificationChannel,
    pub to: Vec<String>,
    pub message: String,
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct BGRulesData {
    pub name: String,
    pub condition_json: Json<Condition>,
    pub action_json: Json<BGNotify>,
}
