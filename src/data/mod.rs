pub mod item_table;
pub mod npc_table;  // 必須有這行，npc_table.rs 才會被視為公開模組
pub mod spawn_table; // 檢查這行是否有 pub
pub use self::item_table::ItemTable; // 重新導出，方便外部引用
