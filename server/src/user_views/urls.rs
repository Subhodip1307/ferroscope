use super::auth;
use super::middleware::user_auth;
use super::streaming;
use super::read;
use super::write;
use crate::objects::AppState;
use axum::middleware::from_fn_with_state;
use axum::{
    Router,
    routing::{get, post},
};


pub fn auth_routers(app_state: AppState)->Router {
    
    let unprotected_auth_routers = Router::new()
        .route("/user_login", post(auth::__loginuser));

    let protected_auth_routers=Router::new()
        .route("/get_userdetails", post(auth::__get_user_name))
        .route("/change_password", post(auth::__change_password))
        .route_layer(from_fn_with_state(app_state.clone(), user_auth)) ;
    
    Router::new().merge(unprotected_auth_routers).merge(protected_auth_routers).with_state(app_state.clone())
}


fn view_routers(app_state: AppState) -> Router {
    Router::new()
        .route("/get_node_list", post(read::__get_node_list))
        .route("/get_node_info", post(read::__get_nodeinfo))
        .route("/get_latest_cpu", post(read::__get_latest_cpu))
        .route("/get_latest_ram", post(read::__get_latest_ram))
        .route("/cpu_stat", post(read::__get_latest_cpu_hisotry))
        .route("/ram_stat", post(read::__get_latest_ram_hisotry))
        .route("/node_services", post(read::__get_all_service_of_node))
        .route(
            "/single_service_current_stat",
            post(read::__get_single_service_current_status),
        )
        .route(
            "/service_current_stat",
            post(read::__get_service_current_status),
        )
        .route_layer(from_fn_with_state(app_state.clone(), user_auth))
        .with_state(app_state.clone())
}

fn streaming_routers(app_state: AppState) -> Router {
    Router::new()
        .route("/cpu", get(streaming::stream_cpu_metrics))
        .route("/ram", get(streaming::stream_ram_metrics))
        .with_state(app_state)
}

fn write_routers(app_state: AppState)->Router {
    Router::new()
    .route("/create_nodes", post(write::__create_node))
    .route("/remove_nodes", post(write::__remove_node))
    .route("/create_rules", post(write::__create_notification_rules))
    .route_layer(from_fn_with_state(app_state.clone(), user_auth))
    .with_state(app_state)
}

pub fn base_routers(app_state: AppState)->Router {
      Router::new()
        .nest("/view", view_routers(app_state.clone()))
        .nest("/auth", auth_routers(app_state.clone()))
        .nest("/stream", streaming_routers(app_state.clone()))
        .nest("/write", write_routers(app_state))
}
