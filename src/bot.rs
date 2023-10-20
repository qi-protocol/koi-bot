use crate::consts::{BUY, CLOSE, MAIN_MENU};
use crate::handlers::callback_handlers::{
    handle_buy_callback, handle_buy_token_callback, handle_close_callback, handle_menu_callback,
    handle_private_tx_callback, handle_rebate_callback, handle_send_tx_callback,
    handle_wallet_callback,
};
use crate::handlers::dialogue_handlers::{
    address_dialogue_handler, receiving_address_or_token_handler, AddressPromptDialogueState,
};
use crate::handlers::{delete_previous_messages, matching_sub_menu, SubMenuType};
use crate::keyboards::buy_buttons::BuyButtons;
use crate::keyboards::menu_keyboard;
use crate::requests::on_chain;
use crate::tg_error;
use std::sync::Arc;
use teloxide::dispatching::HandlerExt;
use teloxide::{
    dispatching::{dialogue::InMemStorage, UpdateFilterExt},
    dptree,
    error_handlers::LoggingErrorHandler,
    payloads::SendMessageSetters,
    prelude::{Dispatcher, Requester},
    types::{CallbackQuery, Message, ParseMode, Update},
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
            .branch(Update::filter_message().filter_command::<Command>().endpoint(command_callback))
            .branch(Update::filter_callback_query().endpoint(button_callback))
            .branch(
                 Update::filter_message()
                     .enter_dialogue::<Message,InMemStorage<AddressPromptDialogueState>,AddressPromptDialogueState>()
                         .branch(dptree::case![AddressPromptDialogueState::StartAddressPrompt]
                             .endpoint(address_dialogue_handler))
                         .branch(dptree::case![AddressPromptDialogueState::ReceiveAddress]
                             .endpoint(receiving_address_or_token_handler))
            );

        Dispatcher::builder(self.bot, handler)
            .error_handler(LoggingErrorHandler::with_custom_text(
                "An error has occurred in the dispatcher",
            ))
            .dependencies(dptree::deps![
                InMemStorage::<AddressPromptDialogueState>::new()
            ])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
        Ok(())
    }
}

async fn command_callback(bot: Bot, cmd: Command, msg: Message) -> Result<(), tg_error::TgError> {
    match cmd {
        Command::Help => {
            let _ = bot
                .send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Menu => {
            let keyboard = menu_keyboard();
            let menu_msg = on_chain::get_on_chain_info().await?;

            // send the new message
            let message_sent = bot
                .send_message(msg.chat.id, menu_msg)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(keyboard)
                .await?;

            // delete previous messages
            let last_message_id = message_sent.id;
            let _ =
                delete_previous_messages(&bot, msg.chat.id.0, last_message_id.0 - 1, 20).await?;
        }
        Command::Start => {
            sleep(Duration::from_secs(3)).await;
            let keyboard = menu_keyboard();
            let menu_msg = on_chain::get_on_chain_info_start().await?;

            // send the new message
            let _message_sent = bot
                .send_message(msg.chat.id, menu_msg)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(keyboard)
                .await?;
        }
        Command::Wallets => {
            todo!()
        }
        Command::History => {
            todo!()
        }
    }
    Ok(())
}

async fn button_callback(
    bot: Bot,
    q: CallbackQuery,
    storage: Arc<InMemStorage<AddressPromptDialogueState>>,
) -> Result<(), tg_error::TgError> {
    if let Some(action) = &q.data {
        match action.as_str() {
            // main-menu
            BUY => handle_buy_callback(&bot, &q).await?,
            MAIN_MENU => handle_menu_callback(&bot, &q).await?,
            CLOSE => handle_close_callback(&bot, &q).await?,

            // sub-menus
            _ => match matching_sub_menu(&bot, &q) {
                Some(SubMenuType::SendBuyTx) => match BuyButtons::new(action) {
                    BuyButtons::SendBuyTx => handle_send_tx_callback(&bot, &q).await?,
                    BuyButtons::PrivateTx(_) => handle_private_tx_callback(&bot, &q).await?,
                    BuyButtons::Rebate(_) => handle_rebate_callback(&bot, &q).await?,
                    BuyButtons::Wallet1(_) | BuyButtons::Wallet2(_) | BuyButtons::Wallet3(_) => {
                        handle_wallet_callback(&bot, &q).await?
                    }
                    BuyButtons::BuyToken => {
                        handle_buy_token_callback(
                            &bot,
                            AddressPromptDialogueState::StartAddressPrompt,
                            &q,
                            storage,
                        )
                        .await?
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
