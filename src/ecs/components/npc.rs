use sqlx::FromRow;
/// NPC template data loaded from the `npc` database table.
///
/// This is the static template data, shared by all instances of the same NPC type.
#[derive(Debug, Clone, FromRow)] // <--- 這裡也要加上 FromRow
pub struct NpcTemplate {
    pub npc_id: i32,
    pub name: String,
    pub nameid: String,
    #[sqlx(rename = "impl")]
    pub impl_type: String,    // "L1Monster", "L1Guard", "L1Merchant", etc.
    pub gfxid: i32,
    pub level: i32,
    pub hp: i32,
    pub mp: i32,
    pub ac: i32,
    // --- 這裡開始是重點 ---
    #[sqlx(rename = "str")]  // 告訴程式：資料庫欄位其實叫 str
    pub str_stat: i32,
    #[sqlx(rename = "con")]
    pub con_stat: i32,
    #[sqlx(rename = "dex")]
    pub dex_stat: i32,
    #[sqlx(rename = "wis")]
    pub wis_stat: i32,
    #[sqlx(rename = "intel")] // 舊版資料庫通常叫 intel 或 intel_stat
    pub int_stat: i32,
    pub mr: i32,
    pub exp: i32,
    pub lawful: i32,
    pub size: String,         // "small", "large"
    pub undead: i32,
    pub poison_atk: i32,
    pub paralysis_atk: i32,
    pub agro: i32,           // aggressive
    pub agrososc: i32,       // aggressive on sight
    pub agrocoi: i32,        // aggressive on combat
    pub family: i32,
    pub agrofamily: i32,
    #[sqlx(rename = "picupitem")]
    pub pickup_item: i32,
    #[sqlx(rename = "bravespeed")] // 嘗試去掉底線，或是對照你資料庫裡的實際名稱
    pub brave_speed: i32,
    pub passi_speed: i32,
    // 記得 atk_speed 要對齊資料庫名
    #[sqlx(rename = "atkspeed")]
    pub atk_speed: i32,
    pub atk_magic_speed: i32,
    pub tamable: i32,
    pub teleport: i32,
    pub doppel: i32,
    pub hpr_interval: i32,
    pub hpr: i32,
    pub mpr_interval: i32,
    pub mpr: i32,
    pub ranged: i32,
    pub light_size: i32,
    pub change_head: i32,
    pub damage_reduction: i32,
    pub hard: i32,
    pub karma: i32,
    pub transform_id: i32,
    pub transform_gfxid: i32,
    pub cant_resurrect: i32,
}

/// AI state for a single NPC instance.
#[derive(Debug, Clone)]
pub struct AiState {
    /// Is the AI actively running?
    pub active: bool,
    /// Ticks until next AI action.
    pub sleep_ticks: i32,
    /// Are there players nearby? (If false, AI can be skipped)
    pub players_nearby: bool,
    /// Current hate list: (entity_id, hate_value)
    pub hate_list: Vec<(i32, i32)>,
    /// Current target entity ID (0 = no target)
    pub target_id: i32,
    /// Home position (spawn point) for return behavior
    pub home_x: i32,
    pub home_y: i32,
    /// Random walk state
    pub random_walk_distance: i32,
    pub random_walk_direction: i32,
}

impl AiState {
    pub fn new(home_x: i32, home_y: i32) -> Self {
        AiState {
            active: false,
            sleep_ticks: 0,
            players_nearby: false,
            hate_list: Vec::new(),
            target_id: 0,
            home_x,
            home_y,
            random_walk_distance: 0,
            random_walk_direction: 0,
        }
    }
}

/// Spawn info for an NPC instance (from spawnlist table).
#[derive(Debug, Clone)]
pub struct SpawnInfo {
    pub spawn_id: i32,
    pub npc_template_id: i32,   // 改成 i32。)
    pub loc_x: i32,
    pub loc_y: i32,
    pub map_id: i32,
    pub heading: i32,
    pub randomx: i32,
    pub randomy: i32,
    pub min_respawn_delay: i32,
    pub max_respawn_delay: i32,
    pub count: i32,
    pub movement_distance: i32,
}
