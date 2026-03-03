use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tracing::{info, warn};
use anyhow::Result;

// 1. 在新版 rand 中，只要這樣導入即可
use rand::Rng;

pub mod player;
pub mod npc;
pub mod item;

#[derive(Clone, Default)]
pub struct Recognizer {
    base_accuracy: f64,
    attempts: usize,
}

impl Recognizer {
   pub fn recognize(&mut self, input: &str) -> bool {
        self.attempts += 1;
        
        let add_bonus = (self.attempts as f64 * 0.05).min(0.2);
        let success_prob = (self.base_accuracy + add_bonus).min(1.0);
        
        // ✨ 修正：直接使用 rand::random_bool，不用建立 rng 物件，也不用導入 Rng Trait
        let success = rand::random_bool(success_prob);

        if success {
            info!("辨識成功: {} (率: {:.1}%)", input, success_prob * 100.0);
        } else {
            warn!("辨識失敗: {} (嘗試 {}/10, 率: {:.1}%)", input, self.attempts.min(10), success_prob * 100.0);
        }
        success
    }

    pub fn batch_recognize(&mut self, inputs: Vec<&str>) -> (usize, f64) {
        let mut successes = 0;
        let total = inputs.len();
        for input in inputs {
            if self.recognize(input) {
                successes += 1;
            }
        }
        let rate = if total == 0 { 0.0 } else { (successes as f64 / total as f64) * 100.0 };
        info!("批次辨識完成: {} 成功 / {} 總數 (率: {:.1}%)", successes, total, rate);
        (successes, rate)
    }

    pub fn check_success(success_prob: f64) -> bool {
        // ✨ 同樣使用函數式寫法
        rand::random_bool(success_prob.clamp(0.0, 1.0))
    }

    
}

/// 處理 TCP 連線
pub async fn handle_connection(stream: TcpStream, recognizer: Arc<Mutex<Recognizer>>) -> Result<()> {
    let addr = stream.peer_addr()?;
    info!("新連線處理中: {}", addr);

    let (_successes, rate) = {
        let mut recognizer_locked = recognizer.lock().unwrap();
        let player_inputs = vec!["player_001", "item_sword", "npc_encounter"];
        recognizer_locked.batch_recognize(player_inputs)
    };

    if rate > 70.0 {
        info!("玩家辨識率良好 ({}%), 允許登入", rate);
    } else {
        warn!("辨識率低 ({}%), 拒絕連線", rate);
    }

    Ok(())
}