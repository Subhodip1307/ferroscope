//  views realted to user mangements
use super::payloads;
use super::response;
use super::types::{ApiResponse, AuthUser, RespMessage};
use crate::state::AppState;
use axum::{Extension, Json, extract::State, http::StatusCode};
use ferroscope_server::global::utils_functions::hash_password;
// helping function  to check if it's user is a admin or not
// may switch the poistion in future if required
async fn check_admin(
    db: &sqlx::PgPool,
    user_id: i64,
    forbidden_msg: &'static str,
) -> Result<(), (StatusCode, RespMessage)> {
    // only admin can delete a user
    let is_admin: bool =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM users WHERE id = $1 AND is_admin = true)")
            .bind(user_id)
            .fetch_one(db)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    RespMessage {
                        res: false,
                        msg: "Database error.",
                    },
                )
            })?;

    if !is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            RespMessage {
                res: false,
                msg: forbidden_msg,
            },
        ));
    }
    Ok(())
}
// check admin error type
type ApiResult = Result<(StatusCode, RespMessage), (StatusCode, RespMessage)>;

pub(super) async fn __create_user(
    State(db_state): State<AppState>,
    Extension(user_id): Extension<AuthUser /*using Authuser here just to get the id*/>,
    Json(user): Json<payloads::UserDetails>,

)->ApiResult {
    check_admin(
        &db_state.db,
        user_id.user_id,
        "You don't have permission to delete this user.",
    )
    .await?;
    sqlx::query("insert into users (username,email,is_admin,password_hash) values ($1,$2,$3)")
    .bind(user.username)
    .bind(user.email)
    .bind(user.is_admin)
    .bind(hash_password(&user.password))
    .execute(&db_state.db).await.map_err(|e|{
        if let sqlx::Error::Database(db_err)=e{
            if db_err.is_unique_violation(){
                return (StatusCode::CONFLICT,RespMessage{res:true,msg:"Username already exists"});
            }
        }
        (StatusCode::INTERNAL_SERVER_ERROR,RespMessage{res:true,msg:"Something Went Wrong while Creating User"})
    })?;
    Ok((StatusCode::CREATED,RespMessage{res:true,msg:"User Creation successfully done"}))
}

pub(super) async fn __get_all_user_list(
    State(db_state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
)-> Result<ApiResponse<Vec<response::UserList>>,(StatusCode, RespMessage)> {
    check_admin(
        &db_state.db,
        auth_user.user_id,
        "You don't have permission to delete this user.",
    )
    .await?;
    // get list of all users except current user
    let user_row = sqlx::query_as::<_, response::UserList>(
        "SELECT id,username,email,joined_date,is_admin FROM users where id != $1",
    )
    .bind(auth_user.user_id)
    .fetch_all(&db_state.db)
    .await
    .unwrap();
    Ok(ApiResponse { data: user_row })
}

pub(super) async fn __delete_user(
    State(db_state): State<AppState>,
    Extension(user_id): Extension<AuthUser /*using Authuser here just to get the id*/>,
) -> ApiResult {
    check_admin(
        &db_state.db,
        user_id.user_id,
        "You don't have permission to delete this user.",
    )
    .await?;
    sqlx::query("DELETE FROM users where id = $1")
        .bind(user_id.user_id)
        .execute(&db_state.db)
        .await
        .unwrap();
    Ok((
        StatusCode::OK,
        RespMessage {
            res: true,
            msg: "User deleted successfully.",
        },
    ))
}

pub(super) async fn __edit_user_details(
    State(db_state): State<AppState>,
    Extension(user_id): Extension<AuthUser /*using Authuser here just to get the id*/>,
    Json(user): Json<payloads::UserDetailsEdit>,
) -> ApiResult {
    check_admin(
        &db_state.db,
        user_id.user_id,
        "You don't have permission to edit this user.",
    )
    .await?;
    // just edit user details
    let hashed_password: Option<String> = match user.password {
        Some(v) => Some(hash_password(
            &v,
        )),
        None => None,
    };
    let result= sqlx::query("UPDATE users SET username = $2,email=$3,password_hash=COALESCE($4,password_hash),is_admin=$5 where id = $1")
    .bind(user.id)
    .bind(user.username)
    .bind(user.email)
    .bind(hashed_password)
    .bind(user.is_admin)
    .execute(&db_state.db).await;
    //TODO: check the edit count/ impacted rows
    if result.is_err() {
        //give better error
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            RespMessage {
                res: false,
                msg: "Something went wrong",
            },
        ));
    }
    Ok((
        StatusCode::CREATED,
        RespMessage {
            res: true,
            msg: "User data updated",
        },
    ))
    //  StatusCode::CREATED
}

// assigin Permission
