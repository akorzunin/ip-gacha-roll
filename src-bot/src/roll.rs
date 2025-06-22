use std::env;

use ip_gacha_roll_shared::keen_utils::{KeenClient, reroll_interface};
use log::info;
use teloxide::prelude::*;
use tokio::{task::spawn_blocking, time::Duration, time::sleep};

fn do_roll() -> String {
    let c = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .user_agent("curl")
        .build()
        .expect("Failed to create HTTP client for Router");
    let router_ip = option_env!("ROUTER_IP").unwrap_or("192.168.1.1");
    let router_user = option_env!("ROUTER_USER").unwrap_or("admin");
    let router_pass = env::var("ROUTER_PASS").expect("ROUTER_PASS environment variable not set");
    let dry_run = env::var("DRY_RUN").expect("DRY_RUN environment variable not set") == "true";

    let kc = KeenClient::new(router_ip.to_string()).expect("Failed to create HTTP client");
    match kc.auth(router_user, &router_pass, &c) {
        Ok(authenticated) => {
            if !authenticated {
                return "Authentication failed".to_string();
            }
            reroll_interface(&kc, &c, dry_run)
        }
        Err(e) => e.to_string(),
    }
}

pub async fn roll_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    let m = bot.send_message(msg.chat.id, "Rolling new IP...").await?;
    let res = spawn_blocking(do_roll).await.unwrap();
    sleep(Duration::from_secs(100)).await;
    info!("res: {:?}", res);
    if res.is_empty() {
        let _ = bot
            .edit_message_text(m.chat.id, m.id, "IP rolled successfully")
            .await;
    } else {
        let _ = bot.send_message(msg.chat.id, &res).await;
    }

    Ok(())
}

#[tokio::test]
async fn test_hello_world() {
    use teloxide_tests::{MockBot, MockMessageText};
    let message = MockMessageText::new().text("Hi!");
    let mut bot = MockBot::new(
        message,
        dptree::entry().branch(Update::filter_message().endpoint(roll_command)),
    );
    // Sends the message as if it was from a user
    bot.dispatch().await;

    let responses = bot.get_responses();
    let message = responses
        .sent_messages
        .last()
        .expect("No sent messages were detected!");
    assert_eq!(message.text(), Some("Rolling new IP..."));
}
