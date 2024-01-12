use std::cell::RefCell;
use std::error::Error;

use env_file_reader::read_file;
use rand::prelude::*;
use teloxide::{prelude::*, types::Me};
use teloxide::utils::command::BotCommands;

use crate::commands::{answer};

mod commands;

struct GlobalData {
    racista_ratio: u32,
}

thread_local!(static GLOBAL_DATA: RefCell<GlobalData> = RefCell::new(GlobalData { racista_ratio: 10 }));

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let env_variables = read_file(".env")?;
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::new(&env_variables["TELOXIDE_TOKEN"]);

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler));

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
    Ok(())
}

async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(cmd) => {
                answer(bot, msg, cmd).await?;
            }

            Err(_) => {
                let number: u32 = thread_rng().gen_range(0..100);
                let mut send_racista = false;
                let _ = GLOBAL_DATA.with(|dict| {
                    if number <= dict.borrow().racista_ratio {
                        send_racista = true;
                    }
                });

                if send_racista {
                    bot.send_message(msg.chat.id, "racista").reply_to_message_id(msg.id).await?;
                    send_racista = false;
                }
            }
        }
    }

    Ok(())
}