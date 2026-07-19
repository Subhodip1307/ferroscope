// shared types
use serde::{Serialize,Deserialize};

#[derive(Clone, Serialize,Deserialize)]
pub struct AuthUser {
    pub user_id: i64,
}