use serde::{Deserialize, Serialize};

use ip_gacha_roll_shared::net_utils::{get_ip, ping_ip_tcp};

#[derive(Serialize, Deserialize)]
pub struct CheckNatRes {
    ip: String,
    nat: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CheckNatErr {
    detail: String,
}

#[tauri::command(async)]
pub async fn check_nat() -> Result<CheckNatRes, CheckNatErr> {
    let ip = match get_ip().await {
        Ok(ip) => ip,
        Err(e) => {
            return Err(CheckNatErr {
                detail: e.to_string(),
            })
        }
    };
    #[allow(clippy::manual_unwrap_or_default, clippy::manual_unwrap_or)]
    let nat = match ping_ip_tcp(ip, None).await {
        Ok(nat) => nat,
        Err(e) => {
            return Err(CheckNatErr {
                detail: e.to_string(),
            })
        }
    };
    Ok(CheckNatRes {
        ip: ip.to_string(),
        nat,
    })
}
