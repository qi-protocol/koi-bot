use crate::consts::{BOT_NAME, BUY_TOKEN};
use crate::handlers::delete_up_to_messages;
use crate::handlers::find_keyboard_from_message;
use crate::requests::on_chain;
use crate::storages::{TgMessageStorage, GLOBAL_BUY_MENU_STORAGE};
use crate::tg_error;
use ethers::types::Address;
use std::str::FromStr;
use teloxide::{
    dispatching::dialogue::{Dialogue, InMemStorage},
    payloads::EditMessageTextSetters,
    requests::Requester,
    types::{InlineKeyboardButtonKind, Message, ParseMode},
    Bot,
};

pub(crate) type AddressPromptDialogue =
    Dialogue<AddressPromptDialogueState, InMemStorage<AddressPromptDialogueState>>;

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub(crate) enum AddressPromptDialogueState {
    #[default]
    StartAddressPrompt,
    ReceiveAddress,
    ReceiveTokenName,
}

pub(crate) async fn address_dialogue_handler(
    bot: Bot,
    dialogue: AddressPromptDialogue,
    msg: Message,
) -> Result<(), tg_error::TgError> {
    bot.send_message(
        msg.chat.id,
        "Enter the address of the token you want to trade",
    )
    .await?;

    dialogue
        .update(AddressPromptDialogueState::ReceiveAddress)
        .await?;

    Ok(())
}

pub(crate) async fn receiving_address_or_token_handler(
    bot: Bot,
    dialogue: AddressPromptDialogue,
    msg: Message,
) -> Result<(), tg_error::TgError> {
    let text = match msg.text() {
        Some(t) => t,
        _ => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
            return Ok(());
        }
    };

    // Checks if it's valid address
    if text.starts_with("0x") && Address::from_str(text).is_ok() {
        let menu_msg = on_chain::get_on_chain_info().await?;

        if let Some(buy_menu) = GLOBAL_BUY_MENU_STORAGE.get(BOT_NAME.to_string()) {
            let buy_msg = buy_menu.message;
            let buy_msg_id = buy_menu.message_id;
            let keyboard = find_keyboard_from_message(&buy_msg)?;
            let mut new_keyboard = keyboard.clone();

            let new_button_text = format!("{}", text);
            if let Some(button) = new_keyboard
                .inline_keyboard
                .get_mut(4)
                .and_then(|row| row.get_mut(0))
            {
                button.text = new_button_text.to_string();
                button.kind = InlineKeyboardButtonKind::CallbackData(BUY_TOKEN.to_string());
            };

            // Edit the message with the new keyboard
            bot.edit_message_text(msg.chat.id, buy_msg_id, menu_msg)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(new_keyboard)
                .await?;
            dialogue.exit().await?;
            log::info!("last message id: {}", msg.id.0);
            log::info!("last message id: {}", buy_msg_id.0);
            let _ = delete_up_to_messages(&bot, msg.chat.id.0, msg.id.0, buy_msg_id.0).await?;
        } else {
            log::warn!("message not found");
        }
    } else {
        bot.send_message(msg.chat.id, "Please enter valid address")
            .await?;
    };

    Ok(())
}
