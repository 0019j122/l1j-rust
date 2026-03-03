/// Character creation DB operations.

use anyhow::Result;
use sqlx::{MySqlPool, Row}; // 這裡加入 Row
use crate::protocol::client::char_create::{self, NewChar};
use tracing::info;

pub async fn name_exists(pool: &sqlx::MySqlPool, name: &str) -> anyhow::Result<bool> {
    let row = sqlx::query("SELECT char_name FROM characters WHERE char_name = ?")
        .bind(name)
        .fetch_optional(pool)
        .await?;
    Ok(row.is_some())
}

pub async fn create_character(
    pool: &MySqlPool,
    account_name: &str,
    nc: &NewChar,
    objid: i32,
) -> Result<i32> {
    let hp = char_create::get_init_hp(nc.char_type);
    let mp = char_create::calc_init_mp(nc.char_type, nc.wis_stat);
    let now = chrono_free_birthday();

    // 1. 插入角色
    sqlx::query(
        "INSERT INTO characters SET \
         account_name=?, objid=?, char_name=?, birthday=?, level=1, HighLevel=1, \
         Exp=0, MaxHp=?, CurHp=?, MaxMp=?, CurMp=?, Ac=10, \
         Str=?, Con=?, Dex=?, Cha=?, Intel=?, Wis=?, \
         Status=0, Class=0, Sex=?, Type=?, Heading=0, \
         LocX=?, LocY=?, MapID=?, Food=40"
    )
    .bind(account_name).bind(objid).bind(&nc.name).bind(now)
    .bind(hp).bind(hp).bind(mp).bind(mp)
    .bind(nc.str_stat).bind(nc.con_stat).bind(nc.dex_stat)
    .bind(nc.cha_stat).bind(nc.int_stat).bind(nc.wis_stat)
    .bind(nc.sex).bind(nc.char_type)
    .bind(char_create::START_X).bind(char_create::START_Y).bind(char_create::START_MAP)
    .execute(pool)
    .await?;

    // 2. 獲取新手物品
    // 這裡我們用泛型查詢，避免 query! 巨集的編譯期檢查麻煩
    let rows = sqlx::query("SELECT item_id, count, enchantlvl FROM beginner_item")
        .fetch_all(pool)
        .await?;

    for row in rows {
        let item_id: i32 = row.get(0);
        let count: i32 = row.get(1);
        let enchant: i32 = row.get(2);
        // 生成唯一 ID，使用絕對值確保為正數
        let item_obj_id: i32 = rand::random::<i32>().wrapping_abs(); 

        // 插入背包時，我們多帶幾個常見的必填欄位以防萬一
        sqlx::query(
            "INSERT INTO character_items (item_obj_id, char_id, item_id, count, enchantlvl, is_equipped, is_id, duratility) \
             VALUES (?, ?, ?, ?, ?, 0, 1, 0)"
        )
        .bind(item_obj_id)
        .bind(objid)
        .bind(item_id)
        .bind(count)
        .bind(enchant)
        .execute(pool)
        .await?;
    }

    info!("角色 {} (ID: {}) 創建完畢，新手物資已放入背包", &nc.name, objid);
    Ok(objid)
}

// ... 剩下的 name_exists 和 chrono_free_birthday 保持不變 ...

/// Generate a birthday integer in yyyyMMdd format without chrono crate.
fn chrono_free_birthday() -> i32 {
    // Use system time to get approximate date
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // Rough calculation: days since epoch
    let days = secs / 86400;
    let years = 1970 + days / 365;
    let remaining = days % 365;
    let month = remaining / 30 + 1;
    let day = remaining % 30 + 1;
    (years as i32) * 10000 + (month as i32) * 100 + (day as i32)
}
