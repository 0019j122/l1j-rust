/// Castle siege (攻城戰) system.
///
/// Ported from Java L1War.java, WarTimeController.java, L1CastleLocation.java,
/// L1TowerInstance.java, L1DoorInstance.java, L1CrownInstance.java.
///
/// Complete war state machine + castle/tower/door/crown mechanics.

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Castle definitions (from L1CastleLocation.java)
// ---------------------------------------------------------------------------

pub const KENT_CASTLE_ID: i32 = 1;
pub const OT_CASTLE_ID: i32 = 2;
pub const WW_CASTLE_ID: i32 = 3;
pub const GIRAN_CASTLE_ID: i32 = 4;
pub const HEINE_CASTLE_ID: i32 = 5;
pub const DOWA_CASTLE_ID: i32 = 6;
pub const ADEN_CASTLE_ID: i32 = 7;
pub const DIAD_CASTLE_ID: i32 = 8;

/// Castle static data.
#[derive(Debug, Clone)]
pub struct CastleInfo {
    pub castle_id: i32,
    pub name: &'static str,
    /// War area bounds (x1, x2, y1, y2, map_id).
    pub war_area: (i32, i32, i32, i32, i32),
    /// Tower location (x, y, map_id).
    pub tower_loc: (i32, i32, i32),
    /// Inner castle map ID (0 if same as outer).
    pub inner_map_id: i32,
}

/// All 8 castles (from Java L1CastleLocation.java).
pub fn get_castle_info() -> Vec<CastleInfo> {
    vec![
        CastleInfo { castle_id: 1, name: "肯特城",   war_area: (33089, 33219, 32717, 32827, 4),  tower_loc: (33139, 32768, 4),  inner_map_id: 15 },
        CastleInfo { castle_id: 2, name: "妖魔城",   war_area: (32627, 32757, 32742, 32852, 4),  tower_loc: (32675, 32797, 4),  inner_map_id: 16 },
        CastleInfo { castle_id: 3, name: "風木城",   war_area: (32491, 32621, 32572, 32682, 4),  tower_loc: (32555, 32617, 4),  inner_map_id: 19 },
        CastleInfo { castle_id: 4, name: "奇岩城",   war_area: (33504, 33634, 32692, 32802, 4),  tower_loc: (33553, 32742, 4),  inner_map_id: 29 },
        CastleInfo { castle_id: 5, name: "海音城",   war_area: (33647, 33777, 33197, 33307, 4),  tower_loc: (33700, 33261, 4),  inner_map_id: 52 },
        CastleInfo { castle_id: 6, name: "侏儒城",   war_area: (32482, 32612, 33159, 33269, 4),  tower_loc: (32546, 33208, 4),  inner_map_id: 64 },
        CastleInfo { castle_id: 7, name: "亞丁城",   war_area: (34015, 34145, 33166, 33276, 4),  tower_loc: (34065, 33224, 4),  inner_map_id: 300 },
        CastleInfo { castle_id: 8, name: "狄亞得要塞", war_area: (33465, 33595, 32830, 32940, 4), tower_loc: (33523, 32880, 4), inner_map_id: 320 },
    ]
}

// ---------------------------------------------------------------------------
// Castle dynamic data (from CastleTable / castle SQL table)
// ---------------------------------------------------------------------------

/// Castle runtime state loaded from `castle` DB table.
#[derive(Debug, Clone)]
pub struct CastleData {
    pub castle_id: i32,
    pub name: String,
    pub war_time: i64,      // next war time as unix timestamp
    pub tax_rate: i32,      // tax percentage
    pub public_money: i32,  // accumulated tax revenue
    pub owner_clan_id: i32, // clan_data.clan_id that owns this castle (0=none)
}

// ---------------------------------------------------------------------------
// War state machine (from L1War.java)
// ---------------------------------------------------------------------------

/// War type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WarType {
    CastleWar = 1,
    SimulatedWar = 2,
}

/// War message types (S_War packet).
pub mod war_msg {
    pub const DECLARE: i32 = 1;
    pub const SURRENDER: i32 = 2;
    pub const CEASE: i32 = 3;
    pub const VICTORY: i32 = 4;
    pub const ALLIANCE: i32 = 6;
    pub const ALLIANCE_BREAK: i32 = 7;
    pub const AT_WAR: i32 = 8;
}

/// Active war instance.
#[derive(Debug, Clone)]
pub struct ActiveWar {
    pub war_type: WarType,
    pub attack_clans: Vec<String>,
    pub defence_clan: String,
    pub castle_id: i32,       // 0 for simulated war
    pub war_end_time: i64,    // unix timestamp
    pub is_active: bool,
}

