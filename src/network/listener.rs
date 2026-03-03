use crate::network::shared_state::SharedWorld;
use crate::ServerConfig;
use anyhow::Result;
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};

pub async fn start(
    config: ServerConfig,
    db_pool: Option<MySqlPool>,
    world: SharedWorld,
) -> Result<()> {
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Server listening on {}", addr);

    // 將 config 包裝在 Arc 中，避免在 loop 裡不斷重複分配記憶體
    let config_arc = Arc::new(config);

    loop {
        let (socket, client_addr) = listener.accept().await?;
        info!("New connection from {}", client_addr);

        // 複製 Arc 指標和相關狀態
        let cfg = Arc::clone(&config_arc);
        let db = db_pool.clone();
        let w = world.clone();

        // src/network/listener.rs (修改 tokio::spawn 內部的 match 區塊)

        tokio::spawn(async move {
            if let Some(db_pool_inner) = db {
                match crate::network::session::handle_session(
                    socket,
                    cfg, // 🔹 這裡換成真正的 Arc<ServerConfig>，不要用 String
                    Some(db_pool_inner),
                    w, // 🔹 傳入 world 狀態
                )
                .await
                {
                    Ok(_) => (),
                    Err(e) => error!("Session error: {}", e), // 確保使用了 tracing 或 log 的 error
                }
            } else {
                error!("Database pool is not available for {}", client_addr);
            }
        });
    }
}
