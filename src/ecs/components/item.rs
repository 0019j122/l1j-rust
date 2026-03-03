/// Item template and instance data.
///
/// Three item categories: EtcItem (type2=0), Weapon (type2=1), Armor (type2=2).

/// Item category.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemType2 {
    EtcItem = 0,
    Weapon = 1,
    Armor = 2,
}

/// Item template data loaded from etcitem/weapon/armor tables.
#[derive(Debug, Clone)]
pub struct ItemTemplate {
    pub item_id: i32,
    pub name: String,
    pub type2: ItemType2,
    pub item_type: i32,          // sub-type within category
    pub use_type: i32,
    pub material: i32,
    pub weight: i32,
    pub inv_gfx_id: i32,        // inventory graphic
    pub ground_gfx_id: i32,     // dropped-on-ground graphic
    pub item_desc_id: i32,
    pub unidentified_name_id: String,
    pub identified_name_id: String,
    pub min_level: i32,
    pub max_level: i32,
    pub bless: i32,
    pub tradable: bool,
    pub cant_delete: bool,
    pub stackable: bool,         // EtcItem only
    pub max_charge_count: i32,

    // Weapon fields
    pub dmg_small: i32,
    pub dmg_large: i32,
    pub range: i32,
    pub safe_enchant: i32,
    pub hit_modifier: i32,
    pub dmg_modifier: i32,
    pub double_dmg_chance: i32,
    pub magic_dmg_modifier: i32,

    // Armor fields
    pub ac: i32,
    pub damage_reduction: i32,
    pub weight_reduction: i32,

    // Shared stat bonuses
    pub add_str: i32,
    pub add_dex: i32,
    pub add_con: i32,
    pub add_int: i32,
    pub add_wis: i32,
    pub add_cha: i32,
    pub add_hp: i32,
    pub add_mp: i32,
    pub add_hpr: i32,
    pub add_mpr: i32,
    pub add_sp: i32,
    pub m_def: i32,

    // Class restrictions
    pub use_royal: bool,
    pub use_knight: bool,
    pub use_elf: bool,
    pub use_mage: bool,
    pub use_darkelf: bool,
    pub use_dragonknight: bool,
    pub use_illusionist: bool,

    pub haste_item: bool,
    pub max_use_time: i32,
    pub food_volume: i32,        // EtcItem only
}

impl Default for ItemTemplate {
    fn default() -> Self {
        ItemTemplate {
            item_id: 0, name: String::new(), type2: ItemType2::EtcItem,
            item_type: 0, use_type: 0, material: 0, weight: 0,
            inv_gfx_id: 0, ground_gfx_id: 0, item_desc_id: 0,
            unidentified_name_id: String::new(), identified_name_id: String::new(),
            min_level: 0, max_level: 0, bless: 1, tradable: true,
            cant_delete: false, stackable: false, max_charge_count: 0,
            dmg_small: 0, dmg_large: 0, range: 0, safe_enchant: 0,
            hit_modifier: 0, dmg_modifier: 0, double_dmg_chance: 0,
            magic_dmg_modifier: 0, ac: 0, damage_reduction: 0,
            weight_reduction: 0, add_str: 0, add_dex: 0, add_con: 0,
            add_int: 0, add_wis: 0, add_cha: 0, add_hp: 0, add_mp: 0,
            add_hpr: 0, add_mpr: 0, add_sp: 0, m_def: 0,
            use_royal: false, use_knight: false, use_elf: false,
            use_mage: false, use_darkelf: false, use_dragonknight: false,
            use_illusionist: false, haste_item: false, max_use_time: 0,
            food_volume: 0,
        }
    }
}

/// A single item instance owned by a character or on the ground.
#[derive(Debug, Clone)]
pub struct ItemInstance {
    pub object_id: i32,
    pub item_id: i32,            // references ItemTemplate
    pub count: i32,
    pub is_equipped: bool,
    pub enchant_level: i32,
    pub is_identified: bool,
    pub durability: i32,
    pub charge_count: i32,
    pub remaining_time: i32,
    pub bless: i32,
    pub attr_enchant_kind: i32,  // 1=earth,2=fire,4=water,8=wind
    pub attr_enchant_level: i32,
}

impl ItemInstance {
    pub fn new(object_id: i32, item_id: i32) -> Self {
        ItemInstance {
            object_id,
            item_id,
            count: 1,
            is_equipped: false,
            enchant_level: 0,
            is_identified: false,
            durability: 0,
            charge_count: 0,
            remaining_time: 0,
            bless: 1,
            attr_enchant_kind: 0,
            attr_enchant_level: 0,
        }
    }

    /// Get the display name including enchant prefix.
    pub fn get_view_name(&self, template: &ItemTemplate) -> String {
        let base = &template.name;
        if self.enchant_level > 0 {
            format!("+{} {}", self.enchant_level, base)
        } else {
            base.clone()
        }
    }

    /// Get effective weight (count * unit weight / 1000, min 1).
    pub fn get_weight(&self, template: &ItemTemplate) -> i32 {
        let w = self.count as i64 * template.weight as i64 / 1000;
        w.max(1) as i32
    }
}

