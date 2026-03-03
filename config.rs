use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Item {
    pub item_id: i32,
    pub gfx_id: Option<i32>,
    pub count: u32,
    pub enchant_level: u8,
}

// 🔹 補上這個，不然 listener.rs 會報錯說找不到 host/port
#[derive(Deserialize, Debug, Clone)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug, Clone)] // 🔹 加上 Clone
pub struct GameRates {
    pub exp: f64,
    pub gold_drop: f64,
    pub baby_rate: f64,
}

#[derive(Deserialize, Debug, Clone)] // 🔹 加上 Clone
pub struct StarterGear {
    pub items: Vec<Item>,
}

#[derive(Deserialize, Debug, Clone)] // 🔹 加上 Clone
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)] // 🔹 關鍵！加上 Clone，解決 E0599
pub struct ServerConfig {
    pub server: Server, // 🔹 補上這行
    pub database: DatabaseConfig,
    pub game_rates: GameRates,
    pub starter_gear: StarterGear,
}

// 在 src/config.rs 的最後面補上這段：

pub fn load_config() -> anyhow::Result<ServerConfig> {
    let config_path = "config.toml";
    
    // 讀取檔案內容
    let content = std::fs::read_to_string(config_path)
        .map_err(|e| anyhow::anyhow!("找不到 {}: {}", config_path, e))?;
    
    // 將 TOML 轉成 ServerConfig 結構
    let config: ServerConfig = toml::from_str(&content)
        .map_err(|e| anyhow::anyhow!("設定檔格式解析失敗 (TOML error): {}", e))?;
    
    Ok(config)
}