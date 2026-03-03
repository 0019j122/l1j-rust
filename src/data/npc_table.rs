use sqlx::MySqlPool;
use std::collections::HashMap;
use crate::models::npc::NpcTemplate;
use tracing::info; // 確保有引用日誌，方便除錯

pub struct NpcTable {
    pub templates: HashMap<i32, NpcTemplate>,
}

impl NpcTable {
    /// 這是一個非同步函數，用來從資料庫初始化所有 NPC 模板
    pub async fn load_all(pool: &MySqlPool) -> anyhow::Result<Self> {
        // 👇 所有的 let 指令都必須待在 fn 裡面
        let rows = sqlx::query_as::<_, NpcTemplate>("SELECT * FROM npc")
            .fetch_all(pool)
            .await?;

        let mut templates = HashMap::new();
        for row in rows {
            // 恢復成這行
templates.insert(row.npc_id, row);
        }

        info!("成功載入 {} 筆 NPC 模板", templates.len());
        Ok(Self { templates })
    }
}