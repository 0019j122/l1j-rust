#[derive(Debug, Clone)]
pub struct AccountData {
    pub login: String,
    pub password: String,
    pub access_level: i32,
    pub online: i32,
    pub banned: i32,
    pub character_slot: i32,
    pub online_status: i32,
    pub birthday: i32, // <--- 檢查這行是否存在
}


pub async fn load_account(pool: &MySqlPool, login: &str) -> Result<Option<AccountData>> {
    // 這裡的 tuple 增加一個 i32 來接收 birthday
    let row: Option<(String, String, i32, i32, i32, i32, i32, i32)> = sqlx::query_as(
        "SELECT login, password, \
         CAST(access_level AS SIGNED), \
         CAST(online AS SIGNED), \
         CAST(banned AS SIGNED), \
         CAST(character_slot AS SIGNED), \
         CAST(OnlineStatus AS SIGNED), \
         CAST(birthday AS SIGNED) \
         FROM accounts WHERE login = ? LIMIT 1"
    )
    .bind(login)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| AccountData {
        login: r.0,
        password: r.1,
        access_level: r.2,
        online: r.3,
        banned: r.4,
        character_slot: r.5,
        online_status: r.6,
        birthday: r.7, // <--- 補上這一行，對應 SQL 的第 8 個欄位
        // 這裡一定要對應到 r.7 (也就是 SQL 裡的第 8 個欄位)
        // 你的 AccountData 結構體裡如果沒有 birthday 欄位，記得也要補上去
    }))
    
}

pub async fn set_online(pool: &MySqlPool, login: &str, ip: &str) -> Result<()> {
    sqlx::query("UPDATE accounts SET online = 1, ip = ?, lastactive = NOW() WHERE login = ?")
        .bind(ip)     // 第一個 ?
        .bind(login)  // 第二個 ?
        .execute(pool)
        .await?;
    Ok(())
}
pub async fn set_offline(pool: &MySqlPool, login: &str) -> Result<()> {
    sqlx::query("UPDATE accounts SET online = 0, OnlineStatus = 0 WHERE login = ?")
        .bind(login)
        .execute(pool)
        .await?;
    Ok(())
}