use teloxide::{prelude::*, utils::command::BotCommands, types::ParseMode};
use rand::prelude::*;
use crate::{GLOBAL_DATA, GlobalData};

static VERSION: &str = "2\\.1";
static GUS_API_VER: &str = "1\\.2";
static FELA_VER: &str = "5\\.8";
static FELA_INSULTS: [&str; 20] = ["you incompetent fuck", "lmao you gai", "fuck off", "bitch", "dumbass TSF fuck",
"yo mama so fat she can eat all unigram bugs and still be hungry", "candy ass madafakas",
"shut your face you non\\-dev scumbag", "fuck you, lucas and every user ever", "lol you so stoooopid",
"joe mama sooooo gai", "dima has more hair than joe mama", "hoe",
"if you were italian your name would be Retardo", "suck my Qt dick",
"here\\'s a box of boosts you poor\\-ass bitch", "TSF stands for True Suckers For devs",
"you're on the latest beta\\. I'm on your gal", "you look like gal gadot's armpit",
"Netherrealm could make a game of you\\. Age of the Faggots"];

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "send racista.")]
    Racista,
    #[command(description = "set racista ratio.")]
    SetRacista(u32),
    #[command(description = "version info.")]
    Version,
    #[command(description = "fela.")]
    Fela,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "alias for /setracista.")]
    S(u32),
    #[command(description = "alias for /racista.")]
    R,
    #[command(description = "alias for /version.")]
    V,
    #[command(description = "alias for /fela.")]
    F,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Racista | Command::R => {
            match msg.reply_to_message() {
                Some(reply_message) => bot.send_message(msg.chat.id, "racista")
                    .reply_to_message_id(reply_message.id).await?,
                None => bot.send_message(msg.chat.id, "racista").
                    reply_to_message_id(msg.id).await?
            }
        },
        Command::SetRacista(ratio) | Command::S(ratio) => {
            GLOBAL_DATA.with(|dict| {
                *dict.borrow_mut() = GlobalData { racista_ratio: ratio }
            });

            bot.send_message(msg.chat.id, format!("racista ratio set to {ratio}%"))
                .reply_to_message_id(msg.id).await?
        },
        Command::Version | Command::V => {
            let version_text = format!("**Gustavotron \\- Free**\n\n\
            _Version_: {VERSION}\n_Gustavo API Level_: {GUS_API_VER}\n\
            _Fela API Level_: {FELA_VER}\n\nLicensed to Gustavo Fring \\(@NowPremiumUser\\)\n\
            _For legal information and privacy policy contact @OnetimeUsername_");
            let version_text_premium = format!("**Gustavotron \\- Premium**\n\n\
            _Version_: {VERSION}\n_Gustavo API Level_: {GUS_API_VER}\n\
            _Fela API Level_: {FELA_VER}\n\nLicensed to Gustavo Fring \\(@NowPremiumUser\\)\n\
            _For legal information and privacy policy contact @OnetimeUsername_");

            if msg.from().unwrap().is_premium {
                bot.send_message(msg.chat.id, version_text_premium)
                    .reply_to_message_id(msg.id).parse_mode(ParseMode::MarkdownV2).await?
            } else {
                bot.send_message(msg.chat.id, version_text)
                    .reply_to_message_id(msg.id).parse_mode(ParseMode::MarkdownV2).await?
            }
        }
        Command::Fela | Command::F => {
            let number: usize = rand::thread_rng().gen_range(0..FELA_INSULTS.len());
            bot.send_message(msg.chat.id, FELA_INSULTS[number])
                .reply_to_message_id(msg.id).parse_mode(ParseMode::MarkdownV2).await?
        },
        Command::Username(username) => {
            if username == "" {
                bot.send_message(msg.chat.id, "No Username Found\nTry /username [username_here]")
                    .reply_to_message_id(msg.id).await?
            } else {
                bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                    .reply_to_message_id(msg.id).await?
            }
        }
    };

    Ok(())
}