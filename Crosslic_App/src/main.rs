// Crosslic_App/src/main.rs

pub mod commands;

use crosslic::run_app;
use include_dir::{Dir, include_dir};
use serde::Deserialize;
use std::{net::SocketAddr, process::Command};
use tempfile::TempDir;

// Встраиваем папку frontend/dist и сам файл конфига на этапе компиляции
static FRONTEND_DIST: Dir = include_dir!("$CARGO_MANIFEST_DIR/../frontend/dist");
const CONFIG_JSON: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.crosslic.json"));

#[derive(Debug, Deserialize)]
struct AppConfig {
    // путь к фронтенду больше не нужен на этапе запуска
    websocket: String,
    url: String,
    name: String,
    resolution: [String; 2],
    client_start: Option<String>,
}

fn main() {
    // Десериализуем config из встроенной строки
    let config: AppConfig =
        serde_json::from_str(CONFIG_JSON).expect("Failed to parse embedded config.crosslic.json");

    // Парсим адрес и разрешение
    let addr: SocketAddr = config.url.parse().expect("Invalid URL format");
    let [width, height] = config
        .resolution
        .map(|v| v.parse().expect("Invalid resolution"));

    // При необходимости запускаем внешний dev‑сервер (для разработки)
    if let Some(start_cmd) = &config.client_start {
        let parts: Vec<&str> = start_cmd.split_whitespace().collect();
        if !parts.is_empty() {
            let mut cmd = Command::new(parts[0]);
            if parts.len() > 1 {
                cmd.args(&parts[1..]);
            }
            cmd.spawn()
                .expect("Failed to launch frontend client process");
        }
    }

    // Распаковываем встроенный фронтенд во временную папку
    let temp_dir = TempDir::new().expect("Could not create temp dir for frontend");
    FRONTEND_DIST
        .extract(temp_dir.path())
        .expect("Failed to extract embedded frontend files");

    // Запускаем сервер + WebView, передавая путь к распакованному фронтенду
    run_app(
        temp_dir.path().to_string_lossy().into_owned(),
        config.websocket,
        addr,
        config.name,
        width,
        height,
    );

    // Чтобы временная папка не удалялась до завершения работы приложения
    std::mem::forget(temp_dir);
}
