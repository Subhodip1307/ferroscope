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
            sqlx::query("SELECT u.id,u.username FROM auth_tokens a JOIN users u on a.user_id = u.id where a.token=$1")
                .bind(auth_str)
                .fetch_optional(&db_state.db)
                .await.unwrap();
            
        let out_put: (bool, i64,String) = match fetch_data {
            Some(value) => (true, value.get("id"),value.get("username")),
            None => {
                (false, 0,"".to_string())
            }
        };
        if !out_put.0{
                return Err(StatusCode::UNAUTHORIZED);
        }
        req.extensions_mut().insert(AuthUser{user_id:out_put.1,username:out_put.2});
        let response = next.run(req).await;
        return Ok(response);
    }
    Err(StatusCode::UNAUTHORIZED)
}