pub mod config;
pub mod db;
pub mod data;
pub mod models;
pub mod network;
pub mod protocol; // 👈 確保這行一定要有，不然 50 個錯誤都跟這行有關

// 🔹 關鍵：把 ServerConfig 重新導出到根目錄
pub use crate::config::{ServerConfig, load_config}; // 🔹 確保 load_config 也有被 pub use
pub use crate::network::shared_state::SharedWorld;

// pub mod ecs; // 暫時封印

pub const DEFAULT_CHARACTER_SLOT: u8 = 8;

use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameConfig {
    pub name: String,
}