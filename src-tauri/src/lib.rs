use std::sync::Mutex;

use tauri::Manager;

mod check_nat;
mod keen_client;
mod migrations;
mod settings;

struct AppState<'a> {
    keen_client: &'a reqwest::blocking::Client,
}

pub fn init_rest_client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .cookie_store(true)
        .user_agent("curl")
        .build()
        .expect("Failed to create HTTP client for Router")
}

impl AppState<'static> {
    pub fn new() -> Self {
        let client = init_rest_client();

        Self {
            keen_client: Box::leak(Box::new(client)),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::new()));
            Ok(())
        })
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations("sqlite:data.db", migrations::m())
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            keen_client::keen_run,
            check_nat::check_nat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
