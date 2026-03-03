use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

// --- 常數定義 ---
pub const ACTION_IDLE: i32 = 0;
pub const ACTION_ATTACK: i32 = 1;
pub const ACTION_DAMAGE: i32 = 2;
pub const ACTION_DIE: i32 = 8;
pub const ACTION_PICKUP: i32 = 15;

// --- 封包構造函數 (這些是獨立函數) ---

pub fn build_attack_packet(attacker_id: i32, target_id: i32, action_id: i32, damage: i32, heading: i32, effect: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_ATTACKPACKET)
        .write_c(action_id)
        .write_d(attacker_id)
        .write_d(target_id)
        .write_h(damage)
        .write_c(heading)
        .write_d(0)
        .write_c(effect)
        .build()
}

pub fn build_hp_meter(object_id: i32, cur_hp: i32, max_hp: i32) -> Vec<u8> {
    let ratio = if max_hp > 0 { (100 * cur_hp / max_hp).clamp(0, 100) } else { 0 };
    PacketBuilder::new(server::S_OPCODE_HPMETER)
        .write_d(object_id)
        .write_h(ratio as i32)
        .build()
}

/// 讓玩家看到地上的物品 (放在這裡，因為它是伺服器發出的封包)
pub fn build_drop_item(object_id: i32, x: i32, y: i32, gfx_id: i32, count: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_DROPITEM)
        .write_h(x)
        .write_h(y)
        .write_d(object_id)
        .write_h(gfx_id)
        .write_c(0) // 狀態
        .write_c(0) // 祝福
        .write_d(count)
        .write_c(0)
        .build()
}

// --- 世界狀態邏輯 (這部分通常在 shared_state.rs，如果要在這裡補，請確保結構正確) ---

/* impl WorldState {
    pub fn apply_npc_damage(&mut self, npc_id: i32, damage: i32) -> Option<(i32, i32, bool)> {
        if let Some(npc) = self.npcs.get_mut(&npc_id) {
            npc.cur_hp = (npc.cur_hp - damage).max(0);
            let cur = npc.cur_hp;
            let max = npc.template.hp;
            let is_dead = cur <= 0;
            return Some((cur, max, is_dead));
        }
        None
    }
}
*/