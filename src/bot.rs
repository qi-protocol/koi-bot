use crate::keyboards::menu_keyboard;
use crate::tg_error;
use crate::utils;
use teloxide::{
    dispatching::UpdateFilterExt,
    dptree,
    error_handlers::LoggingErrorHandler,
    payloads::{EditMessageTextSetters, SendMessageSetters},
    prelude::{Dispatcher, Requester},
    types::{
        CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Me, Message, ParseMode, Update,
    },
    utils::command::BotCommands,
    Bot,
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "Show this text")]
    Help,
    #[command(description = "Main Menu")]
    Menu,
    #[command(description = "Display all wallet addresses")]
    Wallets,
}

#[derive(Clone, Debug)]
pub(crate) struct TgBot {
    bot: Bot,
}

impl TgBot {
    pub(crate) fn new() -> Self {
        let bot = Bot::from_env();
        Self { bot }
    }

    pub(crate) async fn init(self) -> Result<(), tg_error::TgError> {
        let handler = dptree::entry()
            .branch(Update::filter_message().endpoint(message_handler))
            .branch(Update::filter_callback_query().endpoint(callback_handler));

        Dispatcher::builder(self.bot, handler)
            .error_handler(LoggingErrorHandler::with_custom_text(
                "An error has occurred in the dispatcher",
            ))
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
        Ok(())
    }
}

fn make_keyboard(context: &str) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let actions: Vec<&str> = match context {
        "main" => vec!["Buy", "Sell", "Limit Buy", "Limit Sell"],
        "Buy" => vec!["BTC", "ETH", "LTC", "BCH", "Main Menu", "Close"],
        "Sell" => vec!["BTC", "ETH", "LTC", "BCH", "Main Menu", "Close"],
        "Limit Buy" => vec!["BTC", "ETH", "LTC", "BCH", "Main Menu", "Close"],
        "Limit Sell" => vec!["BTC", "ETH", "LTC", "BCH", "Main Menu", "Close"],
        _ => vec![],
    };

    for action in actions.chunks(3) {
        let row = action
            .iter()
            .map(|&action| InlineKeyboardButton::callback(action.to_owned(), action.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

#[allow(clippy::redundant_closure)]
async fn message_handler(bot: Bot, msg: Message, me: Me) -> Result<(), tg_error::TgError> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                let _ = bot
                    .send_message(msg.chat.id, Command::descriptions().to_string())
                    .await
                    .map_err(|e| tg_error::TgError::TeloxideRequest(e));
            }
            Ok(Command::Menu) => {
                let keyboard = menu_keyboard();
                let menu_msg = utils::get_on_chain_info().await?;
                let _ = bot
                    .send_message(msg.chat.id, menu_msg)
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(keyboard)
                    .await
                    .map_err(|e| tg_error::TgError::TeloxideRequest(e));
            }
            Ok(Command::Wallets) => {
                todo!()
            }
            Err(_) => {
                let _ = bot
                    .send_message(
                        msg.chat.id,
                        "Command not found. Press /help to see all supported commands",
                    )
                    .await
                    .map_err(|e| tg_error::TgError::TeloxideRequest(e));
            }
        }
    }

    Ok(())
}

#[allow(clippy::redundant_closure)]
async fn handle_menu_callback(bot: &Bot, q: &CallbackQuery) -> Result<(), tg_error::TgError> {
    let keyboard = menu_keyboard();
    bot.answer_callback_query(&q.id).await?;
    if let Some(Message { id, chat, .. }) = &q.message {
        let menu_msg = utils::get_on_chain_info().await?;
        let _ = bot
            .edit_message_text(chat.id, *id, menu_msg)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(keyboard)
            .await
            .map_err(|e| tg_error::TgError::TeloxideRequest(e));
    };
    Ok(())
}

#[allow(clippy::redundant_closure)]
async fn handle_close_callback(bot: &Bot, q: &CallbackQuery) -> Result<(), tg_error::TgError> {
    bot.answer_callback_query(&q.id).await?;
    if let Some(Message { id, chat, .. }) = &q.message {
        let _ = bot
            .delete_message(chat.id, *id)
            .await
            .map_err(|e| tg_error::TgError::TeloxideRequest(e));
    };
    Ok(())
}

#[allow(clippy::redundant_closure)]
async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), tg_error::TgError> {
    if let Some(action) = &q.data {
        match action.as_str() {
            "Buy" | "Sell" | "Limit Buy" | "Limit Sell" => {
                let keyboard = make_keyboard(action);
                let text = format!("Choose an option for {}:", action);
                bot.answer_callback_query(q.id).await?;
                if let Some(Message { chat, .. }) = q.message {
                    let _ = bot
                        .send_message(chat.id, text)
                        .reply_markup(keyboard)
                        .await
                        .map_err(|e| tg_error::TgError::TeloxideRequest(e));
                }
            }
            "Main Menu" => handle_menu_callback(&bot, &q).await?,
            "Close" => handle_close_callback(&bot, &q).await?,
            _ => {
                let text = format!("You chose: {}", action);
                bot.send_message(q.from.id, text).await?;
            }
        }

        log::info!("You chose: {}", action);
    }

    Ok(())
}
