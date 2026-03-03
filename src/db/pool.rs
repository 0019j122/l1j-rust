// src/db/pool.rs
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use anyhow::Result;

// 必須有 pub，且名稱必須完全一致
pub async fn init_db_pool(db_url: &str) -> Result<MySqlPool> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;
    Ok(pool)
}