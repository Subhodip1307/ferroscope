use super::views;
use crate::objects::AppState;
use axum::{Router, routing::{post,get}};
use super::auth::{login_user as __loginUser,__get_user_name,__change_password};
use super::middleware::user_auth;
use axum::middleware::from_fn_with_state;
use super::streaming;



fn streaming_routers(app_state: AppState)->Router{
    Router::new()
    .route("/cpu", get(streaming::stream_cpu_metrics)).with_state(app_state.clone())
    .route("/ram", get(streaming::stream_ram_metrics)).with_state(app_state.clone())

}



pub fn view_routers(app_state: AppState) -> Router {
    let auth_routers=Router::new()
    .route("/user_login", post(__loginUser)).with_state(app_state.clone());
    
    let data_router = Router::new()
        .route("/get_node_list", post(views::__get_node_list))
        .route("/get_node_info", post(views::__get_nodeinfo))
        .route("/get_latest_cpu", post(views::__get_latest_cpu))
        .route("/get_latest_ram", post(views::__get_latest_ram))
        .route("/cpu_stat", post(views::__get_latest_cpu_hisotry))
        .route("/ram_stat", post(views::__get_latest_ram_hisotry))
        .route("/node_services", post(views::__get_all_service_of_node))
        .route("/single_service_current_stat", post(views::__get_single_service_current_status))
        .route("/service_current_stat",post(views::__get_service_current_status))
        .route("/get_userdetails",post(__get_user_name))
        .route("/change_password",post(__change_password))
        
    .route_layer(from_fn_with_state(app_state.clone(), user_auth))
        .with_state(app_state.clone());

    Router::new().nest("/view", data_router)
    .nest("/auth", auth_routers)
    .nest("/stream", streaming_routers(app_state))
}
