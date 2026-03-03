/// Skill system components.
///
/// Covers skill templates, active buffs/debuffs, and cooldowns.

use std::collections::HashMap;

/// Skill template loaded from the `skills` database table.
#[derive(Debug, Clone)]
pub struct SkillTemplate {
    pub skill_id: i32,
    pub name: String,
    pub skill_level: i32,
    pub skill_number: i32,     // internal ID for the skill icon
    pub mp_consume: i32,
    pub hp_consume: i32,
    pub item_consume_id: i32,
    pub item_consume_count: i32,
    pub reuse_delay: i32,      // cooldown in ms
    pub buff_duration: i32,    // buff duration in seconds (0 = instant)
    pub target: String,        // "none","attack","buff","self" etc.
    pub target_to: i32,        // target type enum
    pub damage_value: i32,
    pub damage_dice: i32,
    pub damage_dice_count: i32,
    pub probability_value: i32,
    pub attr: i32,             // element: 0=none,1=earth,2=fire,4=water,8=wind
    pub skill_type: i32,       // 0=active, 1=passive
    pub is_through: bool,      // penetrates barriers
    pub range: i32,
    pub area: i32,             // AoE range
    pub action_id: i32,        // casting animation
    pub cast_gfx: i32,        // casting visual effect
    pub cast_gfx2: i32,       // secondary effect
    pub sys_msg_id_happen: i32,
    pub sys_msg_id_stop: i32,
    pub sys_msg_id_fail: i32,
}

/// Active buff/debuff effect on an entity.
#[derive(Debug, Clone)]
pub struct ActiveEffect {
    pub skill_id: i32,
    pub remaining_ticks: i32,  // ticks left (0 = permanent until removed)
    pub value: i32,            // effect strength (e.g., AC bonus, damage per tick)
}

/// Skill effects component - tracks all active buffs/debuffs on an entity.
#[derive(Debug, Clone)]
pub struct SkillEffects {
    /// Active effects keyed by skill_id.
    pub effects: HashMap<i32, ActiveEffect>,
}

impl SkillEffects {
    pub fn new() -> Self {
        SkillEffects {
            effects: HashMap::new(),
        }
    }

    /// Add or refresh a buff/debuff.
    pub fn add_effect(&mut self, skill_id: i32, duration_ticks: i32, value: i32) {
        self.effects.insert(skill_id, ActiveEffect {
            skill_id,
            remaining_ticks: duration_ticks,
            value,
        });
    }

    /// Remove an effect by skill_id.
    pub fn remove_effect(&mut self, skill_id: i32) -> bool {
        self.effects.remove(&skill_id).is_some()
    }

    /// Check if a specific effect is active.
    pub fn has_effect(&self, skill_id: i32) -> bool {
        self.effects.contains_key(&skill_id)
    }

    /// Tick all effects, removing expired ones. Returns list of expired skill IDs.
    pub fn tick(&mut self) -> Vec<i32> {
        let mut expired = Vec::new();
        self.effects.retain(|&skill_id, effect| {
            if effect.remaining_ticks == 0 {
                return true; // permanent, keep
            }
            effect.remaining_ticks -= 1;
            if effect.remaining_ticks == 0 {
                expired.push(skill_id);
                false // remove
            } else {
                true // keep
            }
        });
        expired
    }

    /// Get total stat modifier from all active effects for a given stat type.
    pub fn get_stat_modifier(&self, _stat_type: &str) -> i32 {
        // Simplified: sum all effect values
        // In full implementation, each skill_id maps to specific stat effects
        self.effects.values().map(|e| e.value).sum()
    }
}

