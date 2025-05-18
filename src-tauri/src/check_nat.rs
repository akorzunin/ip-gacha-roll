use std::{error::Error, net::Ipv4Addr, time::Duration};

use reqwest::Client;
use serde::{Deserialize, Serialize};

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
    let nat = match ping_ip(ip, None).await {
        Ok(nat) => nat,
        Err(e) => {
            return Err(CheckNatErr {
                detail: e.to_string(),
            })
        }
    };
    Ok(CheckNatRes {
        ip: ip.to_string(),
        nat: nat,
    })
}

#[derive(Serialize, Deserialize)]
struct IpRes {
    ip: String,
}

pub async fn get_ip() -> Result<Ipv4Addr, Box<dyn Error>> {
    let client = Client::new();
    let res = client
        .get("https://api.ipify.org?format=json")
        .send()
        .await?;
    let res = &res.text().await?;
    let ip_obj: IpRes = serde_json::from_str(res)?;
    let ip = ip_obj.ip.parse::<Ipv4Addr>()?;
    Ok(ip)
}

#[tokio::test]
async fn test_get_ip() {
    let ip = get_ip().await.unwrap();
    println!("ip: {}", ip.to_string());
    assert_eq!(ip.is_loopback(), false);
}

pub async fn ping_ip(ip: Ipv4Addr, timeout: Option<Duration>) -> Result<bool, Box<dyn Error>> {
    let timeout = timeout.unwrap_or(Duration::from_secs(10));
    let ping_future = surge_ping::ping(ip.to_string().parse()?, &[0; 32]);
    match tokio::time::timeout(timeout, ping_future).await {
        Ok(result) => {
            result?;
            Ok(true)
        }
        Err(_) => Ok(false),
    }
}

#[tokio::test]
async fn test_ping_ip() {
    let ip = get_ip().await.unwrap();
    let nat = ping_ip(ip, None).await.unwrap();
    assert_eq!(nat, true);
}

#[tokio::test]
async fn test_ping_ip_loopback() {
    let nat = ping_ip(Ipv4Addr::new(127, 0, 0, 1), None).await.unwrap();
    assert_eq!(nat, true);
}

#[tokio::test]
async fn test_ping_ip_not_nat() {
    let nat = ping_ip(
        Ipv4Addr::new(198, 17, 255, 255),
        Some(Duration::from_secs(1)),
    )
    .await
    .unwrap();
    assert_eq!(nat, false);
}
