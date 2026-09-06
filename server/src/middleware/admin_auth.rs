// use super::response::AuthUser;
use crate::state::AppState;
use crate::user_views::types::AuthUser;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn admin_auth(
    State(db_state): State<AppState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    if let Some(auth) = req.headers().get("authorization") {
        let auth_str = match auth.to_str() {
            Ok(v) => v,
            Err(_) => return Err(StatusCode::UNAUTHORIZED),
        };
        let cache_key = format!("user_auth_{auth_str}");
        let out_put: (bool, i64) = match db_state.cache.get(&cache_key) {
            Some(value) => (true, value),
            None => {
                let fetch_data = 
    sqlx::query_scalar::<_,i64>("SELECT t.user_id FROM auth_tokens t  where t.token=$1 AND EXISTS (SELECT 1 FROM users u where u.id = t.user_id AND u.is_admin = true )  ")
                    .persistent(true)
                    .bind(auth_str)
                    .fetch_optional(&db_state.db)
                    .await
                    .unwrap();
                let out_put: (bool, i64) = match fetch_data {
                    Some(value) => {
                        // setting the cache
                        db_state.cache.insert(cache_key, value);
                        (true, value)
                    }
                    None =>{db_state.cache.insert(cache_key, 0); (false, 0)},
                };
                // println!("the output is {:?} and the token is {auth_str}",out_put);
                out_put
            }
        };

        // println!("the output is {:?}",out_put);
        if !out_put.0 {
            return Err(StatusCode::FORBIDDEN);
        }
        req.extensions_mut().insert(AuthUser { user_id: out_put.1 });
        let response = next.run(req).await;
        return Ok(response);
    }
    Err(StatusCode::FORBIDDEN)
}
