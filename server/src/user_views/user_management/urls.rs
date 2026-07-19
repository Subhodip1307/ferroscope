use super::super::admin_middleware;
use crate::state::AppState;
use axum::middleware::from_fn_with_state;
use axum::{Router,routing::{get, post}};
use super::create;
use super::views;

pub fn access_control(app_state: AppState) -> Router {
    Router::new()
    .nest("/views", view_routes(app_state.clone()))
    .nest("/create", create_routes(app_state.clone()))
    .route_layer(from_fn_with_state(app_state,admin_middleware))
}

fn create_routes(app_state: AppState)-> Router {
    Router::new()
    .route("/create_user", post(create::__create_user))
    .route("/delete_user", post(create::__delete_user))
    .route("/edit_user_details", post(create::__edit_user_details))
    .route("/assign_permission", post(create::__assign_permission))
    .with_state(app_state)

}
fn view_routes(app_state: AppState)-> Router {
    Router::new()
    .route("/all_users", get(views::__get_all_user_list))
    .route("/users/{user_id}/permissions", get(views::__get_user_permissions))
    .with_state(app_state)
}