use crate::auth::KeenClient;
use crate::settings;
use crate::AppState;
use std::sync::Mutex;

fn get_interface(client: &KeenClient, rest_client: &reqwest::blocking::Client) -> String {
    match client.get("rci/show/interface/PPPoE0", rest_client) {
        Ok(res) => res
            .text()
            .unwrap_or_else(|_| "Failed to get response text".to_string()),
        Err(e) => e.to_string(),
    }
}

fn reroll_interface(
    client: &KeenClient,
    rest_client: &reqwest::blocking::Client,
    dry_run: bool,
) -> String {
    let req = match dry_run {
        true => &serde_json::json!([
            {
                "parse": "interface PPPoE0 up"
            },
        ]),
        false => &serde_json::json!([
            {
                "parse": "interface PPPoE0 down"
            },
            {
                "parse": "interface PPPoE0 up"
            },
        ]),
    };
    match client.post("rci/", req, rest_client) {
        Ok(res) => res
            .text()
            .unwrap_or_else(|_| "Failed to get response text".to_string()),
        Err(e) => e.to_string(),
    }
}

#[tauri::command]
pub fn keen_run(
    settings: Option<settings::Settings>,
    command: String,
    app_state: tauri::State<'_, Mutex<AppState>>,
) -> String {
    let settings = settings::get_settings(settings);
    println!(
        "settings: router_ip={}, user={}, dry_run={}",
        settings.router_ip, settings.user, settings.dry_run
    );
    let rest_client = app_state.lock().unwrap().keen_client;
    let client = KeenClient::new(settings.router_ip).expect("Failed to create HTTP client");
    match client.auth(&settings.user, &settings.pass, rest_client) {
        Ok(authenticated) => {
            if !authenticated {
                return serde_json::json!({
                    "detail": "Authentication failed"
                })
                .to_string();
            }
            match command.as_str() {
                "get_interface" => get_interface(&client, rest_client),
                "reroll" => reroll_interface(&client, rest_client, settings.dry_run),
                _ => "Unknown command".to_string(),
            }
        }
        Err(e) => e.to_string(),
    }
}
