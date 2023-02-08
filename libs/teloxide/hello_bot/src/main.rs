use dotenv::dotenv;
use std::env;

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    // let bot = Bot::from_env(); // taking the token by default set into `~/.zprofile`
    dotenv().ok();
    let bot = Bot::new(&env::var("TELEGRAM_BOT_KEY").unwrap());

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        log::info!("the no. is: {}", msg.chat.username().unwrap());
        Ok(())
    })
    .await;
}
