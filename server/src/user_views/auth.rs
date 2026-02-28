use std::collections::HashMap;

use crate::objects::AppState;
use axum::http::{HeaderMap, StatusCode};
use sqlx::Row;
use axum::{Json,extract::{State}};
use super::payloads::Login;
use super::response::UserToken;
use uuid::Uuid;


pub(super) async fn login_user(
    State(db_state): State<AppState>,
    Json(cread):Json<Login>
)->Result<(StatusCode,Json<UserToken>),Json<HashMap<&'static str,&'static str>>>{
    // currenlty Storeing password in plain text need to fixed
    let user=
    sqlx::query("SELECT id from users where username= $1 and password_hash=$2")
    .bind(cread.username)
    .bind(cread.password)
    .fetch_optional(&db_state.db)
    .await.unwrap();
    if let Some(user_id)=user{

        let user_model_id:i64=user_id.get("id");
        // user found creating a token 
        let mut tx=db_state.db.begin().await.unwrap();

            sqlx::query("DELETE  from auth_tokens where user_id = $1")
            .bind(user_model_id)
            .fetch_optional(&mut *tx)
            .await.unwrap();

            // creating token
            let token= Uuid::new_v4().to_string();

            sqlx::query("insert into auth_tokens (user_id, token) values ($1,$2)")
            .bind(user_model_id)
            .bind(&token)
            .execute(&mut *tx)
            .await.unwrap();
            tx.commit().await.unwrap();
            return Ok(
                (StatusCode::OK,Json(UserToken{token}))
            );
        }
    Err(
        Json(HashMap::from([
            ("msg","no user found")
        ]))
    )

}