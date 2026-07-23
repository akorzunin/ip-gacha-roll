use std::{collections::HashMap, env, sync::OnceLock, time::Duration};

use ip_gacha_roll_shared::net_utils;
use teloxide::prelude::*;
use tokio::{task::JoinHandle, time::sleep};

const DEFAULT_NAT_CHECK_INTERVAL_SECS: u64 = 300;
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
    value
        .and_then(|value| value.parse().ok())
        .filter(|&seconds| seconds > 0)
        .unwrap_or(DEFAULT_NAT_CHECK_INTERVAL_SECS)
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
    loop {
        let (current, state) = nat_status().await;
        if previous != Some(state) {
            if let Err(error) = bot.send_message(chat_id, current).await {
                log::warn!("Could not send NAT status: {error}");
            }
            previous = Some(state);
        }
        sleep(interval).await;
    }
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
