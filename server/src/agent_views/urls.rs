use super::views;
use crate::objects::AppState;
use axum::{Router, routing::post};

pub fn send_routers(app_state: AppState) -> Router {
    Router::new()
        .route("/send_systeminfo", post(views::__system_info))
        .route("/send_cpu", post(views::__cpu_metrix))
        .route("/send_memory", post(views::__memory_metrix))
        .route("/send_service", post(views::__service_monitor))
        .with_state(app_state)
    
}
