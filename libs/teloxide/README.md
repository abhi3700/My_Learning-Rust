# Teloxide

## About

- Telegram bot framework for Rust.

## Getting Started

1. `$ cargo new hello_bot`
2. copy & paste this into `[dependencies]` in `Cargo.toml` file:

   ```toml
   teloxide = { version = "0.12", features = ["macros"] }
   log = "0.4"
   pretty_env_logger = "0.4"
   tokio = { version =  "1.8", features = ["rt-multi-thread", "macros"] }
   ```

3. `$ cargo add dotenv`
4. `$ cp .env.example .env` - create a `.env` file.
5. add the bot token in `.env` from BotFather in telegram w/o string quotes like this:

   ```env
   TELEGRAM_BOT_KEY=xxxxxxxxx:xxxxxxxxxxxxxxxxxxxxxxxx
   ```

6. copy this code & paste into `src/main.rs` file:

   ```rs
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
   ```

7. Run via `$ cargo watch -x run` or `$ cargo run`
8. Press <kbd>ctrl+c</kbd> or logout/shutdown/close the active terminal.

## Troubleshoot

### 1. ERROR teloxide::error_handlers > An error from the update listener: Api(TerminatedByOtherGetUpdates)

- _Cause_: This happens when 2 different clients send getUpdates method to Telegram servers for one bot token.
- _Solution_: Kill the terminal & restart the bot on another terminal.

## References

- [Github repo](https://github.com/teloxide/teloxide)
