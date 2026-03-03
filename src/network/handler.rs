use crate::config::ServerConfig;
use anyhow::Result;
use sqlx::MySqlPool;

pub async fn give_starter_items(
    pool: &MySqlPool, // 這裡必須是 &MySqlPool，不能是 Option
    char_id: i32,
    config: &ServerConfig,
) -> Result<()> {
    // 這裡要用到這些變數，警告就會消失
    for item in &config.starter_gear.items {
        sqlx::query!(
            "INSERT INTO character_items (char_objid, item_id, count) VALUES (?, ?, ?)",
            char_id,
            item.item_id,
            item.count
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}