use std::collections::HashMap;
use argon2::{Algorithm, Argon2, Params, PasswordVerifier, Version};
use crate::objects::AppState;
use axum::http::{StatusCode};
use sqlx::Row;
use axum::{Json,extract::{State},Extension};
use super::payloads::{Login,UsernamePasswordReset};
use super::response::{UserToken,AuthUser,AuthuserIdPassword};
use uuid::Uuid;
use argon2::password_hash::{SaltString, PasswordHasher,PasswordHash};
use rand::rngs::OsRng;

pub(super) async fn login_user(
    State(db_state): State<AppState>,
    Json(cread):Json<Login>
)->Result<(StatusCode,Json<UserToken>),Json<HashMap<&'static str,&'static str>>>{
    // currenlty Storeing password in plain text need to fixed
    let user=
    sqlx::query("SELECT id,password_hash from users where username= $1")
    .bind(cread.username)
    .fetch_optional(&db_state.db)
    .await.unwrap();

    if let Some(user_obj)=user{
        // matching user password
        let uesr_password:String=user_obj.get("password_hash");
        match verify_password(&cread.password,&uesr_password) {
            true =>{},
            false=>{
                return Err(
                    Json(HashMap::from([
                        ("msg","no user found")
                    ]))
                );
            }            
        }


        let user_model_id:i64=user_obj.get("id");
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


pub(super) async fn __get_user_name(
    State(db_state): State<AppState>,
    Extension(auth_user):Extension<AuthUser>
)->Json<AuthuserIdPassword>{
    let query=sqlx::query(
        "SELECT username FROM users WHERE id=$1"
    )
    .bind(auth_user.user_id)
    .fetch_one(&db_state.db).await.unwrap();
    // returns User name and id
    Json(AuthuserIdPassword{user_id:auth_user.user_id,username:query.get("username")})
}


pub(super) async fn __change_password(
    State(db_state): State<AppState>,
    Extension(auth_user):Extension<AuthUser>,
    Json(cread):Json<UsernamePasswordReset>,
)->StatusCode{
    let query_status=sqlx::query(
        "UPDATE users SET username=$1,password_hash=$2 WHERE id=$3"
    )
    .bind(cread.username)
    .bind(hash_password(&cread.password))
    .bind(auth_user.user_id)
    .execute(&db_state.db).await;

    match query_status {
        Ok(_)=>return StatusCode::OK,
        Err(_)=>return StatusCode::CONFLICT
    }
}



fn argon2_instance() -> Argon2<'static> {
    let params = Params::new(
        19_456, // memory (KB)
        2,      // iterations
        1,      // parallelism
        None,
    ).unwrap();

    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = argon2_instance();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

fn verify_password(
    password: &str,
    stored_hash: &str,
) -> bool {
    let argon2 = argon2_instance();

    let parsed_hash = match PasswordHash::new(stored_hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}