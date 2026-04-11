use serde::{Deserialize,Serialize};

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
pub(super) struct IdQuery {//being used for nodeid service id or getting anyother types of id
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

#[derive(Serialize,Debug,Deserialize)]
enum ConditionField {
    Status,//node status
    Value,//check certain values
}

#[derive(Serialize,Deserialize,Debug)]
pub(super) struct Condition {
    field: ConditionField,
    operator: String,
    value: i32,//0 for down 1 for up
}

#[derive(Serialize,Deserialize,Debug)]
enum NotificationChannel{
    Webhook,
    Email
}

#[derive(Deserialize,Debug)]
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



#[derive(Deserialize,Debug,Serialize)]
pub(super)  struct Notify{
    channel:NotificationChannel,
    to:Vec<String>,
    message:String
}


#[derive(Deserialize,Debug)]
pub (super) struct RulesData {
    pub name:String,
    pub active:bool,
    pub condition:Condition,
    pub event_type:EventType,
    pub action:Notify

}