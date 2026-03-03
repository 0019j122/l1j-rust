use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::models::npc::{NpcTemplate, OnlineNpc};
use crate::models::player::OnlinePlayer;
use crate::models::item::OnlineItem;

pub type SharedWorld = Arc<Mutex<WorldState>>;

/// 建立全域世界狀態
pub fn new_shared_world() -> SharedWorld {
    Arc::new(Mutex::new(WorldState::new()))
}

pub struct WorldState {
    pub players: HashMap<i32, OnlinePlayer>,
    pub npcs: HashMap<i32, OnlineNpc>,
    pub npc_templates: HashMap<i32, NpcTemplate>,
    pub items: HashMap<i32, OnlineItem>,
    pub next_npc_id: u32,
    pub next_item_id: u32,
}

impl WorldState {      
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            npcs: HashMap::new(),
            npc_templates: HashMap::new(),
            items: HashMap::new(),
            next_npc_id: 1,
            next_item_id: 1,
        }
    }

    pub fn generate_next_id(&mut self) -> i32 {
        self.next_npc_id += 1;
        self.next_npc_id as i32
    }

    pub fn add_player(&mut self, player: OnlinePlayer) {
        self.players.insert(player.object_id, player);
    }

    pub fn remove_player(&mut self, object_id: i32) { 
        self.players.remove(&object_id);
    }

    pub fn update_position(&mut self, object_id: i32, x: i32, y: i32, heading: u8) {
        if let Some(player) = self.players.get_mut(&object_id) {
            player.x = x;
            player.y = y;
            player.heading = heading as i32; 
        }
    }

    pub fn get_nearby_players(&self, map_id: i32, x: i32, y: i32) -> Vec<OnlinePlayer> {
        self.players.values()
            .filter(|p| p.map_id as i32 == map_id && (p.x - x).abs() <= 20 && (p.y - y).abs() <= 20)
            .cloned()
            .collect()
    }

    /// 廣播封包給附近的玩家
    pub fn broadcast_nearby(&self, map_id: u32, x: i32, y: i32, pkt: Vec<u8>) {
        for player in self.players.values() {
            if player.map_id == map_id as i16 && 
               (player.x - x).abs() <= 18 && 
               (player.y - y).abs() <= 18 
            {
                if let Some(ref tx) = player.packet_tx {
                    let _ = tx.send(pkt.clone());
                }
            }
        }
    }

    /// 將 NPC 生成到戰場上
    pub fn spawn_to_battlefield(&mut self, template_id: i32, x: i32, y: i32, map: i32) {
        if let Some(temp) = self.npc_templates.get(&template_id).cloned() {
            let obj_id = self.generate_next_id();
            let active_npc = OnlineNpc {
                object_id: obj_id,
                template_id,
                cur_hp: temp.hp, 
                template: temp,  
                x,
                y,
                map_id: map as i16,
                heading: 0, 
            };
            self.npcs.insert(obj_id, active_npc);
        }
    }   
        
    /// 處理 NPC 死亡，將其從活躍列表移除並回傳資料
    pub fn handle_npc_death(&mut self, object_id: i32) -> Option<OnlineNpc> {
        self.npcs.remove(&object_id)
    }

    /// 在地上生成一個掉落物實體
    pub fn spawn_item_on_ground(&mut self, item_id: i32, count: i32, x: i32, y: i32, map_id: i16) -> i32 {
        let obj_id = self.generate_next_id();
        let new_item = OnlineItem {
            object_id: obj_id,
            item_id,
            count,
            x,
            y,
            map_id,
            is_equipped: false, 
        };
        self.items.insert(obj_id, new_item.clone());
        obj_id
    }
}