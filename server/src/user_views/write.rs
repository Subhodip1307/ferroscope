use super::payloads;
use super::response as get_payload;
use crate::objects::AppState;
use axum::{Extension, Json, extract::State, http::StatusCode};
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

pub(super) async fn __remove_node(
    State(db_state): State<AppState>,
    Json(params): Json<payloads::IdQuery>,
) -> StatusCode {
    let _ = sqlx::query("delete from nodes where id =$1")
        .bind(params.node)
        .execute(&db_state.db)
        .await;
    StatusCode::OK
}

pub(super) async fn __create_notification_rules(
    State(db_state): State<AppState>,
    Extension(auth_user): Extension<get_payload::AuthUser>,
    Json(data): Json<payloads::RulesData>,
) -> StatusCode {
    // will add payload checking code in future
    let create = sqlx::query(
        "INSERT INTO rules (name,is_active,event_type,condition_json,action_json,created_by)
        VALUES ($1,$2,$3,$4,$5,$6);
        ",
    )
    .bind(data.name)
    .bind(data.active)
    .bind(data.event_type.to_string())
    .bind(data.condition)
    .bind(data.action)
    .bind(auth_user.user_id)
    .execute(&db_state.db)
    .await;

    match create {
        Ok(_) => StatusCode::CREATED,
        Err(e) => {
            println!("{e}");
            StatusCode::BAD_REQUEST
        }
    }
}
