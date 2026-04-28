use serde_json::Value;
use std::sync::Arc;

pub(super) enum __MessageBody {
    Node(__MessageNode),
    Service(__MessageService),
}

impl __MessageBody {
    pub fn get_message(&self) -> String {
        match self {
            __MessageBody::Node(n) => n.msg.clone(),
            __MessageBody::Service(s) => s.msg.clone(),
        }
    }

    pub fn get_node_name(&self) -> &str {
        match self {
            __MessageBody::Node(n) => &n.name,
            __MessageBody::Service(s) => &s.node_name,
        }
    }

    pub fn get_service_name(&self) -> &str {
        match self {
            __MessageBody::Service(s) => &s.service_name,
            _ => "<Not Found>",
        }
    }

    pub fn get_unique_id(&self)->i64{
        match self {
            __MessageBody::Node(n) => n.unique_id,
            __MessageBody::Service(s) => s.unique_id,
        }
    }
}

pub(super) struct __MessageNode {
    pub unique_id: i64,
    pub name: String,
    pub msg: String,
}

pub(super) struct __MessageService {
    pub unique_id: i64,
    pub service_name: String,
    pub node_name: String,
    pub msg: String,
}

pub(super) struct __MailMessageBody {
    pub destination: Vec<String>,
    pub msg: Arc<String>,
    pub subject: String,
}

pub(super) struct __WebHookMEssageBody {
    pub destination: Vec<String>,
    pub value: Value,
}

