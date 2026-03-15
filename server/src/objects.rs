use crate::user_views::{LatestCpu, LatestRam};
use dashmap::DashMap;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::watch::Sender;
use mini_moka::sync::Cache;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub cpu_strem: Arc<DashMap<String, Sender<LatestCpu>>>,
    pub ram_strem: Arc<DashMap<String, Sender<LatestRam>>>,
    pub cache: Cache<String, i64>, //cache to store i64
}
