use std::{net::Ipv4Addr, time::Duration};

use reqwest::{Client, header::USER_AGENT};
use tokio::net::TcpStream;

#[allow(dead_code)]
pub async fn ping_ip(ip: Ipv4Addr, timeout: Option<Duration>) -> Result<bool, anyhow::Error> {
    let timeout = timeout.unwrap_or(Duration::from_secs(10));
    let ping_future = surge_ping::ping(ip.to_string().parse()?, &[0; 32]);
    match tokio::time::timeout(timeout, ping_future).await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
pub async fn ping_ip_tcp(ip: Ipv4Addr, timeout: Option<Duration>) -> Result<bool, anyhow::Error> {
    let timeout = timeout.unwrap_or(Duration::from_secs(10));
    let socket_addr = (ip, 80);
    match tokio::time::timeout(timeout, TcpStream::connect(&socket_addr)).await {
        Ok(Ok(_)) => Ok(true), // Connection successful
        Ok(Err(e)) => {
            eprintln!("Connection error: {}", e);
            Ok(false)
        }
        Err(_) => {
            eprintln!("Connection timed out");
            Ok(false)
        }
    }
}
pub async fn get_ip() -> Result<Ipv4Addr, anyhow::Error> {
    let client = Client::new();
    let res = client
        .get("https://ifconfig.me")
        .header(USER_AGENT, "curl")
        .send()
        .await?;
    let ip_res = &res.text().await?;
    let ip = ip_res.parse::<Ipv4Addr>()?;
    Ok(ip)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_ip() {
        let ip = get_ip().await.unwrap();
        println!("ip: {}", ip.to_string());
        assert_eq!(ip.is_loopback(), false);
    }

    #[tokio::test]
    async fn test_ping_ip() {
        let ip = get_ip().await.unwrap();
        let nat = ping_ip(ip, None).await.unwrap();
        assert_eq!(nat, true);
    }

    #[tokio::test]
    async fn test_ping_ip_tcp() {
        let ip = get_ip().await.unwrap();
        let nat = ping_ip_tcp(ip, None).await.unwrap();
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
}
