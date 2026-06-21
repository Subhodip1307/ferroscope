// shared types
use serde::{Serialize};

#[derive(Clone, Serialize)]
pub struct AuthUser {
    pub user_id: i64,
}