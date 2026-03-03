/// 黑暗妖精技能系統 - 基於官方數據。
///
/// 資料來源：天堂透視鏡、巴哈姆特、LoA 美版天堂資料庫。
///
/// 技能分四級：
///   一級 (Lv15)：暗隱術、附加劇毒、影之防護、力量提升、提煉魔石、行走加速
///   二級 (Lv30)：燃燒鬥志、暗黑盲咒、毒性抵抗、雙重破壞、暗影閃避
///   三級 (Lv45)：暗影之牙、會心一擊、力量提升、敏捷提升、閃避提升
///   四級 (Lv60+)：破壞盔甲、刺客、熾烈鬥志

/// 黑暗妖精技能 ID。
pub mod de_skill_id {
    // 一級 (Lv15)
    pub const DARK_BLIND: i32 = 97;       // 暗隱術
    pub const ENCHANT_VENOM: i32 = 98;    // 附加劇毒
    pub const SHADOW_ARMOR: i32 = 99;     // 影之防護
    pub const BRING_STONE: i32 = 100;     // 提煉魔石
    pub const MOVING_ACCEL: i32 = 101;    // 行走加速

    // 二級 (Lv30)
    pub const BURNING_SPIRIT: i32 = 102;  // 燃燒鬥志
    pub const DARK_BLIND_ATTACK: i32 = 103; // 暗黑盲咒
    pub const VENOM_RESIST: i32 = 104;    // 毒性抵抗
    pub const DOUBLE_BREAK: i32 = 105;    // 雙重破壞
    pub const SHADOW_FANG: i32 = 106;     // 暗影閃避

    // 三級 (Lv45)
    pub const SHADOW_BITE: i32 = 107;     // 暗影之牙
    pub const FINAL_BURN: i32 = 108;      // 會心一擊
    pub const STR_UP: i32 = 109;          // 力量提升
    pub const DEX_UP: i32 = 110;          // 敏捷提升
    pub const DODGE_UP: i32 = 111;        // 閃避提升

    // 四級 (Lv60+)
    pub const ARMOR_BREAK: i32 = 112;     // 破壞盔甲
}

/// 黑暗妖精技能模板。
#[derive(Debug, Clone)]
pub struct DarkElfSkill {
    pub skill_id: i32,
    pub name: &'static str,
    pub learn_level: i32,
    pub mp_cost: i32,
    pub hp_cost: i32,
    pub black_stone_cost: i32,   // 黑魔石消耗
    pub cast_time_ms: i32,       // 施法延遲（毫秒）
    pub duration_sec: i32,       // 持續時間（秒），0=瞬發
    pub skill_grade: i32,        // 1-4 級
}

