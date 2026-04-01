use crate::user_views::{LatestCpu, LatestRam};
use dashmap::DashMap;
use ferroscope_server::global::structure::NotificationData;
use mini_moka::sync::Cache;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::watch::Sender;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub cpu_strem: Arc<DashMap<String, Sender<LatestCpu>>>,
    pub ram_strem: Arc<DashMap<String, Sender<LatestRam>>>,
    pub helth_check: Arc<DashMap<i64, u64>>,
    pub cache: Cache<String, i64>, //cache to store i64
    pub notifier: mpsc::Sender<NotificationData>,
}

impl AppState {
    pub fn new(
        pg_pool: sqlx::Pool<sqlx::Postgres>,
        notifier: mpsc::Sender<NotificationData>,
    ) -> Self {
        // cache for user auth
        let cache: Cache<String, i64> = Cache::builder()
            .max_capacity(100)
            .time_to_live(Duration::from_secs(60 * 5))
            .build();
        Self {
            db: pg_pool,
            cpu_strem: Arc::new(DashMap::new()),
            ram_strem: Arc::new(DashMap::new()),
            helth_check: Arc::new(DashMap::new()),
            cache,
            notifier,
        }
    }
}
