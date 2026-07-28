use crate::config::Config;
use log::debug;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub async fn connect_db(db_url: &str, config: &mut Config) -> anyhow::Result<PgPool> {
    if config.db_conncter_max_connctions < 5 {
        config.db_conncter_max_connctions = 6;
        debug!(
            "The minimum number of database connections configured is less than the number of hard-coded, adjusted"
        )
    }
    let pool = PgPoolOptions::new()
        .max_connections(config.db_conncter_max_connctions)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(db_url)
        .await?;
    Ok(pool)
}
