use anyhow::{Context, Result};
use tracing::info; // 暫時拿掉 warn，減少 unused warning
use dotenv::dotenv;

// 🔹 統一從根目錄引用，這樣類型就會跟 start 函數要求的完全一致
use l1j_rust::{ServerConfig, config::load_config}; 
use l1j_rust::db::init_db_pool;
use l1j_rust::data::npc_table::NpcTable;
use l1j_rust::data::spawn_table::load_npc_spawn_table;
use l1j_rust::network::shared_state::new_shared_world;
use l1j_rust::network::listener::start;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // 1. 載入設定
    let server_config = load_config().context("讀取設定檔失敗")?;

    // 2. 初始化資料庫
    let db_pool = init_db_pool(&server_config.database.url)
        .await
        .context("資料庫連線池建立失敗")?;

    // 3. 建立全域世界狀態
    let world_shared = new_shared_world();

    // 4. 載入 NPC 模板 (注意這裡用 load_all)
    let _npc_templates = NpcTable::load_all(&db_pool).await?;

    // 5. 載入並生成 NPC (修正 map_id 的 .into())
    let spawn_list = load_npc_spawn_table(&db_pool).await?;
    {
        let mut world = world_shared.lock().unwrap();
        for spawn in spawn_list {
            world.spawn_to_battlefield(
                spawn.npc_template_id, 
                spawn.loc_x, 
                spawn.loc_y, 
                spawn.map_id.into() // 👈 i16 轉 i32
            );
        }
    }

    info!("伺服器準備就緒，啟動網路監聽器...");

    // 6. 啟動伺服器 (確保傳入的是正確的 server_config)
    start(server_config, Some(db_pool), world_shared).await?;

    Ok(())
}