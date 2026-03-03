pub mod handlers;

use crate::config::ServerConfig;
use crate::network::cipher::Cipher;
use crate::network::shared_state::SharedWorld;
use anyhow::Result;
use log::{error, info};
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

// 🔹 定義連線狀態
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionState {
    Handshake,
    VersionVerified,
    Authenticated,
    InGame,
}

// 🔹 定義 Session 容器
pub struct Session {
    pub stream: TcpStream,
    pub cipher: Option<Cipher>,
    pub state: SessionState,
}

impl Session {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            cipher: None,
            state: SessionState::Handshake,
        }
    }

    // 發送加密封包
    // src/network/session/mod.rs

    // src/network/session/mod.rs 中的 send_packet 邏輯
    pub async fn send_packet(&mut self, data: &[u8]) -> Result<()> {
        use tokio::io::AsyncWriteExt;

        // 1. 複製原始資料
        let mut encrypted_payload = data.to_vec();

        // 2. 🔹 關鍵：只對內容加密，這會增加 Cipher 的內部 Step
        if let Some(ref mut cipher) = self.cipher {
            cipher.encrypt(&mut encrypted_payload);
        }

        // 3. 計算總長度 (Payload + 2 bytes 標頭)
        let total_len = (encrypted_payload.len() + 2) as u16;

        // 4. 🔹 必須先發送「明文」的長度標頭
        self.stream.write_all(&total_len.to_le_bytes()).await?;

        // 5. 再發送加密內容
        self.stream.write_all(&encrypted_payload).await?;
        self.stream.flush().await?;

        Ok(())
    }

    // src/network/session/mod.rs

    pub async fn process_packets(&mut self) -> Result<()> {
        loop {
            // 1. 讀取封包前 2 bytes (這代表整個封包的長度，包含這 2 bytes 自身)
            let mut head = [0u8; 2];
            if let Err(_) = self.stream.read_exact(&mut head).await {
                info!("客戶端已中斷連線。");
                break;
            }

            let packet_len = u16::from_le_bytes(head) as usize;

            // 3.80c 封包最小長度通常是 2 (只有長度標頭) 或更多
            if packet_len < 2 || packet_len > 2048 {
                continue;
            }

            // 2. 讀取剩餘的封包內容 (packet_len - 2)
            let data_len = packet_len - 2;
            let mut data = vec![0u8; data_len];
            if let Err(e) = self.stream.read_exact(&mut data).await {
                error!("讀取封包內容失敗: {}", e);
                break;
            }

            // 3. 解密封包 (如果加密通道已開啟)
            if let Some(ref mut cipher) = self.cipher {
                cipher.decrypt(&mut data);
            }

            // 4. 解析 Opcode 並分發
            if data.is_empty() {
                continue;
            }
            let opcode = data[0];
            let payload = &data[1..];

            // 這裡加上一個 Debug Log，幫你確認有沒有抓到 Opcode 07
            info!(
                "--- 收到封包 Opcode: {:02X}, 長度: {} ---",
                opcode, packet_len
            );

            match self.state {
                SessionState::Handshake => {
                    handlers::handle_connected(self, opcode, payload).await?;
                }
                SessionState::VersionVerified => {
                    handlers::handle_login_process(self, opcode, payload).await?;
                }
                SessionState::Authenticated => {
                    handlers::handle_authenticated(self, opcode, payload).await?;
                }
                SessionState::InGame => {
                    handlers::handle_in_game(self, opcode, payload).await?;
                }
            }
        }
        Ok(())
    }
}

// 🔹 被 listener.rs 呼叫的入口
pub async fn handle_session(
    stream: TcpStream,
    _config: Arc<ServerConfig>,
    _db_pool: Option<MySqlPool>,
    _world: SharedWorld,
) -> Result<()> {
    let mut session = Session::new(stream);

    // 1. 發送 0x33 握手
    handlers::send_handshake(&mut session).await?;

    // 2. 開始監聽客戶端回傳的 9D, 07 等封包
    if let Err(e) = session.process_packets().await {
        error!("Session loop error: {}", e);
    }

    Ok(())
}
