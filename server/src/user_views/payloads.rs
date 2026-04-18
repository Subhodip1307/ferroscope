use ferroscope_server::global::structure::{Condition, NotificationChannel};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

#[derive(Deserialize)]
pub(super) struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub(super) struct UsernamePasswordReset {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub(super) struct IdQuery {
    //being used for nodeid service id or getting anyother types of id
    pub node: i64,
}

#[derive(Deserialize)]
pub(super) struct ServiceQuery {
    // use to query the node and a specific service of it.
    pub node: i64,
    pub service_name: String,
}

#[derive(Deserialize)]
pub(super) struct CreateNode {
    pub name: String,
}

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

#[derive(Deserialize, Debug, Serialize)]
pub struct Notify {
    channel: Json<NotificationChannel>,
    to: Vec<String>,
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct RulesData {
    pub name: String,
    pub active: bool,
    pub condition: Json<Condition>,
    pub event_type: Json<EventType>,
    pub action: Json<Notify>,
}
