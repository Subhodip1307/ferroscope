mod state;
use state::AppState;
mod agent_views;
mod user_views;
use std::env;
mod bg_services;
mod process;
use tokio::sync::mpsc;
mod app;
mod middleware;

#[tokio::main]
async fn main() {
    println!("Runing Version : {}",env!("CARGO_PKG_VERSION"));
    let (tx, rx) = mpsc::channel::<ferroscope_server::global::structure::NotificationData>(20);
    let app_state = AppState::new( tx).await;
    let pg_pool=app_state.db.clone();

    sqlx::migrate!("./migrations").run(&pg_pool).await.unwrap();
    let app = app::create_axum_app(app_state.clone());
    let host = env::var("HOST").unwrap_or("0.0.0.0:8000".to_string());
    bg_services::node_status_check(app_state).await;
    bg_services::notification_service(pg_pool, rx).await;
    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();
    println!("runing on {}", host);
    axum::serve(listener, app).await.unwrap();
}
