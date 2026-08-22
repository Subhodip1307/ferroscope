use super::payloads;
use super::response as get_payload;
use crate::state::AppState;
use crate::user_views::types;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use sqlx::Row;
use std::collections::HashMap;

pub(super) async fn __get_node_list(
    State(db_state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<get_payload::NodesList>>), StatusCode> {
    let rows: Vec<get_payload::NodesList> = sqlx::query_as("SELECT id,name FROM nodes")
        .fetch_all(&db_state.db)
        .await
        .unwrap();
    Ok((StatusCode::OK, Json(rows)))
}

pub(super) async fn __get_nodeinfo(
    //sysinfo
    State(db_state): State<AppState>,
    Query(params): Query<payloads::IdQuery>,
) -> Result<(StatusCode, Json<get_payload::SysInfo>), StatusCode> {
    let row = sqlx::query_as::<_,get_payload::SysInfo>(
        "SELECT s.system_name, s.kernel_version, s.os_version, s.uptime, s.cpu_threads, s.cpu_vendor, n.name as node_name FROM sysinfo s JOIN nodes n ON s.node_id = n.id where node_id = $1",
    )
    .bind(params.node)
    .fetch_optional(&db_state.db)
    .await.unwrap();

    if let Some(item) = row {
        return Ok((StatusCode::OK, Json(item)));
    };
    Err(StatusCode::NO_CONTENT)
}

pub(super) async fn __get_latest_cpu(
    State(db_state): State<AppState>,
    Query(params): Query<payloads::IdQuery>,
) -> Result<(StatusCode, Json<get_payload::LatestCpu>), StatusCode> {
    let row = sqlx::query(
        "SELECT value,date_time FROM cpu_stats where node_id = $1 ORDER BY date_time DESC LIMIT 1",
    )
    .bind(params.node)
    .fetch_optional(&db_state.db)
    .await
    .unwrap();

    if let Some(item) = row {
        let value: f64 = item.get("value");
        let date_time: DateTime<Utc> = item.get("date_time");
        return Ok((
            StatusCode::OK,
            Json(get_payload::LatestCpu { value, date_time }),
        ));
    };
    Err(StatusCode::NO_CONTENT)
}

pub(super) async fn __get_latest_ram(
    State(db_state): State<AppState>,
    Query(params): Query<payloads::IdQuery>,
) -> Result<(StatusCode, Json<get_payload::LatestRam>), StatusCode> {
    let value = sqlx::query(
        "SELECT free,total,date_time FROM memory_metrics where node_id = $1 ORDER BY date_time DESC LIMIT 1",
    )
    .bind(params.node)
    .fetch_optional(&db_state.db)
    .await
    .unwrap();
    if let Some(row) = value {
        let total: String = row.get("total");
        let free: String = row.get("free");
        let timestamp: DateTime<Utc> = row.get("date_time");
        return Ok((
            StatusCode::OK,
            Json(get_payload::LatestRam {
                total,
                free,
                timestamp,
            }),
        ));
    }
    Err(StatusCode::NO_CONTENT)
}

pub(super) async fn __get_latest_cpu_hisotry(
    State(db_state): State<AppState>,
    Query(params): Query<payloads::IdQuery>,
) -> (StatusCode, Json<Vec<get_payload::LatestCpu>>) {
    let row: Vec<get_payload::LatestCpu> = sqlx::query_as(
        "SELECT value,date_time FROM cpu_stats where node_id = $1 ORDER BY date_time DESC LIMIT 20",
    )
    .bind(params.node)
    .fetch_all(&db_state.db)
    .await
    .unwrap();
    (StatusCode::OK, Json(row))
}

pub(super) async fn __get_latest_ram_hisotry(
    State(db_state): State<AppState>,
    Query(params): Query<payloads::IdQuery>,
) -> (StatusCode, Json<Vec<get_payload::LatestRam>>) {
    let row:Vec<get_payload::LatestRam> = sqlx::query_as(
        "SELECT free,total,date_time as timestamp FROM memory_metrics where node_id = $1 ORDER BY date_time DESC LIMIT 20",
    )
    .bind(params.node)
    .fetch_all(&db_state.db)
    .await
    .unwrap();
    (StatusCode::OK, Json(row))
}

pub(super) async fn __get_all_service_of_node(
    State(db_state): State<AppState>,
    Query(params): Query<payloads::IdQuery>,
) -> (
    StatusCode,
    Json<HashMap<String, Vec<get_payload::ServiceList>>>,
) {
    // TODO: Update this code DOCS/ or remove it
    let rows: Vec<get_payload::ServiceList> = sqlx::query_as(
        "SELECT service_name,category,ssl_exp FROM service_monitor where node_id = $1",
    )
    .bind(params.node)
    .fetch_all(&db_state.db)
    .await
    .unwrap();

    let mut grouped: HashMap<String, Vec<get_payload::ServiceList>> = HashMap::new();
    for service in rows {
        grouped
            .entry(service.category.clone())
            .or_default()
            .push(service);
    }
    (StatusCode::OK, Json(grouped))
}

pub(super) async fn __get_all_service_name_of_node(
    State(db_state): State<AppState>,
    Query(params): Query<payloads::IdQuery>,
)-> (
    StatusCode,
    Json<Vec<String>>,
) 
{
    let rows: Vec<String> = sqlx::query_scalar(
        "SELECT service_name FROM service_monitor where node_id = $1",
    )
    .bind(params.node)
    .fetch_all(&db_state.db)
    .await
    .unwrap();
    (StatusCode::OK,Json(rows))


}

pub(super) async fn __get_single_service_current_status(
    State(db_state): State<AppState>,
    Json(payload): Json<payloads::ServiceQuery>,
) -> Result<(StatusCode, Json<get_payload::SingleServiceStatus>), StatusCode> {
    // will get node id and service name from query parameter or from json payload then the responce will be returnd
    // remove this Unreachable and Reachable logic it's wrong
    let row = sqlx::query(
        "SELECT  error_msg,status,category,ssl_exp
         FROM service_monitor where node_id = $1 and service_name= $2",
    )
    .bind(payload.node)
    .bind(payload.service_name)
    .fetch_optional(&db_state.db)
    .await
    .unwrap();
    if let Some(value) = row {
        let error_msg = value.get("error_msg");
        let status = value.get("status");
        let category = value.get("category");
        let ssl_exp = value.get("ssl_exp");
        return Ok((
            StatusCode::OK,
            Json(get_payload::SingleServiceStatus {
                status,
                error_msg,
                category,
                ssl_exp,
            }),
        ));
    }
    Err(StatusCode::NO_CONTENT)
}

pub(super) async fn __get_service_current_status(
    State(db_state): State<AppState>,
    Query(params): Query<payloads::IdQuery>,
) -> (
    StatusCode,
    Json<HashMap<String, Vec<get_payload::ServiceStatus>>>,
) {
    let rows = sqlx::query_as::<_, get_payload::ServiceStatus>(
        "SELECT  error_msg,status,service_name,category,ssl_exp
         FROM service_monitor where node_id = $1 ",
    )
    .bind(params.node)
    .fetch_all(&db_state.db)
    .await
    .unwrap();

    let mut grouped: HashMap<String, Vec<get_payload::ServiceStatus>> = HashMap::new();
    for service in rows {
        grouped
            .entry(service.category.clone())
            .or_default()
            .push(service);
    }

    (StatusCode::OK, Json(grouped))
}

pub(super) async fn __get_event_type() -> Json<get_payload::__ArrayType<'static>> {
    let data = Vec::from(["SERVICE", "NODE"]);
    Json(get_payload::__ArrayType { data })
}

