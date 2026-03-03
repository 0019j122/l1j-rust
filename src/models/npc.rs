use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SpawnInfo {
    #[sqlx(rename = "id")]
    pub spawn_id: i32,
    pub count: i32,
    #[sqlx(rename = "npc_templateid")]
    pub npc_template_id: i32,
    #[sqlx(rename = "locx")]
    pub loc_x: i32,
    #[sqlx(rename = "locy")]
    pub loc_y: i32,
    #[sqlx(rename = "mapid")]
    pub map_id: i16,
    pub heading: i32,
    pub randomx: i32,
    pub randomy: i32,
    pub min_respawn_delay: i32,
    pub max_respawn_delay: i32,
    pub movement_distance: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NpcTemplate {
    #[sqlx(rename = "npcid")]
    pub npc_id: i32,       // 改成 u32 來對應資料庫的 UNSIGNED INT
    pub name: String,
    pub nameid: String,
    pub note: Option<String>,
    pub level: i32,
    pub hp: i32,
    pub mp: i32,
    pub ac: i32,
    pub str: i32,
    pub con: i32,
    pub dex: i32,
    pub int: i32,
    pub wis: i32,
    pub cha: i32,
    pub gfxid: i32,
    pub light_size: i32,
    pub exp: i32,
    pub lawful: i32,
}

#[derive(Debug, Clone)]
pub struct OnlineNpc {
    pub object_id: i32,
    pub template_id: i32,
    pub x: i32,
    pub y: i32,
    pub map_id: i16,
    pub heading: i32,
    pub cur_hp: i32,       // 👈 補上這個欄位
    pub template: NpcTemplate, // 👈 補上這個欄位，存儲 NPC 的原始屬性
}