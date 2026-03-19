use super::payloads;
use super::response as get_payload;
use crate::objects::AppState;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;
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
        "SELECT system_name,kernel_version,os_version,uptime,cpu_threads,cpu_vendor FROM sysinfo where node_id = $1",
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
) -> (StatusCode, Json<HashMap<String, Vec<get_payload::ServiceList>>>) {
    // TODO: Update this code DOCS/ or remove it
    let rows: Vec<get_payload::ServiceList> =
        sqlx::query_as("SELECT service_name,category,ssl_exp FROM service_monitor where node_id = $1")
            .bind(params.node)
            .fetch_all(&db_state.db)
            .await
            .unwrap();

    let mut grouped: HashMap<String, Vec<get_payload::ServiceList>> = HashMap::new();
    for service in rows {
        grouped
            .entry(service.category.clone())
            .or_insert_with(Vec::new)
            .push(service);
    }
    (StatusCode::OK, Json(grouped))
}

pub(super) async fn __get_single_service_current_status(
    State(db_state): State<AppState>,
    Json(payload): Json<payloads::ServiceQuery>,
) -> Result<(StatusCode, Json<get_payload::SingleServiceStatus>), StatusCode> {
    // will get node id and service name from query parameter or from json payload then the responce will be returnd
    let row = sqlx::query(
        "SELECT  error_msg,status,category,ssl_exp
            CASE WHEN updated_at < NOW() - INTERVAL '5 minutes'
            THEN 'Unreachable'
            ELSE 'Reachable'
            END AS service_status
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
        let service_status = value.get("service_status");
        let category = value.get("category");
        let ssl_exp = value.get("ssl_exp");
        return Ok((
            StatusCode::OK,
            Json(get_payload::SingleServiceStatus {
                status,
                error_msg,
                service_status,category,ssl_exp
            }),
        ));
    }
    Err(StatusCode::NO_CONTENT)
}

pub(super) async fn __get_service_current_status(
    State(db_state): State<AppState>,
    Query(params): Query<payloads::IdQuery>,
) -> (StatusCode, Json<HashMap<String, Vec<get_payload::ServiceStatus>>>) {
    let rows = sqlx::query_as::<_, get_payload::ServiceStatus>(
        "SELECT  error_msg,status,service_name,category,ssl_exp,
            CASE 
            WHEN  updated_at < NOW() - INTERVAL '5 minutes'
            THEN 'Unreachable'
            ELSE 'Reachable'
            END AS service_status
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
            .or_insert_with(Vec::new)
            .push(service);
    }

    (StatusCode::OK, Json(grouped))
}

pub(super) async fn __create_node(
    State(db_state): State<AppState>,
    Json(params): Json<payloads::CreateNode>,
)->Result<(StatusCode,Json<get_payload::AuthToken>),StatusCode>{

    let token = Uuid::new_v4().to_string();
    let create=sqlx::query(
        "INSERT INTO nodes (name,token) VALUES
        ($1,$2);
        "
    ).bind(params.name)
    .bind(&token)
    .execute(&db_state.db).await;
    
    if create.is_ok(){
        return Ok((StatusCode::OK,Json(get_payload::AuthToken{token})));
    }
    Err(StatusCode::CONFLICT)
}