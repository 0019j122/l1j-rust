// src/db/mod.rs
pub mod pool;
pub mod account;
pub mod character;
pub mod char_create;

// 這一行會把 pool.rs 裡的函數拉出來，讓 main.rs 可以用 l1j_rust::db::init_db_pool 呼叫
pub use self::pool::init_db_pool;