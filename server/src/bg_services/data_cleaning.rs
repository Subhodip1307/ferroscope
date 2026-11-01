// clean data based on data retention
// selected tables (memory_metrics,cpu_stats)
use tokio::time::{Duration, interval};

pub async fn clean_data(pg_pool: sqlx::Pool<sqlx::Postgres>, retention_days: u16) {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(86400));
        loop {

            let _ = sqlx::query(
                "DELETE FROM memory_metrics
     WHERE date_time < NOW() - ($1 * INTERVAL '1 day')",
            )
            .bind(retention_days as i32)
            .execute(&pg_pool)
            .await;
            let _ = sqlx::query(
                "DELETE FROM cpu_stats
     WHERE date_time < NOW() - ($1 * INTERVAL '1 day')",
            )
            .bind(retention_days as i32)
            .execute(&pg_pool)
            .await;
            ticker.tick().await;
        }
    });
}
