use crate::protocol::packet::PacketBuilder;
use crate::protocol::opcodes::server;
use crate::network::shared_state::SharedWorld;

// 
// --- 第一部分：封包建構函數 ---
pub fn build_move_char(id: i32, x: i32, y: i32, heading: u8) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_MOVEOBJECT)
        .write_d(id)
        .write_h(x)
        .write_h(y)
        .write_c(heading as i32) // 將 u8 方位轉為 i32 寫入封包
        .build()
}

// --- 輔助函數：計算移動增量 ---
fn heading_delta(heading: i32) -> (i32, i32) {
    match heading {
        0 => (0, -1), 1 => (1, -1), 2 => (1, 0),  3 => (1, 1),
        4 => (0, 1),  5 => (-1, 1), 6 => (-1, 0), 7 => (-1, -1),
        _ => (0, 0),
    }
}

// --- 第二部分：AI 處理邏輯 ---
pub async fn process_npc_ai(world: &SharedWorld) {
    // 建立一個臨時容器，存放「誰」要發送「什麼封包」到「哪張地圖的座標」
    let mut movements: Vec<(Vec<u8>, i32, i32, i32)> = Vec::new(); 

    // 第一步：鎖定世界狀態，計算哪些 NPC 要移動
    {
        let mut world_lock = world.lock().unwrap();
        for npc in world_lock.npcs.values_mut() {
            // 隨機走動機率
            if rand::random::<f32>() < 0.1 { 
                let new_heading = rand::random::<i32>().abs() % 8;
                let (dx, dy) = heading_delta(new_heading);
                
           // 更新座標與朝向
                npc.x += dx; 
                npc.y += dy;
                npc.heading = new_heading as i32; // 改為轉型為 i32
                
                // 產生封包並存入 movements 隊列
                let move_packet = build_move_char(
                npc.object_id as i32,
                npc.x as i32,  // 轉為 i32 以符合函數定義
                npc.y as i32,
                
                // 按照編譯器建議，在傳入 heading 時轉換：
       npc.heading.try_into().unwrap_or(0),
                
                );
                
                // 記錄封包、地圖ID、以及座標（用於後續廣播）
                movements.push((move_packet, npc.map_id as i32, npc.x as i32, npc.y as i32));
            }
        }
    } // 這裡釋放世界鎖，避免長時間佔用導致伺服器卡頓

    // 第二步：將移動封包廣播給附近的玩家
    if !movements.is_empty() {   
        let world_lock = world.lock().unwrap(); 
        for (_packet, map_id, x, y) in movements {
            // 取得附近的玩家清單
            let nearby_players = world_lock.get_nearby_players(map_id, x, y);
            for _player_session in nearby_players {
                // 假設 player_session 有一個發送封包的方法
                // player_session.send_packet(packet.clone()).await;
            }
        }
    }
}