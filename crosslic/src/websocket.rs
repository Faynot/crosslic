use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use warp::ws::{Message, WebSocket};

/// Pull inside the warp route:
///     ws.on_upgrade(|socket| async { websocket::handle(socket).await; })
pub async fn handle_websocket(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();

    while let Some(Ok(msg)) = rx.next().await {
        if let Ok(txt) = msg.to_str() {
            if let Ok(data) = serde_json::from_str::<Value>(txt) {
                if let (Some(id), Some(cmd)) = (
                    data.get("id").and_then(|v| v.as_str()),
                    data.get("command").and_then(|v| v.as_str()),
                ) {
                    let result = crate::api::COMMANDS
                        .lock()
                        .handle(cmd, data["data"].clone());

                    let resp = match result {
                        Ok(d) => json!({ "id": id, "result": d }),
                        Err(e) => json!({ "id": id, "error": e }),
                    };

                    if let Err(e) = tx.send(Message::text(resp.to_string())).await {
                        eprintln!("WebSocket send error: {}", e);
                    }
                }
            }
        }
    }
}
