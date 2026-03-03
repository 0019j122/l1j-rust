use sqlx::FromRow; 

#[derive(Debug, Clone, FromRow)] // 確保每個要從資料庫讀取的 struct 都有這個
pub struct Health {
    pub cur_hp: i32,
    pub max_hp: i32,
    pub cur_mp: i32,
    pub max_mp: i32,
}

/// Combat stats component.
#[derive(Debug, Clone, FromRow)] 
pub struct CombatStats {
    pub level: i32,
    #[sqlx(rename = "str")]  
    pub str_stat: i32,
    #[sqlx(rename = "dex")]
    pub dex_stat: i32,
    #[sqlx(rename = "con")]
    pub con_stat: i32,
    #[sqlx(rename = "wis")]
    pub wis_stat: i32,
    #[sqlx(rename = "cha")]
    pub cha_stat: i32,
    #[sqlx(rename = "intel")] 
    pub int_stat: i32,
    pub ac: i32,
    pub mr: i32,
    pub exp: i32,
    pub lawful: i32,
}

// src/ecs/components/stats.rs

#[derive(Debug, Clone, sqlx::FromRow)] // 確保有 FromRow
pub struct Speed {
    // 修正：資料庫通常叫 passispeed
    #[sqlx(rename = "passispeed")] 
    pub move_speed: i32,     
    
    // 修正：資料庫通常叫 atkspeed
    #[sqlx(rename = "atkspeed")]   
    pub atk_speed: i32,       
    
    // 修正：這個欄位也檢查一下，如果資料庫沒底線就改成 atkmagicspeed
    #[sqlx(rename = "atkmagicspeed")] 
    pub atk_magic_speed: i32,
    
    #[sqlx(default)] 
    pub brave_speed: i32,
}