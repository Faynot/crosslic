use std::{net::SocketAddr, thread};
use warp::Filter;
use web_view::{Content, builder};

/// Starts the server and WebView
pub fn run_app(
    static_dir: impl Into<String>,
    ws_path: impl Into<String>,
    addr: SocketAddr,
    title: impl Into<String>,
    width: i32,
    height: i32,
) {
    let static_dir = static_dir.into();
    let ws_path = ws_path.into();
    let title = title.into();

    // Preparing routes
    let ws_route = warp::path(ws_path.trim_start_matches('/').to_string())
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|socket| async move {
                crate::websocket::handle_websocket(socket).await;
            })
        });

    let routes = warp::fs::dir(static_dir.clone()).or(ws_route);

    // Server in a separate thread
    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {
            warp::serve(routes).run(addr).await;
        });
    });

    // URL with cache buster
    let url = format!(
        "http://{host}:{port}/?t={time}",
        host = addr.ip(),
        port = addr.port(),
        time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    // WebView
    builder()
        .title(&title)
        .content(Content::Url(url))
        .size(width, height)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_, _| Ok(()))
        .run()
        .unwrap();
}
