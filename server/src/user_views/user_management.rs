//  views realted to user mangements
use super::response;
use super::payloads;
use crate::state::AppState;
use axum::{
    Json,Extension,
    extract::{State},http::StatusCode
};
use super::types::{AuthUser,ApiResponse,RespMessage};


pub (super) async fn __get_all_user_list(State(db_state): State<AppState>,Extension(auth_user): Extension<AuthUser>)->ApiResponse<Vec<response::UserList>>{
    // get list of all users except current user
    let user_row=sqlx::query_as::<_,response::UserList>("SELECT id,username,email,joined_date FROM users where id != $1")
    .bind(auth_user.user_id)
    .fetch_all(&db_state.db).await.unwrap();
    ApiResponse{data:user_row}
}

pub (super) async fn __delete_user(State(db_state): State<AppState>,Json(user_id): Json<AuthUser /*using Authuser here just to get the id*/>)
->StatusCode
{
    // deleting a User in future will check that the user has that power or not
    sqlx::query("DELETE FROM users where id = $1")
    .bind(user_id.user_id)
    .execute(&db_state.db).await.unwrap();
    StatusCode::OK
}

pub (super) async fn __edit_user_details(
    State(db_state): State<AppState>,Json(user): Json<payloads::UserDetailsEdit>
)
->(StatusCode,RespMessage)
{
    // just edit user details
    let hashed_password:Option<String>=match user.password {
        Some(v)=>Some(ferroscope_server::global::utils_functions::hash_password(&v)),
        None=>None,
    };
    let result= sqlx::query("UPDATE users SET username = $2,email=$3,password_hash=COALESCE($4,password_hash) where id = $1")
    .bind(user.id)
    .bind(user.username)
    .bind(user.email)
    .bind(hashed_password)
    .execute(&db_state.db).await;
    //TODO: check the edit count/ impacted rows  
    if result.is_err(){//give better error 
        return (StatusCode::INTERNAL_SERVER_ERROR,RespMessage{res:false,msg:"Something went wrong"});
     }
     (StatusCode::CREATED,RespMessage{res:false,msg:"User data updated"})
    //  StatusCode::CREATED
}