use sqlx::PgPool;
mod models;
mod objects;
use axum::Router;
use models::{create_tables,create_user_if_not_exist};
use objects::AppState;
mod agent_views;
mod user_views;
use agent_views::send_routers;
use user_views::view_routers;

use tower_http::{cors::AllowOrigin};
use tower_http::cors::{CorsLayer};
use axum::http::{Method, header,header::HeaderValue};
use std::env;

#[tokio::main]
async fn main() {

    #[cfg(not(debug_assertions))]
    let allowed_origins: Vec<HeaderValue> =
    env::var("CORS")
        .unwrap_or_default()
        .split(',')
        .map(|s| HeaderValue::from_str(s).unwrap())
        .collect();
    
     #[cfg(debug_assertions)]
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

    #[cfg(not(debug_assertions))]
    let pg_pool = PgPool::connect(&env::var("PSQL_URL").unwrap_or_default())
        .await
        .unwrap();
    
    #[cfg(debug_assertions)]
    let pg_pool = PgPool::connect("postgres://myuser:mypassword@127.0.0.1:5432/mydatabase")
        .await
        .unwrap();
    
    match create_tables(&pg_pool).await {
        Ok(_) => {println!("table creation done");
        },
        Err(e) => {
            println!("error in table create {:?}", e);
            return;
        }
    };

    match create_user_if_not_exist(&pg_pool).await {
      Ok(_) => {println!("Set up done");
        },
        Err(e) => {
            println!("error in user create {:?}", e);
            return;
        }  
    };


    let app_state = AppState { db: pg_pool };

    let app = Router::new()
        .merge(send_routers(app_state.clone()))
        .merge(view_routers(app_state))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(env::var("HOST").unwrap_or("0.0.0.0:8000".to_string())).await.unwrap();
    println!("runing");
    axum::serve(listener, app).await.unwrap();
}
