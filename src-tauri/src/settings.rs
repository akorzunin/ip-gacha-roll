use serde::{Deserialize, Serialize};

// ROUTER user
pub(crate) const ROUTER_USER: &str = "admin";
// ROUTER pass
pub(crate) const ROUTER_PASS: &str = "admin";
// ROUTER ip
pub(crate) const ROUTER_IP: &str = "192.168.1.1";

pub(crate) const DRY_RUN: bool = false;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub router_ip: String,
    pub user: String,
    pub pass: String,
    pub dry_run: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            router_ip: ROUTER_IP.to_string(),
            user: ROUTER_USER.to_string(),
            pass: ROUTER_PASS.to_string(),
            dry_run: DRY_RUN,
        }
    }
}

pub fn get_settings(settings: Option<Settings>) -> Settings {
    match settings {
        Some(settings) => settings,
        None => Settings::new(),
    }
}
