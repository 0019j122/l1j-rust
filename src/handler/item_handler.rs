// 道具處理核心
pub async fn use_item(player: &mut Player, item: &ItemInstance, world: &SharedWorld) {
    match item.item_id {
        40006 => { // 松木魔杖
            spawn_goblin_squad(player, world).await;
            // 記得扣除次數或消耗道具
            consume_wand_charge(player, item).await;
        }
        _ => info!("未使用邏輯的道具: {}", item.item_id),
    }
}

async fn spawn_goblin_squad(player: &Player, world: &SharedWorld) {
    let mut w = world.lock().unwrap(); // 取得世界的控制權
    
    // 定義陣型：在大臣身邊生出 3 隻哥布林
    for i in -1..=1 {
        let squad_member_id = w.generate_object_id();
        
        // 取得哥布林模板 (從你辛苦載入的 NpcTable)
        if let Some(template) = w.npc_templates.get(&45001) {
            let mut goblin = OnlineNpc::from_template(template);
            
            goblin.object_id = squad_member_id;
            goblin.x = player.x + i; // 橫向排開
            goblin.y = player.y + 1; // 站在大臣面前
            goblin.map_id = player.map_id;
            
            // 賦予靈魂：設定 AI
            goblin.ai_state = AiState::Aggressive; 
            goblin.master_id = Some(player.object_id); // 認你當老大
            
            // 投射到戰場
            w.npcs.insert(squad_member_id, goblin);
            
            // 發送封包：讓客戶端畫面上出現哥布林
            w.broadcast_spawn(squad_member_id);
        }
    }
    println!("大臣指令：哥布林小隊已抵達戰場！");
}

// 想像在 src/handler/item_handler.rs 
fn handle_wand(player: &mut Player, item: &Item) {
    match item.id {
        松木魔杖_ID => {
            // 1. 檢查次數 (Charges)
            // 2. 呼叫召喚函數
            spawn_from_wand(player);
        }
        _ => {}
    }
}
// 召喚哥布林小隊的邏輯  (40006, 0.001, 1, '松木魔杖')
pub fn spawn_goblin_squad(world: &mut SharedWorld, player: &Player) {
    let squad_size = 5;
    let (px, py) = (player.x, player.y);
    
    for i in 0..squad_size {
        // 在玩家座標附近產生偏移，讓他們排成一列
        let offset_x = i - 2; 
        
        let mut goblin = world.create_npc(45001); // 7.6C 的哥布林 ID
        goblin.x = px + offset_x;
        goblin.y = py - 1; // 站在玩家後面一格
        goblin.map_id = player.map_id;
        
        // 關鍵：賦予「出征」狀態
        goblin.ai_state = AiState::Aggressive;
        
        world.spawn_active_npc(goblin);
    }
}  

  // 分級制度：普通的松木魔杖噴哥布林，高級的魔杖噴「食人妖精」或「黑騎士」。 
// 高級魔杖召喚邏輯
match wand_item_id {
    40006 => spawn_squad(world, player, 45001, 5).await, // 普通松木：哥布林小隊
    40007 => spawn_squad(world, player, 45015, 3).await, // 強化松木：黑騎士親衛隊
    40008 => spawn_squad(world, player, 45020, 1).await, // 大臣權杖：食人妖精王
    _ => (),
}
  //進化制度,玩家沒看到,不代表怪物不存在閒置中不如升級 。 現代PC電腦都有GPU_4~8G
// 偽代碼：在 AI Tick 系統中
for npc in world.npcs.values_mut() {
    if !npc.is_in_combat() && npc.last_player_contact_time.elapsed() > 300 {
        // 閒置超過 5 分鐘，觸發進化判定
        npc.idle_ticks += 1;
        
        if npc.idle_ticks > 1000 { // 累積夠久了
            npc.transform_to_super_mode(); // 兔子 -> 狂暴巨兔
            npc.level += 5;
            npc.name = format!("活太久的{}", npc.template_name);
            // 經驗值雖然不變，但強度嚇死人
        }
    }
}

  //人物、NPC不會重疊，所以每個NPC都要有自己的座標和狀態。  
fn get_safe_spawn_coords(world: &WorldState, target_x: i32, target_y: i32) -> (i32, i32) {
    // 檢查該座標是否已有「人物」或「NPC」
    if world.is_cell_occupied(target_x, target_y) {
        // 如果重疊了，往旁邊找空位 (Spiral Search)
        return find_nearest_empty_cell(world, target_x, target_y);
    }
    (target_x, target_y)
}

// 偽代碼：防止疊怪的移動邏輯
fn try_move_to(&mut self, target_x: i32, target_y: i32, world: &WorldState) -> bool {
    if world.is_occupied(target_x, target_y) {
        // 如果有人，絕對不疊上去
        let alternative = self.find_nearby_empty_space(world);
        self.move_to(alternative);
        return false;
    }
    self.coords = (target_x, target_y);
    true
}
