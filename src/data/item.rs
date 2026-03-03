use std::collections::HashMap;
use sqlx::MySqlPool;
use crate::models::item::ItemTemplate; // 👈 統一使用這個，不要在下面重複定義

pub struct ItemTable {
    pub templates: HashMap<i32, ItemTemplate>,
}

impl ItemTable {
    pub fn new() -> Self {
        Self { templates: HashMap::new() }
    }

    /// 從資料庫載入所有物品資料 (etcitem, weapon, armor)
    pub async fn load_from_db(pool: &MySqlPool) -> anyhow::Result<Self> {
        let mut table = Self::new();
        
        // 1. etcitem
        // 使用 query! 宏時，SQL 的 alias (as) 會直接對應到 row 的欄位名
        let etc_rows = sqlx::query!(
            "SELECT item_id, name, invgfx as ground_gfx, weight, 0 as stackable FROM etcitem"
        ).fetch_all(pool).await?;
        
        for row in etc_rows {
            table.templates.insert(row.item_id, ItemTemplate {
                item_id: row.item_id,
                name: row.name,
                ground_gfx: row.ground_gfx.unwrap_or(0),
                weight: row.weight,
                stackable: row.stackable != 0,
                ..Default::default() // 👈 這要求 models/item.rs 裡的 ItemTemplate 必須有 #[derive(Default)]
            });
        }

        // 3. armor (示範如何對齊欄位)
        let armor_rows = sqlx::query!(
            "SELECT item_id, name, invgfx as ground_gfx FROM armor"
        ).fetch_all(pool).await?;

        for row in armor_rows {
            table.templates.insert(row.item_id, ItemTemplate {
                item_id: row.item_id,
                name: row.name,
                ground_gfx: row.ground_gfx.unwrap_or(0),
                ..Default::default()
            });
        }

        tracing::info!("Successfully loaded {} item templates.", table.templates.len());
        Ok(table)
    }

    pub fn get(&self, item_id: i32) -> Option<&ItemTemplate> {
        self.templates.get(&item_id)
    }
}

/// 怪物死亡與掉寶處理系統
pub async fn process_npc_death(
    world: &mut crate::network::shared_state::WorldState,
    drop_table: &crate::data::droplist::DropTable,
    item_table: &ItemTable,
    npc_id: i32,
) {
    if let Some(npc) = world.handle_npc_death(npc_id) {
        let drops = drop_table.calculate_drops(npc.template.npc_id);
        
        for (item_id, count) in drops {
            if let Some(item_temp) = item_table.get(item_id) {
                // 生成地上物品數據
                let obj_id = world.spawn_item_on_ground(
                    item_id, 
                    count, 
                    npc.x, 
                    npc.y, 
                    npc.map_id
                );

                // 構造 S_DROPITEM 封包
                let packet = crate::protocol::server::combat::build_drop_item(
                    obj_id,
                    npc.x,
                    npc.y,
                    item_temp.ground_gfx,
                    count
                );
                
                // 廣播給附近玩家 (18格內)
                world.broadcast_nearby(npc.map_id as u32, npc.x, npc.y, packet);

                tracing::info!("Monster {} dropped {} (x{}), ID: {}", 
                    npc.template.nameid, item_temp.name, count, obj_id);
            }
        }
    }
}