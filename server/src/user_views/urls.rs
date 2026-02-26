use super::views;
use crate::objects::AppState;
use axum::{Router, routing::post};
use super::auth::login_user as __loginUser;

pub fn view_routers(app_state: AppState) -> Router {
    let r = Router::new()
        .route("/user_login", post(__loginUser))
        .route("/get_node_list", post(views::__get_node_list))
        .route("/get_node_info", post(views::__get_nodeinfo))
        .route("/get_latest_cpu", post(views::__get_latest_cpu))
        .route("/get_latest_ram", post(views::__get_latest_ram))
        .route("/cpu_stat", post(views::__get_latest_cpu_hisotry))
        .route("/ram_stat", post(views::__get_latest_ram_hisotry))
        .route("/node_services", post(views::__get_all_service_of_node))
        .route("/single_service_current_stat", post(views::__get_single_service_current_status))
        .route("/service_current_stat",post(views::__get_service_current_status),
    
    )
        .with_state(app_state);

    Router::new().nest("/view", r)
}