/// Known skill IDs for common skills (from Java L1SkillId).
pub mod skill_ids {
    // Mage skills
    pub const ENERGY_BOLT: i32 = 1;
    pub const LIGHT: i32 = 2;
    pub const SHIELD: i32 = 3;
    pub const TELEPORT: i32 = 4;
    pub const ICE_DAGGER: i32 = 6;
    pub const WIND_CUTTER: i32 = 7;
    pub const HEAL: i32 = 9;
    pub const CHILL_TOUCH: i32 = 10;
    pub const CURSE_PARALYZE: i32 = 11;
    pub const HASTE: i32 = 14;
    pub const FIREBALL: i32 = 17;
    pub const GREATER_HEAL: i32 = 19;
    pub const ENCHANT_WEAPON: i32 = 22;
    pub const DETECTION: i32 = 24;
    pub const LIGHTNING: i32 = 26;
    pub const TURN_UNDEAD: i32 = 27;
    pub const GREATER_HASTE: i32 = 31;
    pub const CANCELLATION: i32 = 35;
    pub const EARTH_JAIL: i32 = 36;
    pub const COUNTER_BARRIER: i32 = 38;
    pub const HOLY_WALK: i32 = 42;
    pub const CALL_LIGHTNING: i32 = 44;
    pub const FULL_HEAL: i32 = 46;
    pub const FREEZING_BLIZZARD: i32 = 49;
    pub const METEOR_STRIKE: i32 = 54;
    pub const ABSOLUTE_BARRIER: i32 = 56;

    // Status effects
    pub const STATUS_HASTE: i32 = 1000;
    pub const STATUS_BRAVE: i32 = 1001;
    pub const STATUS_THIRD_SPEED: i32 = 1031;
    pub const STATUS_POISON: i32 = 1060;

    // Item effects
    pub const COOKING_1_7_N: i32 = 3000;
    pub const COOKING_1_7_S: i32 = 3006;
}

/// Skill cooldown tracker.
#[derive(Debug, Clone)]
pub struct SkillCooldowns {
    /// skill_id -> ticks remaining until usable
    pub cooldowns: HashMap<i32, i32>,
}

impl SkillCooldowns {
    pub fn new() -> Self {
        SkillCooldowns {
            cooldowns: HashMap::new(),
        }
    }

    pub fn set_cooldown(&mut self, skill_id: i32, ticks: i32) {
        self.cooldowns.insert(skill_id, ticks);
    }

    pub fn is_ready(&self, skill_id: i32) -> bool {
        !self.cooldowns.contains_key(&skill_id) || self.cooldowns[&skill_id] == 0
    }

    pub fn tick(&mut self) {
        self.cooldowns.retain(|_, ticks| {
            *ticks = ticks.saturating_sub(1);
            *ticks > 0
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_check_effect() {
        let mut effects = SkillEffects::new();
        effects.add_effect(skill_ids::HASTE, 50, 1);

        assert!(effects.has_effect(skill_ids::HASTE));
        assert!(!effects.has_effect(skill_ids::SHIELD));
    }

    #[test]
    fn test_effect_expiry() {
        let mut effects = SkillEffects::new();
        effects.add_effect(skill_ids::SHIELD, 3, 2); // 3 ticks

        let expired = effects.tick(); // 2 remaining
        assert!(expired.is_empty());

        let expired = effects.tick(); // 1 remaining
        assert!(expired.is_empty());

        let expired = effects.tick(); // 0 remaining -> expire
        assert_eq!(expired, vec![skill_ids::SHIELD]);
        assert!(!effects.has_effect(skill_ids::SHIELD));
    }

    #[test]
    fn test_permanent_effect() {
        let mut effects = SkillEffects::new();
        effects.add_effect(99, 0, 5); // permanent (0 ticks)

        for _ in 0..100 {
            effects.tick();
        }
        assert!(effects.has_effect(99)); // still active
    }

    #[test]
    fn test_cooldown() {
        let mut cd = SkillCooldowns::new();
        cd.set_cooldown(skill_ids::FIREBALL, 5);

        assert!(!cd.is_ready(skill_ids::FIREBALL));

        for _ in 0..4 { cd.tick(); }
        assert!(!cd.is_ready(skill_ids::FIREBALL));

        cd.tick();
        assert!(cd.is_ready(skill_ids::FIREBALL));
    }
}
