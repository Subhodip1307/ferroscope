use crate::objects::AppState;
use axum::http::HeaderMap;
use sqlx::Row;

// auth
pub async fn auth(headers: HeaderMap, db_state: AppState) -> (bool, i64) {
    if let Some(auth) = headers.get("authorization") {
        let auth_str = auth.to_str().unwrap();
        let fetch_data =
            sqlx::query("SELECT id FROM nodes where token=$1 ORDER BY id DESC LIMIT 1")
                .bind(auth_str)
                .fetch_optional(&db_state.db)
                .await.expect("Auth error in agent view");
        let out_put = match fetch_data {
            Some(value) => (true, value.get("id")),
            None => {
                (false, 0)
            }
        };
        return out_put;
    }
    println!("auth failed ");
    (false, 0)
}
