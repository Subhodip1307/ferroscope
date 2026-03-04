use sqlx::PgPool;
use tokio::sync::watch::Sender;
use dashmap::{DashMap};
use std::sync::Arc;
use crate::user_views::LatestCpu;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub cpu_strem:Arc<DashMap<String,Sender<LatestCpu>>>
}