impl ActiveWar {
    /// Create a new castle war.
    pub fn new_castle_war(attack_clan: String, defence_clan: String, castle_id: i32, end_time: i64) -> Self {
        ActiveWar {
            war_type: WarType::CastleWar,
            attack_clans: vec![attack_clan],
            defence_clan,
            castle_id,
            war_end_time: end_time,
            is_active: true,
        }
    }

    /// Create a new simulated (clan) war.
    pub fn new_sim_war(attack_clan: String, defence_clan: String, duration_minutes: i64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
        ActiveWar {
            war_type: WarType::SimulatedWar,
            attack_clans: vec![attack_clan],
            defence_clan,
            castle_id: 0,
            war_end_time: now + duration_minutes * 60,
            is_active: true,
        }
    }

    /// Add another attacking clan (alliance attack).
    pub fn add_attacker(&mut self, clan_name: String) {
        if !self.attack_clans.contains(&clan_name) {
            self.attack_clans.push(clan_name);
        }
    }

    /// Check if a clan is involved in this war.
    pub fn involves_clan(&self, clan_name: &str) -> bool {
        self.defence_clan == clan_name || self.attack_clans.contains(&clan_name.to_string())
    }

    /// Check if war time has expired.
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
        now >= self.war_end_time
    }
}

// ---------------------------------------------------------------------------
// Door mechanics (from L1DoorInstance.java)
// ---------------------------------------------------------------------------

/// Door damage states based on remaining HP fraction.
pub mod door_action {
    pub const DOOR_OPEN: i32 = 0;
    pub const DOOR_CLOSE: i32 = 1;
    pub const DOOR_ACTION1: i32 = 2;  // 5/6 HP
    pub const DOOR_ACTION2: i32 = 3;  // 4/6 HP
    pub const DOOR_ACTION3: i32 = 4;  // 3/6 HP
    pub const DOOR_ACTION4: i32 = 5;  // 2/6 HP
    pub const DOOR_ACTION5: i32 = 6;  // 1/6 HP
    pub const DOOR_DIE: i32 = 7;
}

/// A castle door entity.
#[derive(Debug, Clone)]
pub struct DoorState {
    pub object_id: i32,
    pub castle_id: i32,
    pub max_hp: i32,
    pub cur_hp: i32,
    pub is_open: bool,
    pub direction: i32,  // 0 = /, 1 = \
    pub x: i32,
    pub y: i32,
    pub map_id: i32,
}

impl DoorState {
    /// Get the visual action ID based on current HP.
    pub fn get_damage_action(&self) -> i32 {
        if self.cur_hp <= 0 {
            return door_action::DOOR_DIE;
        }
        if self.max_hp == 0 {
            return door_action::DOOR_CLOSE; // indestructible
        }
        let ratio = self.cur_hp as f32 / self.max_hp as f32;
        if ratio > 5.0 / 6.0 { door_action::DOOR_CLOSE }
        else if ratio > 4.0 / 6.0 { door_action::DOOR_ACTION1 }
        else if ratio > 3.0 / 6.0 { door_action::DOOR_ACTION2 }
        else if ratio > 2.0 / 6.0 { door_action::DOOR_ACTION3 }
        else if ratio > 1.0 / 6.0 { door_action::DOOR_ACTION4 }
        else { door_action::DOOR_ACTION5 }
    }

    /// Apply damage. Returns true if destroyed.
    pub fn receive_damage(&mut self, damage: i32) -> bool {
        if self.max_hp == 0 { return false; } // indestructible
        self.cur_hp = (self.cur_hp - damage).max(0);
        self.cur_hp <= 0
    }

    /// Full repair + close.
    pub fn repair(&mut self) {
        self.cur_hp = self.max_hp;
        self.is_open = false;
    }
}

// ---------------------------------------------------------------------------
// Tower mechanics (from L1TowerInstance.java)
// ---------------------------------------------------------------------------

/// Tower crack visual states.
pub mod tower_action {
    pub const TOWER_NORMAL: i32 = 0;
    pub const TOWER_CRACK1: i32 = 1;  // 75% HP
    pub const TOWER_CRACK2: i32 = 2;  // 50% HP
    pub const TOWER_CRACK3: i32 = 3;  // 25% HP
    pub const TOWER_DIE: i32 = 4;
}

/// NPC IDs for towers.
pub const GUARDIAN_TOWER_NPC_ID: i32 = 81111;
pub const ADEN_MAIN_TOWER_NPC_ID: i32 = 81189;
pub const ADEN_SUB_TOWER_NPC_IDS: [i32; 4] = [81190, 81191, 81192, 81193];

/// A castle tower entity.
#[derive(Debug, Clone)]
pub struct TowerState {
    pub object_id: i32,
    pub castle_id: i32,
    pub npc_id: i32,
    pub max_hp: i32,
    pub cur_hp: i32,
    pub x: i32,
    pub y: i32,
    pub map_id: i32,
}

