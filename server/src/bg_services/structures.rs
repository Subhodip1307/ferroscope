use std::{sync::Arc};

pub(super) struct EmailBody {
    pub email_to: Vec<String>,
    pub msg: Arc<String>,
    pub subject: String,
}
