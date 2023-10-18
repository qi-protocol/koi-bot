use crate::consts::BUY_TOKEN;
use crate::handlers::{find_keyboard_from_message, find_sub_menu_type_from_message, SubMenuType};
use crate::requests::on_chain;
use crate::tg_error;
use ethers::types::Address;
use std::str::FromStr;
use teloxide::{
    dispatching::dialogue::{Dialogue, InMemStorage},
    payloads::SendMessageSetters,
    requests::Requester,
    types::{InlineKeyboardButtonKind, Message, ParseMode},
    Bot,
};

type AddressPromptDialogue =
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
    log::info!("Updating state caonimato ReceiveAddress");
    bot.send_message(
        msg.chat.id,
        "Enter the address of the token you want to trade",
    )
    .await?;
    log::info!("Updating state to ReceiveAddress");

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
    log::info!("text: {}", text);

    // Checks if it's valid address
    if text.starts_with("0x") && Address::from_str(text).is_ok() {
        match find_sub_menu_type_from_message(&msg)? {
            SubMenuType::SendBuyTx => {
                let menu_msg = on_chain::get_on_chain_info().await?;
                let keyboard = find_keyboard_from_message(&msg)?;
                let mut new_keyboard = keyboard.clone();

                let new_button_text = format!("{}{}", BUY_TOKEN, text);
                if let Some(button) = new_keyboard
                    .inline_keyboard
                    .get_mut(4)
                    .and_then(|row| row.get_mut(0))
                {
                    button.text = new_button_text.to_string();
                    button.kind =
                        InlineKeyboardButtonKind::CallbackData(new_button_text.to_string());
                };
                // Edit the message with the new keyboard
                bot.send_message(msg.chat.id, menu_msg)
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(new_keyboard)
                    .await?;
                dialogue.exit().await?;
            }
            SubMenuType::SendSellTx => {
                todo!()
            }
        }
    } else {
        bot.send_message(msg.chat.id, "Please enter valid address")
            .await?;
    };

    Ok(())
}
