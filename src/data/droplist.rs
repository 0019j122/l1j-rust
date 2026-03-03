use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rand::Rng; 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropItem {
    pub mob_id: i32,
    pub item_id: i32,
    pub min_count: i32,
    pub max_count: i32,
    pub chance: i32, 
}

pub struct DropTable {
    pub data: HashMap<i32, Vec<DropItem>>,
}

impl DropTable {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    /// 從資料庫載入掉寶表 (伺服器啟動時呼叫一次)
    pub async fn load_from_db(pool: &sqlx::MySqlPool) -> anyhow::Result<Self> {
        let mut table = Self::new();
        
        let rows = sqlx::query!(
            "SELECT mobId, itemId, min, max, chance FROM droplist"
        )
        .fetch_all(pool)
        .await?;

        for row in rows {
            let item = DropItem {
                mob_id: row.mobId,
                item_id: row.itemId,
                min_count: row.min,
                max_count: row.max,
                chance: row.chance,
            };
            table.data.entry(item.mob_id).or_insert_with(Vec::new).push(item);
        }

        tracing::info!("Successfully loaded {} unique mob drop lists.", table.data.len());
        Ok(table)
    }

    pub fn get_drops(&self, mob_id: i32) -> Option<&Vec<DropItem>> {
        self.data.get(&mob_id)
    }

    /// 核心算法：決定怪物這次噴了什麼 (戰鬥時呼叫)
    pub fn calculate_drops(&self, mob_id: i32) -> Vec<(i32, i32)> {
        let mut results = Vec::new();
        let mut rng = rand::rng(); 

        if let Some(possible_drops) = self.get_drops(mob_id) {
            for drop in possible_drops {
                let roll = rng.gen_range(1..=1000000);
                if roll <= drop.chance {
                    let count = if drop.min_count == drop.max_count {
                        drop.min_count
                    } else {
                        rng.gen_range(drop.min_count..=drop.max_count)
                    };
                    results.push((drop.item_id, count));
                }
            }
        }
        results
    }
}