/// Equipment slot indices matching the L1J client.
pub mod equip_slot {
    pub const WEAPON: usize = 8;
    pub const HELM: usize = 1;
    pub const TSHIRT: usize = 2;
    pub const ARMOR: usize = 3;
    pub const CLOAK: usize = 4;
    pub const BOOTS: usize = 5;
    pub const GLOVE: usize = 6;
    pub const SHIELD: usize = 7;
    pub const AMULET: usize = 10;
    pub const BELT: usize = 11;
    pub const EARRING: usize = 12;
    pub const RING_LEFT: usize = 18;
    pub const RING_RIGHT: usize = 19;
}

/// Player inventory.
#[derive(Debug, Clone)]
pub struct Inventory {
    pub items: Vec<ItemInstance>,
    pub max_size: usize,
    pub max_weight: i32,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            items: Vec::new(),
            max_size: 180,
            max_weight: 300_000, // will be recalculated from STR/CON
        }
    }

    /// Add an item to inventory. Merges stackables.
    pub fn add_item(&mut self, item: ItemInstance, template: &ItemTemplate) -> bool {
        if template.stackable {
            if let Some(existing) = self.items.iter_mut().find(|i| i.item_id == item.item_id) {
                existing.count += item.count;
                return true;
            }
        }
        if self.items.len() >= self.max_size {
            return false;
        }
        self.items.push(item);
        true
    }

    /// Remove a specific count of an item. Returns true if successful.
    pub fn remove_item(&mut self, object_id: i32, count: i32) -> bool {
        if let Some(pos) = self.items.iter().position(|i| i.object_id == object_id) {
            if self.items[pos].count <= count {
                self.items.remove(pos);
            } else {
                self.items[pos].count -= count;
            }
            true
        } else {
            false
        }
    }

    /// Check if inventory contains item_id with at least `count`.
    pub fn check_item(&self, item_id: i32, count: i32) -> bool {
        let total: i32 = self.items.iter()
            .filter(|i| i.item_id == item_id)
            .map(|i| i.count)
            .sum();
        total >= count
    }

    /// Find an item instance by object_id.
    pub fn get_item(&self, object_id: i32) -> Option<&ItemInstance> {
        self.items.iter().find(|i| i.object_id == object_id)
    }

    /// Find an item instance by item_id.
    pub fn find_item_id(&self, item_id: i32) -> Option<&ItemInstance> {
        self.items.iter().find(|i| i.item_id == item_id)
    }

    /// Get total weight of all items.
    pub fn get_total_weight(&self, templates: &std::collections::HashMap<i32, ItemTemplate>) -> i32 {
        self.items.iter()
            .filter_map(|item| {
                templates.get(&item.item_id).map(|t| item.get_weight(t))
            })
            .sum()
    }

    /// Get all currently equipped items.
    pub fn get_equipped(&self) -> Vec<&ItemInstance> {
        self.items.iter().filter(|i| i.is_equipped).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_template(item_id: i32, stackable: bool) -> ItemTemplate {
        let mut t = ItemTemplate::default();
        t.item_id = item_id;
        t.name = format!("Item_{}", item_id);
        t.weight = 10_000; // 10 per unit
        t.stackable = stackable;
        t
    }

    #[test]
    fn test_inventory_add_stackable() {
        let mut inv = Inventory::new();
        let t = test_template(40308, true);

        let item1 = ItemInstance { object_id: 1, item_id: 40308, count: 5, ..ItemInstance::new(1, 40308) };
        let item2 = ItemInstance { object_id: 2, item_id: 40308, count: 3, ..ItemInstance::new(2, 40308) };

        inv.add_item(item1, &t);
        inv.add_item(item2, &t);

        assert_eq!(inv.items.len(), 1);
        assert_eq!(inv.items[0].count, 8);
    }

    #[test]
    fn test_inventory_non_stackable() {
        let mut inv = Inventory::new();
        let t = test_template(20, false);

        let item1 = ItemInstance::new(1, 20);
        let item2 = ItemInstance::new(2, 20);

        inv.add_item(item1, &t);
        inv.add_item(item2, &t);

        assert_eq!(inv.items.len(), 2);
    }

    #[test]
    fn test_inventory_remove() {
        let mut inv = Inventory::new();
        let t = test_template(40308, true);

        let item = ItemInstance { object_id: 1, item_id: 40308, count: 10, ..ItemInstance::new(1, 40308) };
        inv.add_item(item, &t);

        inv.remove_item(1, 3);
        assert_eq!(inv.items[0].count, 7);

        inv.remove_item(1, 7);
        assert!(inv.items.is_empty());
    }

    #[test]
    fn test_check_item() {
        let mut inv = Inventory::new();
        let t = test_template(40308, true);

        let item = ItemInstance { object_id: 1, item_id: 40308, count: 5, ..ItemInstance::new(1, 40308) };
        inv.add_item(item, &t);

        assert!(inv.check_item(40308, 5));
        assert!(inv.check_item(40308, 3));
        assert!(!inv.check_item(40308, 6));
        assert!(!inv.check_item(99999, 1));
    }

    #[test]
    fn test_view_name_with_enchant() {
        let t = test_template(20, false);
        let mut item = ItemInstance::new(1, 20);
        item.enchant_level = 7;
        assert_eq!(item.get_view_name(&t), "+7 Item_20");
    }
}
