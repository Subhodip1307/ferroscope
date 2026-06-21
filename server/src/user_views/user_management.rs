//  views realted to user mangements
use super::response as get_payload;
use crate::state::AppState;
use axum::{
    Json,Extension,
    extract::{State}
};
use super::types::{AuthUser};


pub (super) async fn __get_all_user_list(State(db_state): State<AppState>,Extension(auth_user): Extension<AuthUser>)->Json<Vec<get_payload::UserList>>{
    // get list of all users
    let user_row=sqlx::query_as::<_,get_payload::UserList>("SELECT username,email,joined_date where id != $1")
    .bind(auth_user.user_id)
    .fetch_all(&db_state.db).await.unwrap();
    Json(user_row)
}
// next user delete and edit
// pub (super) async 