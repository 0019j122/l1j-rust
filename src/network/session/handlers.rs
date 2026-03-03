use super::{Session, SessionState};
use crate::protocol::opcodes::server::{S_OPCODE_CHARLIST, S_OPCODE_INITPACKET};
use crate::protocol::packet::PacketBuilder;
use anyhow::Result;
use log::info;
use tokio::io::AsyncWriteExt;

/// 1. 握手階段
// src/network/session/handlers.rs

pub async fn send_handshake(session: &mut Session) -> Result<()> {
    let key: i32 = 0x12345678;

    // 🔹 建立標準 0x33 內容 (13 bytes)
    let pkt = PacketBuilder::new(0x33) // S_OPCODE_INITPACKET
        .write_d(key)
        .write_d(0x01)
        .write_d(0x00)
        .build();

    // 🔹 關鍵：手動發送「明文」長度標頭 +「明文」內容
    let len = (pkt.len() + 2) as u16;
    session.stream.write_all(&len.to_le_bytes()).await?; // 明文長度標頭
    session.stream.write_all(&pkt).await?; // 明文 0x33 內容
    session.stream.flush().await?;

    // 🔹 初始化 Cipher，確保 read/write key 都處於初始狀態
    session.cipher = Some(crate::network::cipher::Cipher::new(key));

    info!("【握手重新嘗試】已發送帶標頭的明文 0x33，Key: {:08X}", key);
    Ok(())
}
/// 2. 處理剛連線時的階段 (處理 9D -> 發送 8C)
// src/network/session/handlers.rs

// handlers.rs
// src/network/session/handlers.rs
pub async fn handle_connected(session: &mut Session, opcode: u8, _data: &[u8]) -> Result<()> {
    if opcode == 0x9D {
        info!("收到 9D，發送強效對號 8C (跳過版本檢查)...");

        let pkt = PacketBuilder::new(0x8C)
            .write_c(0x00) // Status: 00 (成功)
            .write_d(-1) // 🔹 使用 -1 作為萬用版本號
            .write_d(0x52213791) // 🔹 固定時間戳，避免動態時間造成的誤差
            .write_c(0x00) // 補位 byte
            .build();

        session.send_packet(&pkt).await?;
        session.state = SessionState::VersionVerified;
        info!("【對號完畢】已發送 8C，等待 07 封包...");
    }
    Ok(())
}

/// 3. 處理登入階段 (處理 07 -> 發送登入序列)
pub async fn handle_login_process(session: &mut Session, opcode: u8, data: &[u8]) -> Result<()> {
    if opcode == 0x07 {
        info!("🔥 抓到帳號輸入！資料長度: {} bytes", data.len());

        // 🔹 步驟 1: 發送登入結果 (S_LoginResult)
        // 許多 3.80c 核心使用 0x27 作為 LoginResult Opcode
        let login_ok = PacketBuilder::new(0x27)
            .write_c(0x00) // 0: 登入成功
            .build();
        session.send_packet(&login_ok).await?;

        // 🔹 步驟 2: 發送角色數量 (S_CharAmount)
        // 這是跳轉到選人畫面的關鍵，必須告知客戶端目前有幾個角色
        let char_amount = PacketBuilder::new(0x1B) // 3.80c 常用的 CharAmount Opcode
            .write_c(0x00) // 角色數量：0
            .write_c(0x08) // 角色欄位上限：8
            .build();
        session.send_packet(&char_amount).await?;

        // 🔹 步驟 3: 發送角色列表 (S_CharList)
        send_char_list(session).await?;

        // 🔹 步驟 4: 切換狀態
        session.state = SessionState::Authenticated;
        info!("【登入序列完成】已切換至 Authenticated 狀態。");
    }
    Ok(())
}
/// 4. 其他狀態接口
pub async fn handle_authenticated(_session: &mut Session, opcode: u8, _data: &[u8]) -> Result<()> {
    info!("已驗證狀態收到 Opcode: {:02X}", opcode);
    Ok(())
}

pub async fn handle_in_game(_session: &mut Session, opcode: u8, _data: &[u8]) -> Result<()> {
    info!("遊戲中收到 Opcode: {:02X}", opcode);
    Ok(())
}

/// 發送角色清單 (僅定義一次)
async fn send_char_list(session: &mut Session) -> Result<()> {
    let pkt = PacketBuilder::new(S_OPCODE_CHARLIST)
        .write_c(0x00) // 0 個角色
        .write_c(0x00)
        .build();
    session.send_packet(&pkt).await?;
    Ok(())
}
