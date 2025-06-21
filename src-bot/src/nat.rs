use teloxide::prelude::*;

use ip_gacha_roll_lib::shared::net_utils;

pub async fn nat_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    let m = bot.send_message(msg.chat.id, "Checking NAT...").await?;
    let ip = match net_utils::get_ip().await {
        Ok(ip) => ip,
        Err(e) => {
            bot.edit_message_text(m.chat.id, m.id, format!("Error: {}", e))
                .await?;
            return Ok(());
        }
    };
    let nat = match net_utils::ping_ip_tcp(ip, None).await {
        Ok(nat) => nat,
        Err(e) => {
            bot.edit_message_text(m.chat.id, m.id, format!("Error: {}", e))
                .await?;
            return Ok(());
        }
    };
    bot.edit_message_text(
        m.chat.id,
        m.id,
        format!("IP: {} nat: {}", ip, if nat { "✅" } else { "❌" }),
    )
    .await?;
    Ok(())
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
