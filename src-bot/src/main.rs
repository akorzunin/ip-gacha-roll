use teloxide::{RequestError, prelude::*, utils::command::BotCommands};
use warp::Filter;

use crate::{nat::nat_command, roll::roll_command};

extern crate log;
extern crate pretty_env_logger;
mod nat;
mod roll;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    match std::env::var("ROUTER_PASS") {
        Ok(_) => {}
        Err(_) => {
            log::warn!("Router password is not set, use ROUTER_PASS environment variable");
        }
    }

    let bot = Bot::from_env();

    let bot_task = Command::repl(bot, answer);
    let health_task = health_check_server();

    tokio::select! {
        _ = bot_task => {},
        _ = health_task => {},
    }
    log::info!("Shutting down...");
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Start,
    #[command(description = "display this text.")]
    Help,
    #[command(description = "roll new ip address.")]
    Roll,
    #[command(description = "check nat status.")]
    Nat,
}

async fn bot_err(bot: Bot, msg: Message, e: RequestError) -> ResponseResult<()> {
    log::error!("Request error: {:?}", e);
    bot.send_message(msg.chat.id, e.to_string()).await?;
    Ok(())
}

async fn auth_user(bot: Bot, msg: Message) -> Result<(), ()> {
    let authorized_users = match std::env::var("AUTHORIZED_BOT_USERS") {
        Ok(users) => users,
        Err(_) => {
            bot.send_message(
                msg.chat.id,
                "AUTHORIZED_BOT_USERS environment variable is not set.",
            )
            .await
            .unwrap();
            return Err(());
        }
    };
    dbg!(&authorized_users);
    let authorized_users_list = authorized_users.split(',').collect::<Vec<&str>>();
    dbg!(&authorized_users_list);
    match msg.from {
        Some(from) => {
            log::info!("Command from user_id: {:?}", from.id);
            if !authorized_users_list.contains(&from.id.to_string().as_str()) {
                bot.send_message(msg.chat.id, "You are not in authorized list to use this bot. Check AUTHORIZED_BOT_USERS environment variable.")
                    .await.unwrap();
                return Err(());
            }
            Ok(())
        }
        None => Err(()),
    }
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let _: Result<(), ()> = match auth_user(bot.clone(), msg.clone()).await {
        Ok(_) => Ok(()),
        Err(_) => return Ok(()),
    };
    let _ = match cmd {
        Command::Start => {
            match bot
                .send_message(msg.chat.id, "Welcome to ip-gacha-roll bot.")
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => bot_err(bot, msg, e).await,
            }
        }
        Command::Help => {
            match bot
                .send_message(msg.chat.id, Command::descriptions().to_string())
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => bot_err(bot, msg, e).await,
            }
        }
        Command::Roll => match roll_command(bot.clone(), msg.clone()).await {
            Ok(_) => Ok(()),
            Err(e) => bot_err(bot, msg, e).await,
        },
        Command::Nat => match nat_command(bot.clone(), msg.clone()).await {
            Ok(_) => Ok(()),
            Err(e) => bot_err(bot, msg, e).await,
        },
    };

    Ok(())
}

async fn health_check_server() {
    let port: u16 = std::env::var("HEALTH_CHECK_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);
    let health_route =
        warp::path!("health").map(|| warp::reply::with_status("OK", warp::http::StatusCode::OK));

    log::info!("Starting health check server on port {}", port);
    warp::serve(health_route).run(([0, 0, 0, 0], port)).await;
}
