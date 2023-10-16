use crate::keyboards::{buy_keyboard, menu_keyboard};
use crate::requests::SendBuyTxRequest;
use crate::tg_error;
use crate::utils;
use teloxide::{
    dispatching::UpdateFilterExt,
    dptree,
    error_handlers::LoggingErrorHandler,
    payloads::{EditMessageTextSetters, SendMessageSetters},
    prelude::{Dispatcher, Requester},
    types::{
        CallbackQuery, ChatId, InlineKeyboardButton, InlineKeyboardMarkup, Me, Message, MessageId,
        ParseMode, Update,
    },
    utils::command::BotCommands,
    Bot,
};
use tokio::time::{sleep, Duration};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "Show Available Commands")]
    Help,
    #[command(description = "Main Menu")]
    Menu,
    #[command(description = "Display all wallet addresses")]
    Wallets,
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Display Trade History")]
    History,
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
            .branch(Update::filter_message().endpoint(message_callback))
            .branch(Update::filter_callback_query().endpoint(button_callback));

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
async fn message_callback(bot: Bot, msg: Message, me: Me) -> Result<(), tg_error::TgError> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                let _ = bot
                    .send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }
            Ok(Command::Menu) => {
                let keyboard = menu_keyboard();
                let menu_msg = utils::get_on_chain_info().await?;

                // send the new message
                let message_sent = bot
                    .send_message(msg.chat.id, menu_msg)
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(keyboard)
                    .await?;

                // delete previous messages
                let last_message_id = message_sent.id;
                let _ =
                    delete_previous_messages(&bot, msg.chat.id.0, last_message_id.0 - 1).await?;
            }
            Ok(Command::Start) => {
                // todp: add start message
                todo!()
            }
            Ok(Command::Wallets) => {
                todo!()
            }
            Ok(Command::History) => {
                todo!()
            }
            Err(_) => {
                let _ = bot
                    .send_message(
                        msg.chat.id,
                        "Command not found. Press /help to see all supported commands",
                    )
                    .await?;
            }
        }
    }

    Ok(())
}

/// Helper function to delete 10 previous messages
async fn delete_previous_messages(
    bot: &Bot,
    chat_id: i64,
    last_message_id: i32,
) -> Result<(), tg_error::TgError> {
    log::info!("last message id: {}", last_message_id);
    for message_id in (last_message_id - 10..=last_message_id).rev() {
        log::info!("last message id: {}", message_id);
        sleep(Duration::from_millis(10)).await;
        let _ = bot
            .delete_message(ChatId(chat_id), MessageId(message_id))
            .await;
    }
    Ok(())
}

/// Upon a user clicks the "Main Menu", it'll clear the text and show the menu again
async fn handle_menu_callback(bot: &Bot, q: &CallbackQuery) -> Result<(), tg_error::TgError> {
    let keyboard = menu_keyboard();
    bot.answer_callback_query(&q.id).await?;
    if let Some(Message { chat, .. }) = &q.message {
        let menu_msg = utils::get_on_chain_info().await?;

        let message_sent = bot
            .send_message(chat.id, menu_msg)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(keyboard)
            .await?;
        let last_message_id = message_sent.id;
        let _ = delete_previous_messages(bot, chat.id.0, last_message_id.0 - 1).await?;
    };
    Ok(())
}

async fn handle_buy_callback(bot: &Bot, q: &CallbackQuery) -> Result<(), tg_error::TgError> {
    let keyboard = buy_keyboard(true, false, true, false, false)?;
    bot.answer_callback_query(&q.id).await?;
    if let Some(Message { id: _id, chat, .. }) = &q.message {
        let menu_msg = utils::get_on_chain_info().await?;
        // todo: add custom info for buy
        let _ = bot
            .send_message(chat.id, menu_msg)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(keyboard)
            .await?;
    }
    Ok(())
}

async fn handle_close_callback(bot: &Bot, q: &CallbackQuery) -> Result<(), tg_error::TgError> {
    bot.answer_callback_query(&q.id).await?;
    if let Some(Message { id, chat, .. }) = &q.message {
        let _ = bot.delete_message(chat.id, *id).await?;
    };
    Ok(())
}

#[derive(Debug)]
enum SubMenuType {
    SendBuyTx,
    SendSellTx,
}

