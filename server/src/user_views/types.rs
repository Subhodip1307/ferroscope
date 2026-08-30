// shared types
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub user_id: i64,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
}
// no need warp with JSON
impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize)]
pub struct RespMessage {
    // to retrun hardcoded messages
    pub msg: &'static str,
    pub res: bool,
}
impl IntoResponse for RespMessage {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub enum Metrixs {
    RAM,
    CPU,
    DISK,
}
impl std::fmt::Display for Metrixs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Metrixs::RAM => write!(f, "RAM"),
            Metrixs::CPU => write!(f, "CPU"),
            Metrixs::DISK => write!(f, "DISK"),
        }
    }
}

#[derive(Debug)]
pub enum PermissionData<T> {
    IsAdmin,
    Data(Vec<T>)
}