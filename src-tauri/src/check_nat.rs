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

async fn _check_nat() -> anyhow::Result<CheckNatRes> {
    let ip = get_ip().await?;
    let nat = ping_ip_tcp(ip, None).await?;
    Ok(CheckNatRes {
        ip: ip.to_string(),
        nat,
    })
}

#[tauri::command(async)]
pub async fn check_nat() -> Result<CheckNatRes, CheckNatErr> {
    match _check_nat().await {
        Ok(res) => Ok(res),
        Err(e) => Err(CheckNatErr {
            detail: e.to_string(),
        }),
    }
}
