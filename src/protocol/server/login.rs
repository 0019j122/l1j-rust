use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

/// Login result codes (matching Java S_LoginResult constants).
pub const REASON_LOGIN_OK: u8 = 0x00;
pub const REASON_ACCOUNT_IN_USE: u8 = 0x16;
pub const REASON_ACCESS_FAILED: u8 = 0x08;

/// Build S_SERVERVERSION packet.
///
/// Sent in response to C_CLIENTVERSION.
/// Version constants are for 3.80c Taiwan server.
pub fn build_server_version(server_start_time: i32) -> Vec<u8> {
    let uptime = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32)
        - server_start_time;

    PacketBuilder::new(server::S_OPCODE_SERVERVERSION)
        .write_c(0x00)          // auth ok flag
        .write_c(0x01)          // server ID
        .write_d(0x07cbf4dd_i32 as i32)  // server version (3.80c TW)
        .write_d(0x07cbf4dd_i32 as i32)  // cache version
        .write_d(0x77fc692d_i32 as i32)  // auth version
        .write_d(0x07cbf4d9_i32 as i32)  // npc version
        .write_d(server_start_time)       // server start time
        .write_c(0x00)          // unknown
        .write_c(0x00)          // unknown
        .write_c(0x03)          // country: 3 = Taiwan
        .write_d(0x087f7dc2_i32 as i32)  // server type
        .write_d(uptime)        // uptime in seconds
        .write_h(0x01)          // unknown
        .build()
}

/// Build S_LOGINRESULT packet.
///
/// Sent after account authentication attempt.
pub fn build_login_result(reason: u8) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_LOGINRESULT)
        .write_c(reason as i32)
        .write_d(0x00000000)  // padding
        .write_d(0x00000000)
        .write_d(0x00000000)
        .write_d(0x00000000)
        .write_h(0x8c)       // unknown
        .build()
}

/// Build S_LOGINTOGAME packet.
///
/// Sent when the player selects a character and enters the game world.
pub fn build_login_to_game(clanid: i32, clan_member_id: i32) -> Vec<u8> {
    if clanid > 0 {
        PacketBuilder::new(server::S_OPCODE_LOGINTOGAME)
            .write_c(0x03)
            .write_d(clan_member_id)
            .write_c(0x9c_u8 as i32)
            .write_c(0x1f)
            .build()
    } else {
        PacketBuilder::new(server::S_OPCODE_LOGINTOGAME)
            .write_c(0x03)
            .write_c(0x53)
            .write_c(0x01)
            .write_c(0x00)
            .write_c(0x8b_u8 as i32)
            .write_c(0x9c_u8 as i32)
            .write_c(0x1f)
            .build()
    }
}
