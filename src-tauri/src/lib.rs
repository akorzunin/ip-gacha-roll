use crate::auth::KeenClient;
mod auth;
mod migrations;
mod settings;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn keen_run(settings: Option<settings::Settings>, command: String) -> String {
    println!("settings: {:?}", settings);
    let settings = settings::get_settings(settings);
    println!("settings: {:?}", settings);
    let client = KeenClient::new(settings.router_ip).expect("Failed to create HTTP client");
    match client.auth(&settings.user, &settings.pass) {
        Ok(authenticated) => {
            if !authenticated {
                return serde_json::json!({
                    "detail": "Authentication failed"
                })
                .to_string();
            }
            match command.as_str() {
                "get_interface" => match client.get("rci/show/interface/PPPoE0") {
                    Ok(res) => res
                        .text()
                        .unwrap_or_else(|_| "Failed to get response text".to_string()),
                    Err(e) => e.to_string(),
                },
                "reroll" => match client.post(
                    "rci/",
                    &serde_json::json!([
                        // {
                        //     "parse": "interface PPPoE0 down"
                        // },
                        {
                            "parse": "interface PPPoE0 up"
                        },
                    ]),
                ) {
                    Ok(res) => res
                        .text()
                        .unwrap_or_else(|_| "Failed to get response text".to_string()),
                    Err(e) => e.to_string(),
                },
                _ => "Unknown command".to_string(),
            }
        }
        Err(e) => e.to_string(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations("sqlite:data.db", migrations::m())
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![keen_run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
