use std::{sync::Arc};

pub(super) struct __MessageBody {
    pub destination: Vec<String>,
    pub msg: Arc<String>,
    pub subject: String,
}
