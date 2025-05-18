mod auth;
mod check_nat;
mod keen_client;
mod migrations;
mod settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
