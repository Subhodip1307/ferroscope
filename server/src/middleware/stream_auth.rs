use crate::state::AppState;
use crate::user_views::types::AuthUser;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn user_auth_sse(
    State(db_state): State<AppState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Just borrow the query string — this does NOT consume or strip anything.
    // The URI stays intact, so all other params still reach your handler.
    let token = req.uri().query().and_then(|q| {
        form_urlencoded::parse(q.as_bytes())
            .find(|(k, _)| k == "token")
            .map(|(_, v)| v.into_owned())
    });

    let token = match token {
        Some(t) if !t.is_empty() => t,
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    let cache_key = format!("user_auth_{token}");

    let out_put: (bool, i64) = match db_state.cache.get(&cache_key) {
        Some(value) => (true, value),
        None => {
            let fetch_data = sqlx::query_scalar::<_, i64>(
                "SELECT user_id FROM auth_tokens WHERE token = $1",
            )
            .persistent(true)
            .bind(&token)
            .fetch_optional(&db_state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // don't panic on DB errors

            match fetch_data {
                Some(value) => {
                    db_state.cache.insert(cache_key, value);
                    (true, value)
                }
                None => (false, 0),
            }
        }
    };

    if !out_put.0 {
        return Err(StatusCode::UNAUTHORIZED);
    }

    req.extensions_mut().insert(AuthUser { user_id: out_put.1 });

    // req forwarded as-is: URI + every query param preserved.
    Ok(next.run(req).await)
}