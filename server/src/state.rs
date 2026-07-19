#[cfg(not(debug_assertions))]
use crate::env;
use crate::user_views::{LatestCpu, LatestRam, NodeDiskIoStats};
use dashmap::DashMap;
use ferroscope_server::global::structure::NotificationData;
use mini_moka::sync::Cache;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::watch::Sender;

#[derive(Clone)]
pub enum StreamPayLoad {
    Cpu(LatestCpu),
    Ram(LatestRam),
    Disk(Arc<Vec<NodeDiskIoStats>>),
}

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub stream_data: Arc<DashMap<String, Sender<StreamPayLoad>>>,
    pub helth_check: Arc<DashMap<i64, u64>>,
    pub cache: Cache<String, i64>, //cache to store i64
    pub notifier: mpsc::Sender<NotificationData>,
}

impl AppState {
    pub async fn new(notifier: mpsc::Sender<NotificationData>) -> Self {
        // cache for user auth
        let cache: Cache<String, i64> = Cache::builder()
            .max_capacity(100)
            .time_to_live(Duration::from_secs(60 * 5))
            .build();
        #[cfg(not(debug_assertions))]
        let pg_pool = PgPool::connect(&env::var("PSQL_URL").unwrap_or_default())
            .await
            .unwrap();

        #[cfg(debug_assertions)]
        let pg_pool = PgPool::connect("postgres://myuser:mypassword@127.0.0.1:5432/mydatabase")
            .await
            .unwrap();
        Self {
            db: pg_pool,
            stream_data: Arc::new(DashMap::new()),
            helth_check: Arc::new(DashMap::new()),
            cache,
            notifier,
        }
    }
}
