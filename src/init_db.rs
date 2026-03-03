use anyhow::{Context, Result};
use sqlx::{Executor, MySql, Pool};
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("❌ 請在 .env 設定 DATABASE_URL");
    let pool = Pool::<MySql>::connect(&db_url).await?;

    println!("🏗️  正在為您建立 3.80c 核心資料表基礎結構...");

    // 🔹 建立核心表格清單 (如果不存在就建立)
    // 這裡我們先建立最容易報錯的幾個核心表格
    let core_tables = [
        "CREATE TABLE IF NOT EXISTS `area` (`id` int NOT NULL, PRIMARY KEY (`id`)) ENGINE=InnoDB;",
        "CREATE TABLE IF NOT EXISTS `npc` (`npcid` int NOT NULL, `name` varchar(255), PRIMARY KEY (`npcid`)) ENGINE=InnoDB;",
        "CREATE TABLE IF NOT EXISTS `weapon` (`id` int NOT NULL, `name` varchar(255), PRIMARY KEY (`id`)) ENGINE=InnoDB;",
        "CREATE TABLE IF NOT EXISTS `armor` (`id` int NOT NULL, `name` varchar(255), PRIMARY KEY (`id`)) ENGINE=InnoDB;",
        "CREATE TABLE IF NOT EXISTS `droplist` (`mobId` int NOT NULL, `itemId` int NOT NULL) ENGINE=InnoDB;",
        "CREATE TABLE IF NOT EXISTS `spawnlist` (`id` int NOT NULL AUTO_INCREMENT, PRIMARY KEY (`id`)) ENGINE=InnoDB;",
        "CREATE TABLE IF NOT EXISTS `etcitem` (`item_id` int NOT NULL, `name` varchar(255), PRIMARY KEY (`item_id`)) ENGINE=InnoDB;",
        "CREATE TABLE IF NOT EXISTS `mapids` (`mapid` int NOT NULL, PRIMARY KEY (`mapid`)) ENGINE=InnoDB;",
    ];

    for table_sql in core_tables {
        let _ = pool.execute(table_sql).await;
    }

    println!("🏠 基礎結構已準備，開始搬運 6 萬筆家具...");

    // 🔹 讀取並修正內容
    let sql_content = fs::read_to_string("l1jdb.txt").context("❌ 找不到 l1jdb.txt")?;
    let fixed_sql = sql_content.replace("REPLAREPLACE", "REPLACE");

    let mut success = 0;
    let mut fail = 0;

    // 逐條執行，確保某一條出錯不會卡死全部
    for (i, statement) in fixed_sql.split(';').enumerate() {
        let cmd = statement.trim();
        if cmd.is_empty() || cmd.starts_with("--") {
            continue;
        }

        match pool.execute(cmd).await {
            Ok(_) => success += 1,
            Err(_) => fail += 1,
        }

        if (success + fail) % 2000 == 0 {
            println!("🚜 搬運進度: {} / 60000...", success + fail);
        }
    }

    println!("========================================");
    println!("✨ ✅ 建築完成！成功：{} 條 / 失敗：{}", success, fail);
    println!("========================================");
    println!("💡 提示：如果失敗數仍多，代表 SQL 檔案內缺少 CREATE TABLE 語句，");
    println!("建議從網路上找一份完整的 3.80c 結構檔 (.sql) 先行導入。");
    Ok(())
}
