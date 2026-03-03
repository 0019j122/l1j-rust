use crate::config::ServerConfig;
use crate::data::item_table::ItemTable;
use crate::protocol::packet::PacketBuilder;
use crate::protocol::opcodes::server;
use anyhow::{Result, Context};
use sqlx::MySqlPool;

pub async fn give_starter_items(
    pool: &MySqlPool,
    item_table: &ItemTable,
    player_tx: &tokio::sync::mpsc::UnboundedSender<Vec<u8>>,
    char_id: i32,
    config: &ServerConfig,
) -> Result<()> {
    println!("🎁 開始發放新手物資給角色 ID: {}", char_id);

    for starter in &config.starter_gear.items {
        // 1. 檢查物品是否存在於模板
        let template = item_table.templates.get(&starter.item_id)
            .context("資料庫找不到該物品模板")?;

        // 2. 寫入資料庫 (持久化)
        // 注意：這裡假設你用自增 ID 或由資料庫生成唯一的 ID
        let result = sqlx::query!(
            "INSERT INTO character_items (char_objid, item_id, count, enchant_level) VALUES (?, ?, ?, ?)",
            char_id,
            starter.item_id,
            starter.count,
            starter.enchant_level
        )
        .execute(pool)
        .await?;

        let new_obj_id = result.last_insert_id() as i32;

        // 3. 構建封包通知客戶端 (S_INVLIST / S_ADD_INVENTORY_ITEM)
        // 天堂的 S_INVLIST 封包格式通常包含非常多欄位：
        // [ID][Name][Type][Status][Count][Enchant]...
        let packet = PacketBuilder::new(server::S_OPCODE_INVLIST)
            .write_d(new_obj_id)       // Object ID
            .write_h(starter.item_id)   // Item ID / GFX ID
            .write_c(0)                // 是否已鑑定
            .write_d(starter.count as i32)              // 數量轉換為 i32
            .write_c(starter.enchant_level as i32)      // 強化等級轉換為 i32
            .build();

        // 4. 發送封包
        player_tx.send(packet).map_err(|e| anyhow::anyhow!("發送物品封包失敗: {}", e))?;
        
        println!("  - 已成功發送物品: {} (ID: {})", template.name, starter.item_id);
    }

    Ok(())
}