/// 所有黑暗妖精技能的官方數據。
pub fn all_darkelf_skills() -> Vec<DarkElfSkill> {
    vec![
        // === 一級技能 (Lv15) ===
        DarkElfSkill {
            skill_id: de_skill_id::DARK_BLIND, name: "暗隱術",
            learn_level: 15, mp_cost: 10, hp_cost: 0, black_stone_cost: 0,
            cast_time_ms: 10000, duration_sec: 32, skill_grade: 1,
        },
        DarkElfSkill {
            skill_id: de_skill_id::ENCHANT_VENOM, name: "附加劇毒",
            learn_level: 15, mp_cost: 10, hp_cost: 1, black_stone_cost: 0,
            cast_time_ms: 360, duration_sec: 320, skill_grade: 1,
        },
        DarkElfSkill {
            skill_id: de_skill_id::SHADOW_ARMOR, name: "影之防護",
            learn_level: 15, mp_cost: 12, hp_cost: 0, black_stone_cost: 0,
            cast_time_ms: 360, duration_sec: 960, skill_grade: 1,
        },
        DarkElfSkill {
            skill_id: de_skill_id::BRING_STONE, name: "提煉魔石",
            learn_level: 15, mp_cost: 5, hp_cost: 0, black_stone_cost: 0,
            cast_time_ms: 360, duration_sec: 0, skill_grade: 1,
        },
        DarkElfSkill {
            skill_id: de_skill_id::MOVING_ACCEL, name: "行走加速",
            learn_level: 15, mp_cost: 10, hp_cost: 0, black_stone_cost: 0,
            cast_time_ms: 360, duration_sec: 960, skill_grade: 1,
        },
        // === 二級技能 (Lv30) ===
        DarkElfSkill {
            skill_id: de_skill_id::BURNING_SPIRIT, name: "燃燒鬥志",
            learn_level: 30, mp_cost: 20, hp_cost: 0, black_stone_cost: 0,
            cast_time_ms: 360, duration_sec: 300, skill_grade: 2,
        },
        DarkElfSkill {
            skill_id: de_skill_id::DARK_BLIND_ATTACK, name: "暗黑盲咒",
            learn_level: 30, mp_cost: 20, hp_cost: 0, black_stone_cost: 0,
            cast_time_ms: 1000, duration_sec: 3, skill_grade: 2,
        },
        DarkElfSkill {
            skill_id: de_skill_id::VENOM_RESIST, name: "毒性抵抗",
            learn_level: 30, mp_cost: 20, hp_cost: 0, black_stone_cost: 0,
            cast_time_ms: 360, duration_sec: 320, skill_grade: 2,
        },
        DarkElfSkill {
            skill_id: de_skill_id::DOUBLE_BREAK, name: "雙重破壞",
            learn_level: 30, mp_cost: 20, hp_cost: 0, black_stone_cost: 0,
            cast_time_ms: 360, duration_sec: 192, skill_grade: 2,
        },
        DarkElfSkill {
            skill_id: de_skill_id::SHADOW_FANG, name: "暗影閃避",
            learn_level: 30, mp_cost: 20, hp_cost: 0, black_stone_cost: 0,
            cast_time_ms: 360, duration_sec: 192, skill_grade: 2,
        },
        // === 三級技能 (Lv45) ===
        DarkElfSkill {
            skill_id: de_skill_id::SHADOW_BITE, name: "暗影之牙",
            learn_level: 45, mp_cost: 20, hp_cost: 0, black_stone_cost: 1,
            cast_time_ms: 360, duration_sec: 192, skill_grade: 3,
        },
        DarkElfSkill {
            skill_id: de_skill_id::FINAL_BURN, name: "會心一擊",
            learn_level: 45, mp_cost: 1, hp_cost: 1, black_stone_cost: 3,
            cast_time_ms: 2500, duration_sec: 0, skill_grade: 3,
        },
        DarkElfSkill {
            skill_id: de_skill_id::STR_UP, name: "力量提升",
            learn_level: 45, mp_cost: 10, hp_cost: 0, black_stone_cost: 0,
            cast_time_ms: 360, duration_sec: 960, skill_grade: 3,
        },
        DarkElfSkill {
            skill_id: de_skill_id::DEX_UP, name: "敏捷提升",
            learn_level: 45, mp_cost: 10, hp_cost: 0, black_stone_cost: 0,
            cast_time_ms: 360, duration_sec: 960, skill_grade: 3,
        },
        DarkElfSkill {
            skill_id: de_skill_id::DODGE_UP, name: "閃避提升",
            learn_level: 45, mp_cost: 15, hp_cost: 0, black_stone_cost: 0,
            cast_time_ms: 360, duration_sec: 32, skill_grade: 3,
        },
        // === 四級技能 (Lv60) ===
        DarkElfSkill {
            skill_id: de_skill_id::ARMOR_BREAK, name: "破壞盔甲",
            learn_level: 60, mp_cost: 32, hp_cost: 30, black_stone_cost: 2,
            cast_time_ms: 2000, duration_sec: 8, skill_grade: 4,
        },
    ]
}

// ===========================================================================
// 破壞盔甲 效果計算 (官方數據)
// ===========================================================================

/// 破壞盔甲效果。
///
/// 官方數據 (LoA 美版天堂 3.63)：
///   - 學習等級：60
///   - 消耗：MP 32、HP 30、二級黑魔石 ×2
///   - 施法延遲：2 秒
///   - 持續時間：8 秒
///   - 效果：增加對目標 58% 的傷害
///   - 射程：3 格內敵方 PC
///   - 無視魔法屏障
#[derive(Debug, Clone)]
pub struct ArmorBreakEffect {
    /// 傷害增幅（58% = 0.58）。
    pub damage_multiplier: f32,
    /// 持續時間（秒）。
    pub duration_sec: i32,
    /// 持續 ticks (duration_sec * 5)。
    pub duration_ticks: i32,
    /// 是否無視魔法屏障。
    pub ignore_barrier: bool,
    /// 施法範圍（格）。
    pub cast_range: i32,
}