impl TowerState {
    /// Get visual crack state based on HP.
    pub fn get_crack_action(&self) -> i32 {
        if self.cur_hp <= 0 { return tower_action::TOWER_DIE; }
        let ratio = self.cur_hp as f32 / self.max_hp as f32;
        if ratio > 0.75 { tower_action::TOWER_NORMAL }
        else if ratio > 0.50 { tower_action::TOWER_CRACK1 }
        else if ratio > 0.25 { tower_action::TOWER_CRACK2 }
        else { tower_action::TOWER_CRACK3 }
    }

    /// Apply damage. Returns true if destroyed.
    pub fn receive_damage(&mut self, damage: i32) -> bool {
        self.cur_hp = (self.cur_hp - damage).max(0);
        self.cur_hp <= 0
    }

    /// Check if this is the Aden main tower.
    pub fn is_aden_main_tower(&self) -> bool {
        self.npc_id == ADEN_MAIN_TOWER_NPC_ID
    }
}

// ---------------------------------------------------------------------------
// Crown (王冠) mechanics - spawns when tower is destroyed
// ---------------------------------------------------------------------------

/// Crown state - only exists after tower destruction during active war.
#[derive(Debug, Clone)]
pub struct CrownState {
    pub object_id: i32,
    pub castle_id: i32,
    pub x: i32,
    pub y: i32,
    pub map_id: i32,
}

// ---------------------------------------------------------------------------
// Siege manager - ties everything together
// ---------------------------------------------------------------------------

/// Manages all active wars, castle state, and siege mechanics.
pub struct SiegeManager {
    pub active_wars: Vec<ActiveWar>,
    pub castles: HashMap<i32, CastleData>,
    pub doors: Vec<DoorState>,
    pub towers: Vec<TowerState>,
    pub crowns: Vec<CrownState>,
    pub castle_info: Vec<CastleInfo>,
    /// Aden sub-tower destruction count (need 3+ to attack main tower).
    pub aden_sub_towers_destroyed: i32,
}

impl SiegeManager {
    pub fn new() -> Self {
        SiegeManager {
            active_wars: Vec::new(),
            castles: HashMap::new(),
            doors: Vec::new(),
            towers: Vec::new(),
            crowns: Vec::new(),
            castle_info: get_castle_info(),
            aden_sub_towers_destroyed: 0,
        }
    }

    /// Check if a castle is currently at war.
    pub fn is_now_war(&self, castle_id: i32) -> bool {
        self.active_wars.iter().any(|w| w.castle_id == castle_id && w.is_active)
    }

    /// Check if a position is inside a castle's war area.
    pub fn is_in_war_area(&self, castle_id: i32, x: i32, y: i32, map_id: i32) -> bool {
        self.castle_info.iter()
            .find(|c| c.castle_id == castle_id)
            .map(|c| {
                let (x1, x2, y1, y2, mid) = c.war_area;
                map_id == mid && x >= x1 && x <= x2 && y >= y1 && y <= y2
            })
            .unwrap_or(false)
    }

    /// Find which castle a position belongs to (if any).
    pub fn get_castle_id_at(&self, x: i32, y: i32, map_id: i32) -> Option<i32> {
        self.castle_info.iter()
            .find(|c| {
                let (x1, x2, y1, y2, mid) = c.war_area;
                map_id == mid && x >= x1 && x <= x2 && y >= y1 && y <= y2
            })
            .map(|c| c.castle_id)
    }

    /// Find active war for a clan.
    pub fn find_war_for_clan(&self, clan_name: &str) -> Option<&ActiveWar> {
        self.active_wars.iter().find(|w| w.involves_clan(clan_name))
    }

    /// Handle tower destruction - spawn crown if appropriate.
    pub fn on_tower_destroyed(&mut self, tower: &TowerState) -> bool {
        // Check Aden sub-tower logic
        if ADEN_SUB_TOWER_NPC_IDS.contains(&tower.npc_id) {
            self.aden_sub_towers_destroyed += 1;
            return false; // sub-tower, don't spawn crown yet
        }

        if tower.is_aden_main_tower() && self.aden_sub_towers_destroyed < 3 {
            return false; // main tower invulnerable until 3+ subs down
        }

        // Spawn crown at tower location
        self.crowns.push(CrownState {
            object_id: 0, // caller assigns
            castle_id: tower.castle_id,
            x: tower.x,
            y: tower.y,
            map_id: tower.map_id,
        });

        true // crown spawned
    }

