//  views realted to user mangements
use super::payloads;
use super::response;
use super::types::{ApiResponse, AuthUser, RespMessage};
use crate::state::AppState;
use axum::{Extension, Json, extract::{State,Path}, http::StatusCode};
use ferroscope_server::global::utils_functions::hash_password;
use std::collections::HashMap;
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
     Json(user): Json<AuthUser>,
) -> ApiResult {
    check_admin(
        &db_state.db,
        user_id.user_id,
        "You don't have permission to delete this user.",
    )
    .await?;
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

// assign Permission
pub(super) async fn __assign_permission(
    State(db_state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Json(data): Json<payloads::AssignPermission>,
) -> ApiResult {
    check_admin(
        &db_state.db,
        user.user_id,
        "You don't have permission to edit this user.",
    )
    .await?;

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


pub(super) async fn __get_user_permissions(
    State(db_state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(target_user_id): Path<i64>,
) -> Result<ApiResponse<response::UserPermissionsResponse>,(StatusCode, RespMessage)> {
    check_admin(
        &db_state.db,
        user.user_id,
        "You don't have permission to view this user.",
    )
    .await?;

    // 1. base node access
    let nodes: Vec<(i64, bool)> = sqlx::query_as(
        "SELECT node_id, is_full_access
         FROM user_node_access
         WHERE user_id = $1",
    )
    .bind(target_user_id)
    .fetch_all(&db_state.db)
    .await.unwrap();

    // 2. metrics
    let metrics: Vec<(i64, String)> = sqlx::query_as(
        "SELECT node_id, metric_name
         FROM user_node_metric_access
         WHERE user_id = $1",
    )
    .bind(target_user_id)
    .fetch_all(&db_state.db)
    .await.unwrap();

    // 3. services — join through service_monitor to know which node each belongs to
    let services: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT sm.node_id, usa.service_id
         FROM user_node_service_access usa
         JOIN service_monitor sm ON sm.id = usa.service_id
         WHERE usa.user_id = $1",
    )
    .bind(target_user_id)
    .fetch_all(&db_state.db)
    .await.unwrap();

    // merge everything keyed by node_id
    let mut map: HashMap<i64, response::NodePermissionView> = nodes
        .into_iter()
        .map(|(node_id, is_full_access)| {
            (node_id, response::NodePermissionView {
                node_id,
                is_full_access,
                metrix: Vec::new(),
                services: Vec::new(),
            })
        })
        .collect();

    for (node_id, metric) in metrics {
        if let Some(n) = map.get_mut(&node_id) {
            n.metrix.push(metric);
        }
    }
    for (node_id, service_id) in services {
        if let Some(n) = map.get_mut(&node_id) {
            n.services.push(service_id);
        }
    }

    let resp = response::UserPermissionsResponse {
        user_id: target_user_id,
        nodes_permissions: map.into_values().collect(),
    };

    Ok(ApiResponse { data: resp })
}

