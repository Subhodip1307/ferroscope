use super::auth::auth;
use super::objects as payload;
use crate::objects::AppState;
use axum::http::{HeaderMap, StatusCode};
use axum::{Json, extract::State};

pub async fn __system_info( headers: HeaderMap,
    State(db_state): State<AppState>,
    data: Json<payload::SysInfo>)->StatusCode{
    
    let (is_auth, nodes_id) = auth(headers, db_state.clone()).await;
    if !is_auth {
        return StatusCode::OK;
        // auth token didn't matched, still sending 200
    }   
     sqlx::query("INSERT INTO sysinfo (node_id,
                system_name,
                kernel_version,
                os_version,
                uptime,
                cpu_threads,
                cpu_vendor)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (node_id)
                DO UPDATE SET
                system_name   = EXCLUDED.system_name,
                kernel_version = EXCLUDED.kernel_version,
                os_version     = EXCLUDED.os_version,
                uptime         = EXCLUDED.uptime,
                cpu_threads    = EXCLUDED.cpu_threads,
                cpu_vendor     = EXCLUDED.cpu_vendor;
                 ")
        .bind(nodes_id)
        .bind(&data.system_name)
        .bind(&data.kernel_version)
        .bind(&data.os_version)
        .bind(data.uptime)
        .bind(data.cpu_threads)
        .bind(&data.cpu_vendor)
        .execute(&db_state.db)
        .await
        .expect("failed to insert user");
    println!("{:?}",data);
    StatusCode::OK
}



pub async fn __cpu_metrix(
    headers: HeaderMap,
    State(db_state): State<AppState>,
    data: Json<payload::CpuStats>,
) -> StatusCode {
    println!("CPU metrix");
    let (is_auth, nodes_id) = auth(headers, db_state.clone()).await;
    if !is_auth {
        return StatusCode::OK;
        // auth token didn't matched, still sending 200
    }
    sqlx::query("INSERT INTO cpu_stats (value,node_id) VALUES ($1,$2)")
        .bind(data.cpu)
        .bind(nodes_id)
        .execute(&db_state.db)
        .await
        .expect("failed to insert user");
    StatusCode::OK
}

pub async fn __memory_metrix(
    headers: HeaderMap,
    State(db_state): State<AppState>,
    data: Json<payload::MemoryStats>,
) -> StatusCode {
    println!("Memory metrix");
    let (is_auth, nodes_id) = auth(headers, db_state.clone()).await;
    if !is_auth {
        return StatusCode::OK;
        // auth token didn't matched, still sending 200
    }
    sqlx::query("INSERT INTO memory_metrics (free,total,node_id) VALUES ($1,$2,$3)")
        .bind(&data.free)
        .bind(&data.total)
        .bind(nodes_id)
        .execute(&db_state.db)
        .await
        .expect("failed to insert user");
    StatusCode::OK
}

pub async fn __service_monitor(
    headers: HeaderMap,
    State(db_state): State<AppState>,
    data: Json<payload::ServiceMonitor>,
) -> StatusCode {
    println!("Service Monitor");
    let (is_auth, nodes_id) = auth(headers, db_state.clone()).await;
    if !is_auth {
        println!("createing not data");
        return StatusCode::OK;
        // auth token didn't matched, still sending 200
    }
    sqlx::query(
        "INSERT INTO service_monitor (service_name,status,error_msg,node_id) VALUES ($1,$2,$3,$4)
        ON CONFLICT (node_id,service_name)
        DO UPDATE SET   
            status = EXCLUDED.status,
            error_msg = EXCLUDED.error_msg,
            updated_at = NOW();
        ",
    )
    .bind(&data.service_name)
    .bind(&data.status)
    .bind(&data.error_msg)
    .bind(nodes_id)
    .execute(&db_state.db)
    .await
    .expect("failed to insert user");
    StatusCode::OK
}
