use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BeginnerItem {
    pub item_id: i32,
    pub count: i32,
    pub enchant: i32,
}

pub struct BeginnerItemTable {
    // Key 是職業 ID (例如: 0=王族, 1=騎士...), Value 是初始物品清單
    pub class_items: HashMap<i32, Vec<BeginnerItem>>,
}

impl BeginnerItemTable {
    pub fn new() -> Self {
        let mut class_items = HashMap::new();

        // 範例：給騎士 (假設 ID 為 1) 一些新手裝備
        class_items.insert(1, vec![
            BeginnerItem { item_id: 40001, count: 1, enchant: 0 }, // 象牙塔長劍
            BeginnerItem { item_id: 40010, count: 10, enchant: 0 }, // 紅色藥水
        ]);

        BeginnerItemTable { class_items }
    }

    pub fn get_items_for_class(&self, class_id: i32) -> Option<&Vec<BeginnerItem>> {
        self.class_items.get(&class_id)
    }
}