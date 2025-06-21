use teloxide::prelude::*;

pub async fn nat_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Checking NAT...").await?;

    Ok(())
}
