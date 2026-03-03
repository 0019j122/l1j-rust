use tokio::sync::mpsc;
use crate::models::item::Inventory;

#[derive(Debug, Clone)]
pub struct OnlinePlayer {
    pub object_id: i32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub map_id: i16,
    pub level: i32,
    pub hp: i32,
    pub mp: i32,
    pub move_speed: i32,
    pub heading: i32,
    pub lawful: i32,
    pub sex: i32,
    pub title: String,
    pub gfx_id: i32,        // 確保是 gfx_id
    pub char_type: i32,     // 確保是 char_type
    pub clan_name: String,  // 確保是 clan_name
    pub online: bool,
    pub inventory: Inventory, 
    pub packet_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,
}