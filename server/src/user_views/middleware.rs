use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::objects::AppState;
use sqlx::Row;

pub(super) async fn user_auth(
    State(db_state): State<AppState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, axum::http::StatusCode>{
    if let Some(auth) = req.headers().get("authorization") {
        let auth_str = auth.to_str().unwrap();
        let fetch_data =
            sqlx::query("SELECT user_id FROM auth_tokens where token=$1")
                .bind(auth_str)
                .fetch_optional(&db_state.db)
                .await.unwrap();
            
        let out_put = match fetch_data {
            Some(value) => (true, value.get("user_id")),
            None => {
                (false, 0)
            }
        };
        if !out_put.0{
                return Err(StatusCode::UNAUTHORIZED);
        }
        req.extensions_mut().insert(out_put.1);
        let response = next.run(req).await;
        return Ok(response);
    }
    Err(StatusCode::UNAUTHORIZED)
}