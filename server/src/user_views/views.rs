use super::payloads;
use super::response as get_payload;
use crate::objects::AppState;
use axum::{Json,extract::{Query, State},http::StatusCode};
use chrono::{DateTime, Utc};
use sqlx::Row;
use axum::http::HeaderMap;
use super::auth::auth;

pub(super) async fn __get_node_list(
    State(db_state): State<AppState>,headers: HeaderMap
) -> Result<(StatusCode, Json<Vec<get_payload::NodesList>>),StatusCode> {
    let (is_auth, _) = auth(headers, db_state.clone()).await;
    if !is_auth {
        return Err(StatusCode::UNAUTHORIZED);
        // auth token didn't matched, still sending 200
    }   
    let rows: Vec<get_payload::NodesList> = sqlx::query_as("SELECT id,name FROM nodes")
        .fetch_all(&db_state.db)
        .await
        .unwrap();
    Ok((StatusCode::OK, Json(rows)))
}


pub(super) async fn __get_nodeinfo(//sysinfo
    State(db_state): State<AppState>,
    Query(params): Query<payloads::IdQuery>,
) -> Result<(StatusCode, Json<get_payload::SysInfo>),StatusCode> {
    let row = sqlx::query_as::<_,get_payload::SysInfo>(
        "SELECT system_name,kernel_version,os_version,uptime,cpu_threads,cpu_vendor FROM sysinfo where node_id = $1",
    )
    .bind(params.node)
    .fetch_optional(&db_state.db)
    .await.unwrap();
    
    if let Some(item)=row{
        return Ok((
            StatusCode::OK,
            Json(item)
        ));
    };
    Err(StatusCode::NO_CONTENT)
}


pub(super) async fn __get_latest_cpu(
    State(db_state): State<AppState>,
    Query(params): Query<payloads::IdQuery>,
) -> Result<(StatusCode, Json<get_payload::LatestCpu>),StatusCode> {
    let row = sqlx::query(
        "SELECT value,date_time FROM cpu_stats where node_id = $1 ORDER BY date_time DESC LIMIT 1",
    )
    .bind(params.node)
    .fetch_optional(&db_state.db)
    .await.unwrap();
    
    if let Some(item)=row{
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
) -> (StatusCode, Json<Vec<get_payload::ServiceList>>) {
    let row: Vec<get_payload::ServiceList> =
        sqlx::query_as("SELECT  DISTINCT service_name FROM service_monitor where node_id = $1")
            .bind(params.node)
            .fetch_all(&db_state.db)
            .await
            .unwrap();
    (StatusCode::OK, Json(row))
}

pub(super) async fn __get_single_service_current_status(
    State(db_state): State<AppState>,
    Json(payload): Json<payloads::ServiceQuery>,
) -> Result<(StatusCode, Json<get_payload::SingleServiceStatus>), StatusCode> {
    // will get node id and service name from query parameter or from json payload then the responce will be returnd
    let row=sqlx::query(
        "SELECT  error_msg,status,
            CASE WHEN NOW() - updated_at - INTERVAL '5 minutes'
            THEN 'Unreachable'
            ELSE 'Reachable'
            END AS service_status
         FROM service_monitor where node_id = $1 and service_name= $2"
    
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
        return Ok((
            StatusCode::OK,
            Json(get_payload::SingleServiceStatus {
                status,error_msg,
                service_status
            }),
        ));
    }
    Err(StatusCode::NO_CONTENT)
}


pub(super) async fn __get_service_current_status(
    State(db_state): State<AppState>,
    Query(params): Query<payloads::IdQuery>,
) -> (StatusCode, Json<Vec<get_payload::ServiceStatus>>){
    let row=sqlx::query_as::<_,get_payload::ServiceStatus>(
        "SELECT  error_msg,status,
            CASE WHEN NOW() - updated_at - INTERVAL '5 minutes'
            THEN 'Unreachable'
            ELSE 'Reachable'
            END AS service_status
         FROM service_monitor where node_id = $1 "
    
        )
            .bind(params.node)
            .fetch_all(&db_state.db)
            .await
            .unwrap();
    
    (
            StatusCode::OK,
            Json(row),
    )
}