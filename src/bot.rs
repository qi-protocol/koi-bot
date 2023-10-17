use crate::handlers::callback_handlers::{
    handle_buy_callback, handle_close_callback, handle_menu_callback, handle_private_tx_callback,
    handle_send_tx_callback, handle_wallet_callback,
};
use crate::handlers::{delete_previous_messages, matching_sub_menu, SubMenuType};
use crate::keyboards::buy_buttons::BuyButtons;
use crate::keyboards::menu_keyboard;
use crate::requests::on_chain;
use crate::tg_error;
use teloxide::{
    dispatching::UpdateFilterExt,
    dptree,
    error_handlers::LoggingErrorHandler,
    payloads::SendMessageSetters,
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
                let menu_msg = on_chain::get_on_chain_info().await?;

                // send the new message
                let message_sent = bot
                    .send_message(msg.chat.id, menu_msg)
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(keyboard)
                    .await?;
                log::info!("message sent: {:?}", message_sent);

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

async fn button_callback(bot: Bot, q: CallbackQuery) -> Result<(), tg_error::TgError> {
    if let Some(action) = &q.data {
        match action.as_str() {
            // main-menu
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
            "Main Menu" => handle_menu_callback(&bot, &q).await?,
            "Close" => handle_close_callback(&bot, &q).await?,

            // sub-menus
            _ => match matching_sub_menu(&bot, &q) {
                Some(SubMenuType::SendBuyTx) => match BuyButtons::new(action) {
                    BuyButtons::SendBuyTx => handle_send_tx_callback(&bot, &q).await?,
                    BuyButtons::PrivateTx(_) => handle_private_tx_callback(&bot, &q).await?,
                    BuyButtons::Wallet1(_) | BuyButtons::Wallet2(_) | BuyButtons::Wallet3(_) => {
                        handle_wallet_callback(&bot, &q).await?
                    }
                    _ => {}
                },
                Some(SubMenuType::SendSellTx) => {
                    todo!()
                }
                _ => {}
            },
        }

        log::info!("You chose: {}", action);
    }

    Ok(())
}
