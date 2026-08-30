use ferroscope_server::global::structure::{Condition, NotificationChannel};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use super::types::Metrixs;

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
pub(super) struct MutiIdQuery {
    pub obj_ids: Vec<i64>,
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

#[derive(Deserialize)]
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

#[derive(Deserialize, Serialize)]
pub struct Notify {
    channel: Json<NotificationChannel>,
    to: Vec<String>,
    message: String,
}

#[derive(Deserialize)]
pub struct RulesData {
    pub name: String,
    pub active: bool,
    pub condition: Json<Condition>,
    pub event_type: Json<EventType>,
    pub action: Json<Notify>,
}

#[derive(Deserialize)]
pub struct UserDetailsEdit {
    pub id: i64,
    pub username: String,
    pub is_admin: bool,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct UserDetails {
    pub username: String,
    pub is_admin: bool,
    pub email: Option<String>,
    pub password: String,
}



#[derive(Deserialize,Debug)]
pub struct UserPermissions{
    pub node_id:i64,
    pub metrix:Option<Vec<Metrixs>>, //Allowed Metrixs 
    pub services:Option<Vec<i64>>, //Services id list
    pub full_permission:Option<bool>
}

#[derive(Deserialize,Debug)]
pub struct AssignPermission{//it's a payload
    pub user_id:i64,
    pub nodes_permissions:Vec<UserPermissions>
}