/// Gets the last vec in the larger vec in the InlineKeyboardMarkup. See https://docs.rs/teloxide/latest/teloxide/types/struct.InlineKeyboardMarkup.html
/// Gets the last button in the last vec, which should either be "Send Buy Tx" or "Send Sell Tx"
fn find_sub_menu_type_from_callback(q: &CallbackQuery) -> anyhow::Result<SubMenuType> {
    q.message
        .as_ref()
        .and_then(|msg| msg.reply_markup())
        .and_then(|keyboard| keyboard.inline_keyboard.last())
        .and_then(|last_vec| last_vec.last())
        .and_then(|last_button| match last_button.text.as_str() {
            "Send Buy Tx" => Some(SubMenuType::SendBuyTx),
            "Send Sell Tx" => Some(SubMenuType::SendSellTx),
            _ => None,
        })
        .ok_or_else(|| anyhow::anyhow!("No valid sub menu found"))
}

fn find_keyboard_from_callback(q: &CallbackQuery) -> anyhow::Result<&InlineKeyboardMarkup> {
    q.message
        .as_ref()
        .and_then(|msg| msg.reply_markup())
        .and_then(|keyboard| Some(keyboard))
        .ok_or_else(|| anyhow::anyhow!("No valid sub menu found"))
}

async fn handle_wallet_callback(bot: &Bot, q: &CallbackQuery) -> Result<(), tg_error::TgError> {
    bot.answer_callback_query(&q.id).await?;
    if let Some(wallet) = &q.data {
        if let Some(Message { id, chat, .. }) = &q.message {
            let menu_msg = utils::get_on_chain_info().await?;
            let keyboard = match (find_sub_menu_type_from_callback(q)?, wallet.as_str()) {
                (SubMenuType::SendBuyTx, "Wallet 1") => {
                    buy_keyboard(true, false, true, false, false)?
                }
                (SubMenuType::SendBuyTx, "Wallet 2") => {
                    buy_keyboard(true, false, false, true, false)?
                }
                (SubMenuType::SendBuyTx, "Wallet 3") => {
                    buy_keyboard(true, false, false, false, true)?
                }
                (SubMenuType::SendSellTx, _) => {
                    todo!(); // Handle the SendSellTx case here
                }
                _ => return Ok(()), // Return early if no match
            };

            // Edit the message with the determined keyboard
            bot.edit_message_text(chat.id, *id, menu_msg)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(keyboard)
                .await?;
        }
    }
    Ok(())
}

async fn handle_send_tx_callback(bot: &Bot, q: &CallbackQuery) -> Result<(), tg_error::TgError> {
    bot.answer_callback_query(&q.id).await?;
    match find_sub_menu_type_from_callback(q)? {
        SubMenuType::SendBuyTx => {
            let keyboard = find_keyboard_from_callback(q)?;
            let _req = SendBuyTxRequest::new(keyboard);
            log::info!("req: {:?}", _req);
        }
        SubMenuType::SendSellTx => {
            todo!()
        }
    }

    Ok(())
}

async fn button_callback(bot: Bot, q: CallbackQuery) -> Result<(), tg_error::TgError> {
    if let Some(action) = &q.data {
        match action.as_str() {
            // main-menu buttons
            "Sell" | "Limit Buy" | "Limit Sell" => {
                let keyboard = make_keyboard(action);
                let text = format!("Choose an option for {}:", action);
                bot.answer_callback_query(q.id).await?;
                if let Some(Message { chat, .. }) = q.message {
                    let _ = bot
                        .send_message(chat.id, text)
                        .reply_markup(keyboard)
                        .await?;
                }
            }
            "Buy" => handle_buy_callback(&bot, &q).await?,
            "🏠 Main Menu" => handle_menu_callback(&bot, &q).await?,
            "❌Close" => handle_close_callback(&bot, &q).await?,

            // sub-menu buttons
            _ => match action.as_str() {
                "Send Buy Tx" | "Send Sell Tx" => handle_send_tx_callback(&bot, &q).await?,
                "Wallet 1" | "Wallet 2" | "Wallet 3" => handle_wallet_callback(&bot, &q).await?,
                _ => {}
            },
        }

        log::info!("You chose: {}", action);
    }

    Ok(())
}
