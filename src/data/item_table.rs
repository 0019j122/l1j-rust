use std::collections::HashMap;
use sqlx::{MySqlPool, Row};
use anyhow::Result;
use crate::models::item::ItemTemplate;

pub struct ItemTable {
    pub templates: HashMap<i32, ItemTemplate>,
}

impl ItemTable {
    pub async fn load(pool: &MySqlPool) -> Result<Self> {
        let rows = sqlx::query(
            "SELECT item_id, name, stackable FROM item"
        )
        .fetch_all(pool)
        .await?;

        let mut templates = HashMap::with_capacity(rows.len());

        for r in rows {
            let id: i32 = r.try_get("item_id")?;
            let template = ItemTemplate {
                stackable: r.try_get::<i32, _>("stackable").unwrap_or(0) != 0,
                item_id: id,
                name: r.try_get("name").unwrap_or_default(),
            
                ..Default::default() 
            };
            templates.insert(id, template);
        }
        Ok(ItemTable { templates })
    }
}