pub(super) async fn __get_notification_type() -> Json<get_payload::__ArrayType<'static>> {
    let data = Vec::from(["webhook", "email"]);
    Json(get_payload::__ArrayType { data })
}

pub(super) async fn __get_nodes_with_services(
    State(db_state): State<AppState>,
    Json(nodes_ids): Json<payloads::MutiIdQuery>
) -> Result<types::ApiResponse<Vec<get_payload::NodeWithServices>>, (StatusCode, types::RespMessage)> {
    // get services list with nodes input
    // TODO: check for admin
    // 1. all nodes (id + name)
    let nodes: Vec<(i64, String)> =
        sqlx::query_as("SELECT id, name FROM nodes WHERE id = ANY($1) ORDER BY id")
        .bind(&nodes_ids.obj_ids)
            .fetch_all(&db_state.db)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                   types::RespMessage { res: false, msg: "Database error." },
                )
            })?;

    // 2. all services (id, name, which node they belong to)
    let services: Vec<(i64, String, i64)> =
        sqlx::query_as("SELECT id, service_name, node_id FROM service_monitor")
            .fetch_all(&db_state.db)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                   types::RespMessage { res: false, msg: "Database error." },
                )
            })?;

    // 3. build node list, remembering each node's position so we can attach services
    let mut result: Vec<get_payload::NodeWithServices> = Vec::with_capacity(nodes.len());
    let mut index_of: HashMap<i64, usize> = HashMap::with_capacity(nodes.len());

    for (node_id, node_name) in nodes {
        index_of.insert(node_id, result.len());
        result.push(get_payload::NodeWithServices {
            node_id,
            node_name,
            services: Vec::new(),
        });
    }

    for (service_id, service_name, node_id) in services {
        if let Some(&idx) = index_of.get(&node_id) {
            result[idx].services.push(get_payload::ServiceInfo {
                id: service_id,
                service_name,
            });
        }
    }

    Ok(types::ApiResponse { data: result })
}