    /// Check war timers - expire wars that have timed out.
    pub fn tick_war_timers(&mut self) -> Vec<i32> {
        let mut ended_castle_ids = Vec::new();
        for war in &mut self.active_wars {
            if war.is_active && war.is_expired() {
                war.is_active = false;
                ended_castle_ids.push(war.castle_id);
            }
        }
        self.active_wars.retain(|w| w.is_active);
        ended_castle_ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_castle_info() {
        let info = get_castle_info();
        assert_eq!(info.len(), 8);
        assert_eq!(info[0].name, "肯特城");
        assert_eq!(info[6].castle_id, ADEN_CASTLE_ID);
    }

    #[test]
    fn test_war_area_check() {
        let mut mgr = SiegeManager::new();
        mgr.castles.insert(1, CastleData {
            castle_id: 1, name: "Kent".into(), war_time: 0,
            tax_rate: 10, public_money: 0, owner_clan_id: 0,
        });

        // Inside Kent war area
        assert!(mgr.is_in_war_area(1, 33150, 32770, 4));
        // Outside
        assert!(!mgr.is_in_war_area(1, 30000, 30000, 4));
    }

    #[test]
    fn test_door_damage() {
        let mut door = DoorState {
            object_id: 1, castle_id: 1, max_hp: 600, cur_hp: 600,
            is_open: false, direction: 0, x: 100, y: 200, map_id: 4,
        };

        assert_eq!(door.get_damage_action(), door_action::DOOR_CLOSE);

        door.receive_damage(200);
        assert_eq!(door.cur_hp, 400);
        assert_eq!(door.get_damage_action(), door_action::DOOR_ACTION2);

        let destroyed = door.receive_damage(500);
        assert!(destroyed);
        assert_eq!(door.get_damage_action(), door_action::DOOR_DIE);

        door.repair();
        assert_eq!(door.cur_hp, 600);
    }

    #[test]
    fn test_tower_crack() {
        let mut tower = TowerState {
            object_id: 1, castle_id: 1, npc_id: GUARDIAN_TOWER_NPC_ID,
            max_hp: 1000, cur_hp: 1000, x: 100, y: 200, map_id: 4,
        };

        assert_eq!(tower.get_crack_action(), tower_action::TOWER_NORMAL);

        tower.receive_damage(300); // 700/1000 = 70%
        assert_eq!(tower.get_crack_action(), tower_action::TOWER_CRACK1);

        tower.receive_damage(250); // 450/1000 = 45%
        assert_eq!(tower.get_crack_action(), tower_action::TOWER_CRACK2);

        tower.receive_damage(300); // 150/1000 = 15%
        assert_eq!(tower.get_crack_action(), tower_action::TOWER_CRACK3);

        let destroyed = tower.receive_damage(200);
        assert!(destroyed);
        assert_eq!(tower.get_crack_action(), tower_action::TOWER_DIE);
    }

    #[test]
    fn test_aden_sub_tower_logic() {
        let mut mgr = SiegeManager::new();

        // Destroy 2 sub-towers
        for &npc_id in &ADEN_SUB_TOWER_NPC_IDS[..2] {
            let tower = TowerState {
                object_id: 1, castle_id: 7, npc_id,
                max_hp: 1000, cur_hp: 0, x: 100, y: 200, map_id: 4,
            };
            let spawned = mgr.on_tower_destroyed(&tower);
            assert!(!spawned); // no crown yet
        }

        // Main tower still invulnerable
        let main_tower = TowerState {
            object_id: 2, castle_id: 7, npc_id: ADEN_MAIN_TOWER_NPC_ID,
            max_hp: 2000, cur_hp: 0, x: 100, y: 200, map_id: 4,
        };
        assert!(!mgr.on_tower_destroyed(&main_tower));

        // Destroy 3rd sub-tower
        let tower3 = TowerState {
            object_id: 3, castle_id: 7, npc_id: ADEN_SUB_TOWER_NPC_IDS[2],
            max_hp: 1000, cur_hp: 0, x: 100, y: 200, map_id: 4,
        };
        mgr.on_tower_destroyed(&tower3);

        // Now main tower can be destroyed -> crown spawns
        assert!(mgr.on_tower_destroyed(&main_tower));
        assert_eq!(mgr.crowns.len(), 1);
        assert_eq!(mgr.crowns[0].castle_id, ADEN_CASTLE_ID);
    }

    #[test]
    fn test_war_timer_expiry() {
        let mut mgr = SiegeManager::new();
        mgr.active_wars.push(ActiveWar {
            war_type: WarType::CastleWar,
            attack_clans: vec!["Attacker".into()],
            defence_clan: "Defender".into(),
            castle_id: 1,
            war_end_time: 0, // already expired
            is_active: true,
        });

        let ended = mgr.tick_war_timers();
        assert_eq!(ended, vec![1]);
        assert!(mgr.active_wars.is_empty());
    }
}
