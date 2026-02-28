use sqlx::PgPool;
mod models;
mod objects;
use axum::Router;
use models::create_tables;
use objects::AppState;
mod agent_views;
mod user_views;
use agent_views::send_routers;
use user_views::view_routers;

use tower_http::{cors::AllowOrigin};
use tower_http::cors::{CorsLayer};
use axum::http::{Method, header,header::HeaderValue};

#[tokio::main]
async fn main() {

    let allowed_origins = [
        HeaderValue::from_static("http://localhost:3001"),
        HeaderValue::from_static("http://127.0.0.1:3001"),
        HeaderValue::from_static("http://192.168.0.161:3001"),
    ];

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true);

    let pg_pool = PgPool::connect("postgres://myuser:mypassword@127.0.0.1:5432/mydatabase")
        .await
        .expect("DB connection failed");
    println!("Hello, world!");
    match create_tables(&pg_pool).await {
        Ok(_) => println!("table creation done"),
        Err(e) => {
            println!("error in table create {:?}", e);
            return;
        }
    };
    let app_state = AppState { db: pg_pool };

    let app = Router::new()
        .merge(send_routers(app_state.clone()))
        .merge(view_routers(app_state))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
