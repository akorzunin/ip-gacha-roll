use std::{collections::HashMap, env, sync::OnceLock, time::Duration};

use crate::roll::{RollResult, reroll};
use ip_gacha_roll_shared::net_utils;
use teloxide::prelude::*;
use tokio::{task::JoinHandle, time::sleep};

const DEFAULT_NAT_CHECK_INTERVAL_SECS: u64 = 300;
const DEFAULT_NAT_FIX_MAX_ATTEMPTS: u64 = 3;
const DEFAULT_NAT_FIX_WAIT_SECS: u64 = 10;
const DUCKDNS_UI_URL: &str = "http://192.168.1.58:3000";
const DUCKDNS_DOMAIN: &str = "akorz.duckdns.org";
static NAT_MONITORS: OnceLock<tokio::sync::Mutex<HashMap<ChatId, JoinHandle<()>>>> =
    OnceLock::new();

pub async fn nat_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    let m = bot.send_message(msg.chat.id, "Checking NAT...").await?;
    let (message, _) = nat_status().await;
    bot.edit_message_text(m.chat.id, m.id, message).await?;
    Ok(())
}

pub fn default_interval_secs() -> u64 {
    interval_secs(env::var("NAT_CHECK_INTERVAL_SECS").ok().as_deref())
}

fn interval_secs(value: Option<&str>) -> u64 {
    positive_secs(value, DEFAULT_NAT_CHECK_INTERVAL_SECS)
}

fn positive_secs(value: Option<&str>, default: u64) -> u64 {
    value
        .and_then(|value| value.parse().ok())
        .filter(|&seconds| seconds > 0)
        .unwrap_or(default)
}

pub async fn start_monitor(bot: Bot, chat_id: ChatId, interval_secs: u64) {
    let monitor = tokio::spawn(monitor_nat(
        bot,
        chat_id,
        Duration::from_secs(interval_secs),
    ));
    let monitors = NAT_MONITORS.get_or_init(Default::default);
    if let Some(previous) = monitors.lock().await.insert(chat_id, monitor) {
        previous.abort();
    }
}

pub async fn stop_monitor(chat_id: ChatId) -> bool {
    let monitors = NAT_MONITORS.get_or_init(Default::default);
    if let Some(monitor) = monitors.lock().await.remove(&chat_id) {
        monitor.abort();
        true
    } else {
        false
    }
}

async fn monitor_nat(bot: Bot, chat_id: ChatId, interval: Duration) {
    let mut previous = None;
    let mut recovery_exhausted = false;
    loop {
        let (mut current, mut state) = nat_status().await;
        if state == Some(false) && !recovery_exhausted {
            (current, state) = recover_nat().await;
            recovery_exhausted = state == Some(false);
        } else if state != Some(false) {
            recovery_exhausted = false;
        }
        if previous != Some(state) {
            if let Err(error) = bot.send_message(chat_id, current).await {
                log::warn!("Could not send NAT status: {error}");
            }
            previous = Some(state);
        }
        sleep(interval).await;
    }
}

async fn recover_nat() -> (String, Option<bool>) {
    let attempts = positive_secs(
        env::var("NAT_FIX_MAX_ATTEMPTS").ok().as_deref(),
        DEFAULT_NAT_FIX_MAX_ATTEMPTS,
    );
    let wait = Duration::from_secs(positive_secs(
        env::var("NAT_FIX_WAIT_SECS").ok().as_deref(),
        DEFAULT_NAT_FIX_WAIT_SECS,
    ));

    for attempt in 1..=attempts {
        match reroll().await {
            Ok(RollResult::DryRun) => {
                return ("NAT recovery skipped: DRY_RUN is set.".into(), Some(false));
            }
            Ok(_) => {}
            Err(error) => {
                log::warn!("NAT recovery roll attempt {attempt}/{attempts} failed: {error}");
                sleep(wait).await;
                continue;
            }
        }

        let (message, state) = nat_status().await;
        if state == Some(true) {
            return match update_duckdns().await {
                Ok(()) => (
                    format!("NAT recovered and DuckDNS updated {attempt}/{attempts}: {message}"),
                    state,
                ),
                Err(error) => (
                    format!("NAT recovered, but DuckDNS update failed: {error}; {message}"),
                    state,
                ),
            };
        }
        if state.is_none() {
            return (
                format!("NAT check failed after roll attempt {attempt}/{attempts}: {message}"),
                state,
            );
        }
        sleep(wait).await;
    }
    (
        format!("NAT remains unreachable after {attempts} roll attempts."),
        Some(false),
    )
}

async fn update_duckdns() -> anyhow::Result<()> {
    let url = env::var("DUCKDNS_UI_URL")
        .ok()
        .filter(|url| !url.is_empty())
        .unwrap_or_else(|| DUCKDNS_UI_URL.into());
    let response = tokio::time::timeout(
        Duration::from_secs(10),
        reqwest::Client::new()
            .post(format!("{}/api/task", url.trim_end_matches('/')))
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"domain":"{DUCKDNS_DOMAIN}","interval":"0"}}"#))
            .send(),
    )
    .await??;
    response.error_for_status()?;
    Ok(())
}

async fn nat_status() -> (String, Option<bool>) {
    let ip = match net_utils::get_ip().await {
        Ok(ip) => ip,
        Err(error) => return (format!("Error: {error}"), None),
    };
    match net_utils::ping_ip_tcp(ip, None).await {
        Ok(nat) => (
            format!("IP: {ip} nat: {}", if nat { "✅" } else { "❌" }),
            Some(nat),
        ),
        Err(error) => (format!("Error: {error}"), None),
    }
}

#[test]
fn interval_defaults_when_missing_or_invalid() {
    assert_eq!(interval_secs(None), DEFAULT_NAT_CHECK_INTERVAL_SECS);
    assert_eq!(
        interval_secs(Some("invalid")),
        DEFAULT_NAT_CHECK_INTERVAL_SECS
    );
    assert_eq!(interval_secs(Some("0")), DEFAULT_NAT_CHECK_INTERVAL_SECS);
    assert_eq!(interval_secs(Some("60")), 60);
    assert_eq!(positive_secs(Some("2"), 3), 2);
    assert_eq!(positive_secs(Some("0"), 3), 3);
}

#[tokio::test]
async fn test_hello_world() {
    use teloxide_tests::{MockBot, MockMessageText};
    let message = MockMessageText::new().text("Hi!");
    let mut bot = MockBot::new(
        message,
        dptree::entry().branch(Update::filter_message().endpoint(nat_command)),
    );
    // Sends the message as if it was from a user
    bot.dispatch().await;

    let responses = bot.get_responses();
    let message = responses
        .sent_messages
        .last()
        .expect("No sent messages were detected!");
    assert_eq!(message.text(), Some("Checking NAT..."));
}
