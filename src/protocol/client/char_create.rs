/// C_NEWCHAR (C_CreateChar) packet parser + character creation logic.
use sqlx::{self, FromRow};
use crate::protocol::packet::PacketReader;

/// Parsed C_NEWCHAR packet.
#[derive(Debug, Clone, FromRow)] // <--- 這裡也要加上 FromRow
pub struct NewChar {
    pub name: String,
    pub char_type: i32,  // 0=Prince,1=Knight,2=Elf,3=Mage,4=DarkElf,5=DragonKnight,6=Illusionist
    pub sex: i32,        // 0=male, 1=female
    // --- 這裡開始是重點 ---
    #[sqlx(rename = "str")]  // 告訴程式：資料庫欄位其實叫 str
    pub str_stat: i32,
    #[sqlx(rename = "dex")]
    pub dex_stat: i32,
    #[sqlx(rename = "con")]
    pub con_stat: i32,
    #[sqlx(rename = "wis")]
    pub wis_stat: i32,
    #[sqlx(rename = "cha")]
    pub cha_stat: i32,
    #[sqlx(rename = "intel")] // 舊版資料庫通常叫 intel 或 intel_stat
    pub int_stat: i32,
}

pub fn parse_new_char(data: &[u8]) -> NewChar {
    let mut r = PacketReader::after_opcode(data);
    let name = r.read_s();
    let char_type = r.read_c() as i32;
    let sex = r.read_c() as i32;
    let str_stat = r.read_c() as i32;
    let dex_stat = r.read_c() as i32;
    let con_stat = r.read_c() as i32;
    let wis_stat = r.read_c() as i32;
    let cha_stat = r.read_c() as i32;
    let int_stat = r.read_c() as i32;
    NewChar { name, char_type, sex, str_stat, dex_stat, con_stat, wis_stat, cha_stat, int_stat }
}

/// Base stats per class (official data).
///         STR DEX CON WIS CHA INT  BonusPts
const BASE_STATS: [[i32; 7]; 7] = [
    [13, 10, 10, 11, 13, 10, 8],  // 0 Prince
    [16, 12, 14,  9, 12,  8, 4],  // 1 Knight
    [11, 12, 12, 12,  9, 12, 7],  // 2 Elf
    [ 8,  7, 12, 12,  8, 12, 16], // 3 Mage
    [12, 15,  8, 10,  9, 11, 10], // 4 DarkElf
    [13, 11, 14, 12,  8, 11, 6],  // 5 DragonKnight
    [11, 10, 12, 12,  8, 12, 10], // 6 Illusionist
];

/// Initial HP per class.
const INIT_HP: [i32; 7] = [14, 16, 15, 12, 12, 16, 14];

/// GFX ID per class+sex. [class][sex] (male=0, female=1)
const CLASS_GFX: [[i32; 2]; 7] = [
    [0, 1],       // Prince
    [61, 48],     // Knight
    [138, 37],    // Elf
    [734, 1186],  // Mage
    [2786, 2796], // DarkElf
    [6658, 6661], // DragonKnight
    [6671, 6650], // Illusionist
];

/// Starting location (all classes).
pub const START_X: i32 = 32689;
pub const START_Y: i32 = 32842;
pub const START_MAP: i32 = 2005;

/// Validate character creation stats.
pub fn validate_stats(nc: &NewChar) -> bool {
    if nc.char_type < 0 || nc.char_type > 6 { return false; }
    let base = &BASE_STATS[nc.char_type as usize];
    let total = nc.str_stat + nc.dex_stat + nc.con_stat + nc.wis_stat + nc.cha_stat + nc.int_stat;
    if total != 75 { return false; }
    if nc.str_stat < base[0] { return false; }
    if nc.dex_stat < base[1] { return false; }
    if nc.con_stat < base[2] { return false; }
    if nc.wis_stat < base[3] { return false; }
    if nc.cha_stat < base[4] { return false; }
    if nc.int_stat < base[5] { return false; }
    true
}

/// Calculate initial MP based on class and WIS.
pub fn calc_init_mp(char_type: i32, wis: i32) -> i32 {
    match char_type {
        0 => { // Prince
            if wis >= 16 { 4 } else if wis >= 12 { 3 } else { 2 }
        }
        1 => { // Knight
            if wis >= 12 { 2 } else { 1 }
        }
        2 => { // Elf
            if wis >= 16 { 6 } else { 4 }
        }
        3 => { // Mage
            if wis >= 16 { 8 } else { 6 }
        }
        4 => { // DarkElf
            if wis >= 16 { 6 } else if wis >= 12 { 4 } else { 3 }
        }
        5 => 2, // DragonKnight
        6 => { // Illusionist
            if wis >= 16 { 6 } else { 5 }
        }
        _ => 2,
    }
}

/// Get GFX ID for class+sex.
pub fn get_gfx_id(char_type: i32, sex: i32) -> i32 {
    if char_type < 0 || char_type > 6 { return 0; }
    CLASS_GFX[char_type as usize][sex.clamp(0, 1) as usize]
}

/// Get initial HP for class.
pub fn get_init_hp(char_type: i32) -> i32 {
    if char_type < 0 || char_type > 6 { return 14; }
    INIT_HP[char_type as usize]
}
