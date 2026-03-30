use super::middleware::agent_auth_middleware;
use super::views;
use crate::objects::AppState;
use axum::middleware::from_fn_with_state;
use axum::{
    Router,
    routing::{get, post},
};

pub fn send_routers(app_state: AppState) -> Router {
    Router::new()
        .route("/send_systeminfo", post(views::__system_info))
        .route("/send_cpu", post(views::__cpu_metrix))
        .route("/send_memory", post(views::__memory_metrix))
        .route("/send_service", post(views::__service_monitor))
        .route("/send_uptime", post(views::__update_uptime))
        .route("/helth_check", get(views::__helth_check))
        .route_layer(from_fn_with_state(app_state.clone(), agent_auth_middleware))
        .with_state(app_state)
}