impl ArmorBreakEffect {
    /// 官方基礎效果。
    pub fn base() -> Self {
        ArmorBreakEffect {
            damage_multiplier: 0.58,  // 增加 58% 傷害
            duration_sec: 8,
            duration_ticks: 40,       // 8秒 * 5 ticks/sec
            ignore_barrier: true,
            cast_range: 3,
        }
    }
}

/// 計算破壞盔甲後的傷害。
///
/// 原始傷害 * (1.0 + 0.58) = 原始傷害 * 1.58
pub fn calc_armor_break_damage(original_damage: i32, effect: &ArmorBreakEffect) -> i32 {
    ((original_damage as f32) * (1.0 + effect.damage_multiplier)) as i32
}

// ===========================================================================
// 其他關鍵技能效果
// ===========================================================================

/// 燃燒鬥志效果。
///
/// 官方：34% 機率攻擊乘以 1.5 倍（巴哈姆特數據）
/// 或 一定機率發出 2 倍攻擊力（LoA 數據）。
/// 採用巴哈姆特數據：34% 機率 × 1.5 倍。
pub struct BurningSpiritEffect {
    pub proc_chance: f32,      // 34%
    pub damage_multiplier: f32, // 1.5x
    pub duration_sec: i32,      // 300 秒
}

impl BurningSpiritEffect {
    pub fn official() -> Self {
        BurningSpiritEffect {
            proc_chance: 0.34,
            damage_multiplier: 1.5,
            duration_sec: 300,
        }
    }
}

/// 雙重破壞效果。
///
/// 官方：32% 機率雙刀/鋼爪傷害乘以 2 倍。
/// 限裝備雙刀、鋼爪才有效。
pub struct DoubleBreakEffect {
    pub proc_chance: f32,      // 32%
    pub damage_multiplier: f32, // 2.0x
    pub duration_sec: i32,      // 192 秒
    /// 限制武器類型（雙刀、鋼爪）。
    pub required_weapon_types: &'static [i32],
}

impl DoubleBreakEffect {
    pub fn official() -> Self {
        DoubleBreakEffect {
            proc_chance: 0.32,
            damage_multiplier: 2.0,
            duration_sec: 192,
            required_weapon_types: &[17, 18], // 17=雙手劍(雙刀), 18=鋼爪
        }
    }
}

/// 暗影之牙效果。
///
/// 官方：提高武器 5 點攻擊力。
/// 消耗：1 個黑魔石。
pub struct ShadowBiteEffect {
    pub damage_bonus: i32,     // +5
    pub duration_sec: i32,      // 192 秒
    pub stone_cost: i32,        // 1
}

impl ShadowBiteEffect {
    pub fn official() -> Self {
        ShadowBiteEffect {
            damage_bonus: 5,
            duration_sec: 192,
            stone_cost: 1,
        }
    }
}

/// 暗影閃避效果。
///
/// 官方：提高閃避率 60 點（巴哈姆特數據）。
pub struct ShadowDodgeEffect {
    pub dodge_bonus: i32,      // +60
    pub duration_sec: i32,      // 192 秒
}

impl ShadowDodgeEffect {
    pub fn official() -> Self {
        ShadowDodgeEffect {
            dodge_bonus: 60,
            duration_sec: 192,
        }
    }
}

/// 暗黑盲咒效果。
///
/// 官方：使目標睡眠 3 秒，受攻擊則醒來。
/// 命中率受等級差影響。
pub struct DarkBlindEffect {
    pub sleep_duration_sec: i32, // 3 秒
    pub base_hit_rate: f32,      // 50%
}

impl DarkBlindEffect {
    pub fn official() -> Self {
        DarkBlindEffect {
            sleep_duration_sec: 3,
            base_hit_rate: 0.50,
        }
    }

    /// 計算實際命中率（考慮等級差）。
    pub fn calc_hit_rate(&self, caster_level: i32, target_level: i32) -> f32 {
        let level_diff = caster_level - target_level;
        let adjustment = level_diff as f32 * 0.02; // 每級差 2%
        (self.base_hit_rate + adjustment).clamp(0.05, 0.95)
    }
}

/// 會心一擊效果。
///
/// 官方：消耗所有 MP 和 HP（變成 1），對目標造成一擊。
/// 傷害公式：(消耗MP + 消耗HP) * 倍率。
pub struct FinalBurnEffect {
    pub damage_multiplier: f32,  // 約 1.5 倍
    pub stone_cost: i32,         // 3
}

