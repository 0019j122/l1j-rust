use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemTemplate {
    pub item_id: i32,
    pub name: String,
    pub ground_gfx: i32,
    pub weight: i32,
    pub stackable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)] // 👈 重點是 PartialEq
pub struct OnlineItem {
    pub object_id: i32,
    pub item_id: i32,
    pub count: i32,
    pub x: i32,
    pub y: i32,
    pub map_id: i16,
    pub is_equipped: bool, // 👈 補上這個欄位，否則編譯器找不到它
}

#[derive(Debug, Clone, PartialEq)]
pub struct Inventory {
    pub items: Vec<OnlineItem>,
    pub max_size: usize,
    pub max_weight: i32,
}

pub type ItemInstance = OnlineItem; 

impl OnlineItem {
    pub fn new(object_id: i32, item_id: i32) -> Self {
        Self {
            object_id,
            item_id,
            count: 1,
            x: 0,
            y: 0,
            map_id: 0,
            is_equipped: false,
        }
    }
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            max_size: 180,
            max_weight: 300_000,
        }
    }
}