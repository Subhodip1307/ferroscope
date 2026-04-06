use super::payloads;
use super::response as get_payload;
use crate::objects::AppState;
use axum::{
    Json,
    extract::{State},
    http::StatusCode,
};
use uuid::Uuid;


pub(super) async fn __create_node(
    State(db_state): State<AppState>,
    Json(params): Json<payloads::CreateNode>,
) -> Result<(StatusCode, Json<get_payload::AuthToken>), StatusCode> {
    let token = Uuid::new_v4().to_string();
    let create: Result<sqlx::postgres::PgQueryResult, sqlx::Error> = sqlx::query(
        "INSERT INTO nodes (name,token) VALUES
        ($1,$2);
        ",
    )
    .bind(params.name)
    .bind(&token)
    .execute(&db_state.db)
    .await;

    if create.is_ok() {
        return Ok((StatusCode::OK, Json(get_payload::AuthToken { token })));
    }
    Err(StatusCode::CONFLICT)
}


pub (super) async  fn __remove_node(
 State(db_state): State<AppState>,
 Json(params): Json<payloads::IdQuery>,
)->StatusCode{
    let _=sqlx::query("delete from nodes where id =$1")
    .bind(params.node)
    .execute(&db_state.db).await;
    StatusCode::OK
}


pub (super) async fn __create_notification_rules(
    // State(db_state): State<AppState>,
    Json(data):Json<payloads::RulesData>
)->StatusCode{
    println!("data is {:?}",data);
    StatusCode::OK
}