/// S_NPCPack - NPC appearance packet sent to clients.
///
/// Ported from Java S_NPCPack.java. Sent when a player first sees
/// an NPC or when the NPC enters the player's screen.
use crate::models::npc::NpcTemplate;
use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

pub struct NpcPosition {
    pub x: i32,
    pub y: i32,
    pub heading: i32,
}
/// Build S_NPCPack for a single NPC.
pub fn build_npc_pack(
    object_id: i32,
    pos: &NpcPosition, // 改用專用的位置結構
    template: &NpcTemplate,
    cur_hp: i32,
    max_hp: i32,
    status_flags: i32,
) -> Vec<u8> {
    let gfx_id = template.gfxid;
    
    // 天堂的 HP Bar 計算：0 為無，1~255 為比例
    let hp_percent = if max_hp > 0 {
        let percent = (cur_hp as f32 / max_hp as f32) * 255.0;
        percent.clamp(0.0, 255.0) as i32
    } else {
        0xFF
    };

    PacketBuilder::new(server::S_OPCODE_CHARPACK)
        .write_h(pos.x.into())             // 👈 使用 .into() 自動轉 i32
        .write_h(pos.y.into())
        .write_d(object_id) 
        .write_h(gfx_id.into())
        .write_c(0)                        
        .write_c(pos.heading.into())       
        .write_c(template.light_size.into())
        .write_c(0)                        
        .write_d(template.exp)             
        .write_h(template.lawful.into())   
        .write_s(Some(&template.nameid))   
        .write_s(Some(""))                 
        .write_c(status_flags.into())      
        .write_d(0)                        
        .write_s(None)                     
        .write_s(None)                     
        .write_c(0)                        
        .write_c(hp_percent.into())        
        .write_c(0)                        
        .write_c(template.level.into())    
        .write_c(0xFF)                     
        .write_c(0xFF)                     
        .write_c(0)                        
        .build()
}
/// Build S_REMOVE_OBJECT - 刪除畫面上的物件
pub fn build_remove_object(object_id: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_REMOVE_OBJECT)
        .write_d(object_id)
        .build()
}