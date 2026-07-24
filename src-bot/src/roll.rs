use std::{env, net::Ipv4Addr, time::Duration};

use ip_gacha_roll_shared::{
    keen_utils::{KeenClient, reroll_interface_status},
    net_utils,
};
use teloxide::{ApiError, RequestError, prelude::*};
use tokio::{
    task::spawn_blocking,
    time::{interval, sleep, timeout},
};

const ROLL_READY_TIMEOUT: Duration = Duration::from_secs(100);
const ROLL_CHECK_INTERVAL: Duration = Duration::from_secs(2);
const TELEGRAM_RETRY_DELAY: Duration = Duration::from_secs(1);

fn do_roll() -> Result<bool, String> {
    let c = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .user_agent("curl")
        .build()
        .expect("Failed to create HTTP client for Router");
    let router_ip = option_env!("ROUTER_IP").unwrap_or("192.168.1.1");
    let router_user = option_env!("ROUTER_USER").unwrap_or("admin");
    let router_pass = env::var("ROUTER_PASS")
        .map_err(|_| "ROUTER_PASS environment variable not set".to_string())?;
    let dry_run = env::var("DRY_RUN").is_ok_and(|value| value == "true");
    if dry_run {
        log::warn!("DRY_RUN is set, no changes will be made");
    }

    let kc = KeenClient::new(router_ip.to_string()).expect("Failed to create HTTP client");
    if !kc
        .auth(router_user, &router_pass, &c)
        .map_err(|error| error.to_string())?
    {
        return Err("Router authentication failed".into());
    }
    reroll_interface_status(&kc, &c, dry_run).map_err(|error| error.to_string())?;
    Ok(dry_run)
}

async fn edit_status(bot: &Bot, message: &Message, text: impl Into<String>) -> ResponseResult<()> {
    let text = text.into();
    loop {
        match bot
            .edit_message_text(message.chat.id, message.id, text.clone())
            .await
        {
            Ok(_) | Err(RequestError::Api(ApiError::MessageNotModified)) => return Ok(()),
            Err(error) => match telegram_retry_delay(&error) {
                Some(delay) => {
                    log::warn!(
                        "Telegram request failed: {error}; retrying in {}s",
                        delay.as_secs()
                    );
                    sleep(delay).await;
                }
                None => return Err(error),
            },
        }
    }
}

fn telegram_retry_delay(error: &RequestError) -> Option<Duration> {
    match error {
        RequestError::Network(_) => Some(TELEGRAM_RETRY_DELAY),
        RequestError::RetryAfter(seconds) => Some(seconds.duration()),
        _ => None,
    }
}

pub async fn roll_command(bot: Bot, msg: Message) -> anyhow::Result<()> {
    let message = bot
        .send_message(msg.chat.id, "🔄 Restarting PPPoE connection…")
        .await?;
    let previous_ip = timeout(Duration::from_secs(5), net_utils::get_ip())
        .await
        .ok()
        .and_then(Result::ok);

    let dry_run = match spawn_blocking(do_roll).await {
        Ok(Ok(dry_run)) => dry_run,
        Ok(Err(error)) => {
            edit_status(&bot, &message, format!("❌ {error}")).await?;
            return Ok(());
        }
        Err(error) => {
            edit_status(&bot, &message, format!("❌ {error}")).await?;
            return Ok(());
        }
    };
    if dry_run {
        edit_status(&bot, &message, "✅ Dry run: router was not restarted.").await?;
        return Ok(());
    }

    edit_status(&bot, &message, "⏳ Waiting for a new reachable IP…").await?;
    match timeout(ROLL_READY_TIMEOUT, wait_for_reachable_ip(previous_ip)).await {
        Ok(ip) => edit_status(&bot, &message, format!("✅ New reachable IP: {ip}")).await?,
        Err(_) => {
            edit_status(
                &bot,
                &message,
                "⚠️ No new reachable IP after 100 seconds. Check /nat or roll again.",
            )
            .await?
        }
    }
    Ok(())
}

async fn wait_for_reachable_ip(previous_ip: Option<Ipv4Addr>) -> Ipv4Addr {
    let mut checks = interval(ROLL_CHECK_INTERVAL);
    loop {
        checks.tick().await;
        let Ok(ip) = net_utils::get_ip().await else {
            continue;
        };
        if !has_new_ip(previous_ip, ip) {
            continue;
        }
        if matches!(
            net_utils::ping_ip_tcp(ip, Some(ROLL_CHECK_INTERVAL)).await,
            Ok(true)
        ) {
            return ip;
        }
    }
}

fn has_new_ip(previous_ip: Option<Ipv4Addr>, ip: Ipv4Addr) -> bool {
    previous_ip != Some(ip)
}

#[test]
fn waits_for_a_changed_ip() {
    let old = Ipv4Addr::new(192, 0, 2, 1);
    assert!(!has_new_ip(Some(old), old));
    assert!(has_new_ip(Some(old), Ipv4Addr::new(192, 0, 2, 2)));
    assert!(has_new_ip(None, old));
}

#[test]
fn telegram_retry_respects_rate_limits() {
    assert_eq!(
        telegram_retry_delay(&RequestError::RetryAfter(
            teloxide::types::Seconds::from_seconds(3)
        )),
        Some(Duration::from_secs(3))
    );
}

#[tokio::test]
async fn test_hello_world() {
    use teloxide_tests::{MockBot, MockMessageText};
    let message = MockMessageText::new().text("Hi!");
    let mut bot = MockBot::new(
        message,
        dptree::entry().branch(Update::filter_message().endpoint(roll_command)),
    );
    bot.dispatch().await;

    let responses = bot.get_responses();
    let message = responses
        .sent_messages
        .last()
        .expect("No sent messages were detected!");
    assert_eq!(message.text(), Some("🔄 Restarting PPPoE connection…"));
}
