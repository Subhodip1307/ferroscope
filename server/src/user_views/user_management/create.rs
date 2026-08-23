// keep edit and create here for now
use super::super::payloads;
use super::super::types::{ AuthUser, RespMessage};
use crate::state::AppState;
use axum::{Json, extract::{State}, http::StatusCode};
use ferroscope_server::global::utils_functions::hash_password;

// helping function  to check if it's user is a admin or not
// may switch the poistion in future if required

// check admin error type
type ApiResult = Result<(StatusCode, RespMessage), (StatusCode, RespMessage)>;

pub(super) async fn __create_user(
    State(db_state): State<AppState>,
    Json(user): Json<payloads::UserDetails>,

)->ApiResult {
    sqlx::query("insert into users (username,email,is_admin,password_hash) values ($1,$2,$3,$4)")
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


pub(super) async fn __delete_user(
    State(db_state): State<AppState>,
     Json(user): Json<AuthUser>,
) -> ApiResult {
    sqlx::query("DELETE FROM users where id = $1")
        .bind(user.user_id)
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
    Json(user): Json<payloads::UserDetailsEdit>,
) -> ApiResult {

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

// assign Permission
pub(super) async fn __assign_permission(
    State(db_state): State<AppState>,
    Json(data): Json<payloads::AssignPermission>,
) -> ApiResult {
    println!("data is {:?}",data);

    // flatten everything up front
    let (node_ids,is_full_power):(Vec<i64>,Vec<bool>)=
    data.nodes_permissions.iter().map(|n|{
        (n.node_id,n.full_permission.unwrap_or(false))
    }).unzip();

    let (metric_node_ids, metric_names): (Vec<i64>, Vec<String>) = data
        .nodes_permissions
        .iter()
        .flat_map(|n| {
            n.metrix
                .iter()
                .flatten()
                .map(move |m| (n.node_id, m.to_string()))
        })
        .unzip();
    let service_ids: Vec<i64> = data
        .nodes_permissions
        .iter()
        .flat_map(|n| n.services.iter().flatten().copied())
        .collect();

    let mut tx = db_state.db.begin().await.unwrap();
    // removeing all the previous data
    sqlx::query("DELETE FROM user_node_access WHERE user_id = $1")
    .bind(data.user_id).execute(&mut *tx).await.unwrap();
    sqlx::query("DELETE FROM user_node_metric_access WHERE user_id = $1")
    .bind(data.user_id).execute(&mut *tx).await.unwrap();
    sqlx::query("DELETE FROM user_node_service_access WHERE user_id = $1")
    .bind(data.user_id).execute(&mut *tx).await.unwrap();

    // insert query
    sqlx::query(
        "INSERT INTO user_node_access (user_id, node_id,is_full_access)
         SELECT $1, * FROM UNNEST($2::bigint[],$3::boolean[])",
    )
    .bind(data.user_id)
    .bind(&node_ids)
    .bind(&is_full_power)
    .execute(&mut *tx)
    .await.unwrap();

    sqlx::query(
        "INSERT INTO user_node_metric_access (user_id, node_id, metric_name)
         SELECT $1, * FROM UNNEST($2::bigint[], $3::text[])",
    )
    .bind(data.user_id)
    .bind(&metric_node_ids)
    .bind(&metric_names)
    .execute(&mut *tx)
    .await.unwrap();

    sqlx::query(
        "INSERT INTO user_node_service_access (user_id, service_id)
         SELECT $1, * FROM UNNEST($2::bigint[])",
    )
    .bind(data.user_id)
    .bind(&service_ids)
    .execute(&mut *tx)
    .await.unwrap();

    tx.commit().await.unwrap();

    Ok((
        StatusCode::CREATED,
        RespMessage {
            res: true,
            msg: "User data updated",
        },
    ))
}

