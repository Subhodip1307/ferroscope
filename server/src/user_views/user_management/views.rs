
use super::super::response;
use super::super::types::{ApiResponse, AuthUser, RespMessage};
use crate::state::AppState;
use axum::{Extension, extract::{State,Path}, http::StatusCode};
use std::collections::HashMap;

pub(super) async fn __get_all_user_list(
    State(db_state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
)-> Result<ApiResponse<Vec<response::UserList>>,(StatusCode, RespMessage)> {
    
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

pub(super) async fn __get_user_permissions(
    State(db_state): State<AppState>,
    Path(target_user_id): Path<i64>,
) -> Result<ApiResponse<response::UserPermissionsResponse>,(StatusCode, RespMessage)> {
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