impl FinalBurnEffect {
    pub fn official() -> Self {
        FinalBurnEffect {
            damage_multiplier: 1.5,
            stone_cost: 3,
        }
    }

    /// 計算會心一擊傷害。
    pub fn calc_damage(&self, cur_hp: i32, cur_mp: i32) -> i32 {
        let consumed = (cur_hp - 1) + (cur_mp - 1);
        (consumed as f32 * self.damage_multiplier) as i32
    }
}

/// 附加劇毒效果。
///
/// 官方：武器攻擊附加中毒效果，成功機率 20%~40%。
pub struct EnchantVenomEffect {
    pub proc_chance_min: f32,   // 20%
    pub proc_chance_max: f32,   // 40%
    pub poison_damage_per_tick: i32, // 毒傷/tick
    pub duration_sec: i32,      // 320 秒 (buff 持續)
}

impl EnchantVenomEffect {
    pub fn official() -> Self {
        EnchantVenomEffect {
            proc_chance_min: 0.20,
            proc_chance_max: 0.40,
            poison_damage_per_tick: 5,
            duration_sec: 320,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_armor_break_damage() {
        let effect = ArmorBreakEffect::base();
        // 100 傷害 * 1.58 = 158
        assert_eq!(calc_armor_break_damage(100, &effect), 158);
        // 50 傷害 * 1.58 = 79
        assert_eq!(calc_armor_break_damage(50, &effect), 79);
    }

    #[test]
    fn test_armor_break_official_values() {
        let effect = ArmorBreakEffect::base();
        assert_eq!(effect.duration_sec, 8);
        assert_eq!(effect.cast_range, 3);
        assert!(effect.ignore_barrier);
    }

    #[test]
    fn test_skill_list_completeness() {
        let skills = all_darkelf_skills();
        assert_eq!(skills.len(), 16); // 5 + 5 + 5 + 1 = 16

        // 驗證等級分布
        let grade1: Vec<_> = skills.iter().filter(|s| s.skill_grade == 1).collect();
        let grade2: Vec<_> = skills.iter().filter(|s| s.skill_grade == 2).collect();
        let grade3: Vec<_> = skills.iter().filter(|s| s.skill_grade == 3).collect();
        let grade4: Vec<_> = skills.iter().filter(|s| s.skill_grade == 4).collect();
        assert_eq!(grade1.len(), 5);
        assert_eq!(grade2.len(), 5);
        assert_eq!(grade3.len(), 5);
        assert_eq!(grade4.len(), 1);
    }

    #[test]
    fn test_armor_break_cost() {
        let skills = all_darkelf_skills();
        let ab = skills.iter().find(|s| s.skill_id == de_skill_id::ARMOR_BREAK).unwrap();
        assert_eq!(ab.mp_cost, 32);
        assert_eq!(ab.hp_cost, 30);
        assert_eq!(ab.black_stone_cost, 2);
        assert_eq!(ab.learn_level, 60);
        assert_eq!(ab.duration_sec, 8);
    }

    #[test]
    fn test_final_burn_damage() {
        let effect = FinalBurnEffect::official();
        // HP=500, MP=200 → 消耗 499+199=698 → 698*1.5=1047
        assert_eq!(effect.calc_damage(500, 200), 1047);
    }

    #[test]
    fn test_dark_blind_hit_rate() {
        let effect = DarkBlindEffect::official();
        // 同等級: 50%
        assert!((effect.calc_hit_rate(50, 50) - 0.50).abs() < 0.01);
        // 高 10 級: 50% + 20% = 70%
        assert!((effect.calc_hit_rate(60, 50) - 0.70).abs() < 0.01);
        // 低 10 級: 50% - 20% = 30%
        assert!((effect.calc_hit_rate(40, 50) - 0.30).abs() < 0.01);
        // 上限 95%
        assert!((effect.calc_hit_rate(99, 1) - 0.95).abs() < 0.01);
    }

    #[test]
    fn test_burning_spirit() {
        let effect = BurningSpiritEffect::official();
        assert!((effect.proc_chance - 0.34).abs() < 0.01);
        assert!((effect.damage_multiplier - 1.5).abs() < 0.01);
    }

    #[test]
    fn test_double_break() {
        let effect = DoubleBreakEffect::official();
        assert!((effect.proc_chance - 0.32).abs() < 0.01);
        assert!((effect.damage_multiplier - 2.0).abs() < 0.01);
    }
}
