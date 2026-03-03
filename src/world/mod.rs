use std::collections::HashMap;
use crate::models::npc::NpcTemplate;
use crate::ecs::components::npc::SpawnInfo; // 確保這裡路徑如編譯器建議
use crate::network::shared_state::WorldState;

// 重點：必須宣告這個子模組，外部才找得到 crate::world::grid
pub mod grid; 

impl WorldState {
    pub fn spawn_monsters(&mut self, _templates: &HashMap<i32, NpcTemplate>, _spawns: Vec<SpawnInfo>) {
        // 加了底線 _spawns 消除警告
    }

    pub fn spawn_npcs(&mut self, _templates: &HashMap<i32, NpcTemplate>, _spawns: Vec<SpawnInfo>) {
        // 加了底線 _spawns 消除警告
    }
}