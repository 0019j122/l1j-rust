use anyhow::{Context, Result};
use sqlx::{MySqlPool, Row};
use tracing::info;
use crate::models::npc::SpawnInfo;

pub async fn load_spawn_table(pool: &MySqlPool) -> Result<Vec<SpawnInfo>> {
    let rows = sqlx::query(
        "SELECT `id`, `count`, `npc_templateid`, `locx`, `locy`, `mapid`, `heading`, \
         `randomx`, `randomy`, `min_respawn_delay`, `max_respawn_delay`, \
         `movement_distance` \
         FROM `spawnlist`"
    )
    .fetch_all(pool)
    .await
    .context("載入怪物出生點失敗")?;

    let mut spawns = Vec::with_capacity(rows.len());
    for row in &rows {
        spawns.push(SpawnInfo {
            spawn_id: row.get(0),  // id (i32)
            count: row.get(1),
            npc_template_id: row.get(2),
            loc_x: row.get(3),
            loc_y: row.get(4),
            map_id: row.get(5),
            heading: row.get(6),
            randomx: row.get(7),
            randomy: row.get(8),
            min_respawn_delay: row.get(9),
            max_respawn_delay: row.get(10),
            movement_distance: row.get(11),
        });
    }

    info!("載入怪物出生點: {} 個", spawns.len());
    Ok(spawns)
}

pub async fn load_npc_spawn_table(pool: &MySqlPool) -> Result<Vec<SpawnInfo>> {
    let rows = sqlx::query(
        "SELECT `id`, `count`, `npc_templateid`, `locx`, `locy`, `mapid`, `heading`, \
         `randomx`, `randomy`, `min_respawn_delay`, `max_respawn_delay`, \
         `movement_distance` \
         FROM `npcspawnlist`"
    )
    .fetch_all(pool)
    .await
    .context("載入 NPC 出生點失敗")?;

    let mut spawns = Vec::with_capacity(rows.len());
    for row in rows {
        spawns.push(SpawnInfo {
            spawn_id: row.get("id"),
            count: row.get("count"),
            npc_template_id: row.get("npc_templateid"),
            loc_x: row.get("locx"),
            loc_y: row.get("locy"),
            map_id: row.get::<i16, _>("mapid"), // 強制指定為 i16
            heading: row.get("heading"),
            randomx: row.get("randomx"),
            randomy: row.get("randomy"),
            min_respawn_delay: row.get("min_respawn_delay"),
            max_respawn_delay: row.get("max_respawn_delay"),
            movement_distance: row.get("movement_distance"),
        });
    }

    info!("載入 NPC 出生點: {} 個", spawns.len());
    Ok(spawns)
}