use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::objects::AppState;
use sqlx::Row;
use super::response::AuthUser;

pub(super) async fn user_auth(
    State(db_state): State<AppState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, axum::http::StatusCode>{
    if let Some(auth) = req.headers().get("authorization") {
        let auth_str = match auth.to_str() {
            Ok(v) => v,
            Err(_) => return Err(StatusCode::UNAUTHORIZED),
        };
        let fetch_data =
            sqlx::query("SELECT user_id FROM auth_tokens token=$1")
                .persistent(true)
                .bind(auth_str)
                .fetch_optional(&db_state.db)
                .await.unwrap();
            
        let out_put: (bool, i64) = match fetch_data {
            Some(value) => (true, value.get("user_id")),
            None => {
                (false, 0)
            }
        };
        if !out_put.0{
                return Err(StatusCode::UNAUTHORIZED);
        }
        req.extensions_mut().insert(AuthUser{user_id:out_put.1});
        let response = next.run(req).await;
        return Ok(response);
    }
    Err(StatusCode::UNAUTHORIZED)
}