use crate::settings;
use crate::auth::KeenClient;

#[tauri::command]
pub fn keen_run(settings: Option<settings::Settings>, command: String) -> String {
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
                "reroll" => {
                    let req = match settings::DRY_RUN {
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
                    match client.post("rci/", req) {
                        Ok(res) => res
                            .text()
                            .unwrap_or_else(|_| "Failed to get response text".to_string()),
                        Err(e) => e.to_string(),
                    }
                }
                _ => "Unknown command".to_string(),
            }
        }
        Err(e) => e.to_string(),
    }
}
