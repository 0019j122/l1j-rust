use sqlx::FromRow; // <--- 補上這行
use anyhow::Result;
use sqlx::MySqlPool;
use sqlx::Row; // <--- 補上這行，讓我們可以使用 row.get() 方法來讀取資料庫欄位值

/// Character data from the `characters` MySQL table.
/// Contains only the fields needed for the character list screen.
#[derive(Debug, Clone, FromRow)] // <--- 這裡也要加上 FromRow
pub struct CharacterListData {
    pub objid: i32,
    pub char_name: String,
    pub clanname: String,
    pub char_type: i32,
    pub sex: i32,
    pub lawful: i32,
    pub cur_hp: i32,
    pub cur_mp: i32,
    pub ac: i32,
    pub level: i32,
    // --- 這裡開始是重點 ---
    #[sqlx(rename = "str")]  // 告訴程式：資料庫欄位其實叫 str
    pub str_stat: i32,
    #[sqlx(rename = "dex")]
    pub dex_stat: i32,
    #[sqlx(rename = "con")]
    pub con_stat: i32,
    #[sqlx(rename = "wis")]
    pub wis_stat: i32,
    #[sqlx(rename = "cha")]
    pub cha_stat: i32,
    #[sqlx(rename = "intel")] // 舊版資料庫通常叫 intel 或 intel_stat
    pub int_stat: i32,
    pub access_level: i32,
    pub birthday: i32,
}

/// Character data loaded for entering the game world.
#[derive(Debug, Clone, FromRow)]
pub struct CharacterFullData {
    pub objid: i32,
    pub char_name: String,
    pub account_name: String,
    pub clanname: String,
    pub clanid: i32,
    pub char_type: i32,
    pub sex: i32,
    pub lawful: i32,
    pub cur_hp: i32,
    pub max_hp: i32,
    pub cur_mp: i32,
    pub max_mp: i32,
    pub ac: i32,
    pub level: i32,
    pub exp: i32,
    // --- 這裡開始是重點 ---
    #[sqlx(rename = "str")]  // 告訴程式：資料庫欄位其實叫 str
    pub str_stat: i32,
    #[sqlx(rename = "dex")]
    pub dex_stat: i32,
    #[sqlx(rename = "con")]
    pub con_stat: i32,
    #[sqlx(rename = "wis")]
    pub wis_stat: i32,
    #[sqlx(rename = "cha")]
    pub cha_stat: i32,
    #[sqlx(rename = "intel")] // 舊版資料庫通常叫 intel 或 intel_stat
    pub int_stat: i32,
    pub loc_x: i32,
    pub loc_y: i32,
    pub map_id: i32,
    pub heading: i32,
    pub access_level: i32,
    pub gfxid: i32,
    pub food: i32,
    pub mons_kill: i32,
}

/// Load character list for account (character select screen).
pub async fn load_char_list(
    pool: &MySqlPool,
    account_name: &str,
) -> Result<Vec<CharacterListData>> {
    let rows = sqlx::query(
        "SELECT CAST(objid AS SIGNED), char_name, IFNULL(Clanname,''), \
         CAST(Type AS SIGNED), CAST(Sex AS SIGNED), CAST(Lawful AS SIGNED), \
         CAST(CurHp AS SIGNED), CAST(CurMp AS SIGNED), CAST(Ac AS SIGNED), \
         CAST(level AS SIGNED), CAST(Str AS SIGNED), CAST(Dex AS SIGNED), \
         CAST(Con AS SIGNED), CAST(Wis AS SIGNED), CAST(Cha AS SIGNED), \
         CAST(Intel AS SIGNED), CAST(AccessLevel AS SIGNED), CAST(IFNULL(birthday,0) AS SIGNED) \
         FROM characters WHERE account_name = ? ORDER BY objid",
    )
    .bind(account_name)
    .fetch_all(pool)
    .await?;

    let mut chars = Vec::with_capacity(rows.len());
    for r in &rows {
        chars.push(CharacterListData {
            objid: r.get(0),
            char_name: r.get(1),
            clanname: r.get(2),
            char_type: r.get(3),
            sex: r.get(4),
            lawful: r.get(5),
            cur_hp: r.get(6),
            cur_mp: r.get(7),
            ac: r.get(8),
            level: r.get(9),
            str_stat: r.get(10),
            dex_stat: r.get(11),
            con_stat: r.get(12),
            wis_stat: r.get(13),
            cha_stat: r.get(14),
            int_stat: r.get(15),
            access_level: r.get(16),
            birthday: r.get(17),
        });
    }
    Ok(chars)
}

/// Load full character data for entering the game world.
pub async fn load_character(
    pool: &MySqlPool,
    char_name: &str,
    account_name: &str,
) -> Result<Option<CharacterFullData>> {
    let row = sqlx::query(
        "SELECT CAST(objid AS SIGNED), char_name, account_name, IFNULL(Clanname,''), CAST(IFNULL(ClanID,0) AS SIGNED), \
         CAST(Type AS SIGNED), CAST(Sex AS SIGNED), CAST(Lawful AS SIGNED), \
         CAST(CurHp AS SIGNED), CAST(MaxHp AS SIGNED), CAST(CurMp AS SIGNED), CAST(MaxMp AS SIGNED), \
         CAST(Ac AS SIGNED), CAST(level AS SIGNED), CAST(Exp AS SIGNED), \
         CAST(Str AS SIGNED), CAST(Dex AS SIGNED), CAST(Con AS SIGNED), \
         CAST(Wis AS SIGNED), CAST(Cha AS SIGNED), CAST(Intel AS SIGNED), \
         CAST(LocX AS SIGNED), CAST(LocY AS SIGNED), CAST(MapID AS SIGNED), \
         CAST(Heading AS SIGNED), CAST(AccessLevel AS SIGNED), \
         CAST(IFNULL(Food, 40) AS SIGNED) \
         FROM characters WHERE char_name = ? AND account_name = ? LIMIT 1",
    )
    .bind(char_name)
    .bind(account_name)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| {
        let char_type: i32 = r.get(5);
        let sex: i32 = r.get(6);
        let gfxid = crate::protocol::client::char_create::get_gfx_id(char_type, sex);
        CharacterFullData {
            objid: r.get(0),
            char_name: r.get(1),
            account_name: r.get(2),
            clanname: r.get(3),
            clanid: r.get(4),
            char_type,
            sex,
            lawful: r.get(7),
            cur_hp: r.get(8),
            max_hp: r.get(9),
            cur_mp: r.get(10),
            max_mp: r.get(11),
            ac: r.get(12),
            level: r.get(13),
            exp: r.get(14),
            str_stat: r.get(15),
            dex_stat: r.get(16),
            con_stat: r.get(17),
            wis_stat: r.get(18),
            cha_stat: r.get(19),
            int_stat: r.get(20),
            loc_x: r.get(21),
            loc_y: r.get(22),
            map_id: r.get(23),
            heading: r.get(24),
            access_level: r.get(25),
            gfxid,
            food: r.get(26),
            mons_kill: 0,
        }
    }))
}

/// Count characters for an account.
pub async fn count_characters(pool: &MySqlPool, account_name: &str) -> Result<i64> {
    let (count,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM characters WHERE account_name = ?")
            .bind(account_name)
            .fetch_one(pool)
            .await?;
    Ok(count)
}
