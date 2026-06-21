use axum::Router;
use crate::agent_views::send_routers;
use crate::user_views::base_routers;
use axum::http::{Method, header, header::HeaderValue};
use tower_http::cors::AllowOrigin;
use tower_http::cors::CorsLayer;
use crate::state::AppState;
#[cfg(not(debug_assertions))]
use crate::env;

fn cors()-> CorsLayer{
    #[cfg(not(debug_assertions))]
    let allowed_origins: Vec<HeaderValue> = env::var("CORS")
        .unwrap_or_default()
        .split(',')
        .map(|s| HeaderValue::from_str(s).unwrap())
        .collect();

    #[cfg(debug_assertions)]
    let allowed_origins = [
        HeaderValue::from_static("http://localhost:3000"),
        HeaderValue::from_static("http://127.0.0.1:3000")
    ];

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true)
}


pub fn create_axum_app(app_state:AppState)-> Router{
    Router::new()
        .merge(send_routers(app_state.clone()))
        .merge(base_routers(app_state.clone()))
        .layer(cors())
}