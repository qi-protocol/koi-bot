use crate::handlers::{
    delete_previous_messages, find_keyboard_from_callback, find_sub_menu_type_from_callback,
    SubMenuType,
};
use crate::keyboards::buy_buttons::{buy_keyboard, BuyButtons};
use crate::keyboards::menu_keyboard;
use crate::requests::on_chain;
use crate::requests::server::SendBuyTxRequest;
use crate::tg_error;
use teloxide::{
    payloads::{EditMessageTextSetters, SendMessageSetters},
    prelude::Requester,
    types::{CallbackQuery, InlineKeyboardButtonKind, Message, ParseMode},
    Bot,
};
/// Upon a user clicks the "Main Menu", it'll clear the text and show the menu again
///
pub(crate) async fn handle_menu_callback(
    bot: &Bot,
    q: &CallbackQuery,
) -> Result<(), tg_error::TgError> {
    let keyboard = menu_keyboard();
    bot.answer_callback_query(&q.id).await?;
    if let Some(Message { chat, .. }) = &q.message {
        let menu_msg = on_chain::get_on_chain_info().await?;

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

pub(crate) async fn handle_buy_callback(
    bot: &Bot,
    q: &CallbackQuery,
) -> Result<(), tg_error::TgError> {
    let keyboard = buy_keyboard(true, false, true, false, false)?;
    bot.answer_callback_query(&q.id).await?;
    if let Some(Message { id: _id, chat, .. }) = &q.message {
        let menu_msg = on_chain::get_on_chain_info().await?;
        // todo: add custom info for buy
        let _ = bot
            .send_message(chat.id, menu_msg)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(keyboard)
            .await?;
    }
    Ok(())
}

pub(crate) async fn handle_close_callback(
    bot: &Bot,
    q: &CallbackQuery,
) -> Result<(), tg_error::TgError> {
    bot.answer_callback_query(&q.id).await?;
    if let Some(Message { id, chat, .. }) = &q.message {
        let _ = bot.delete_message(chat.id, *id).await?;
    };
    Ok(())
}

pub(crate) async fn handle_wallet_callback(
    bot: &Bot,
    q: &CallbackQuery,
) -> Result<(), tg_error::TgError> {
    bot.answer_callback_query(&q.id).await?;

    if let (Some(button), Some(Message { id, chat, .. })) = (&q.data, &q.message) {
        let menu_msg = on_chain::get_on_chain_info().await?;
        let sub_menu_type = find_sub_menu_type_from_callback(q)?;
        let buy_button = BuyButtons::new(button);
        let new_button_text = buy_button.toggle();

        if let SubMenuType::SendBuyTx = sub_menu_type {
            let mut keyboard = find_keyboard_from_callback(q)?.clone();

            // Determine the index based on the button type
            let index = match buy_button {
                BuyButtons::Wallet1(_) => 0,
                BuyButtons::Wallet2(_) => 1,
                BuyButtons::Wallet3(_) => 2,
                _ => return Ok(()), // Return early if no match
            };

            // Update the button text and kind
            if let Some(button) = keyboard
                .inline_keyboard
                .get_mut(3)
                .and_then(|row| row.get_mut(index))
            {
                button.text = new_button_text.to_string();
                button.kind = InlineKeyboardButtonKind::CallbackData(new_button_text.to_string());
            }

            // Edit the message with the updated keyboard
            bot.edit_message_text(chat.id, *id, menu_msg)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(keyboard)
                .await?;
        } else if let SubMenuType::SendSellTx = sub_menu_type {
            todo!(); // Handle the SendSellTx case here
        }
    }

    Ok(())
}

// Note: any value changed to the keyboard layout will affect this function
pub(crate) async fn handle_private_tx_callback(
    bot: &Bot,
    q: &CallbackQuery,
) -> Result<(), tg_error::TgError> {
    bot.answer_callback_query(&q.id).await?;
    match find_sub_menu_type_from_callback(q)? {
        SubMenuType::SendBuyTx => {
            if let Some(button) = &q.data {
                if let Some(Message { id, chat, .. }) = &q.message {
                    let menu_msg = on_chain::get_on_chain_info().await?;

                    // Gets current keyboard layout
                    let keyboard = find_keyboard_from_callback(q)?.clone();
                    let mut new_keyboard = keyboard.clone();
                    let button = BuyButtons::new(button);
                    let new_button_text = button.toggle();

                    // Change the text to toggled value
                    if let Some(button) = new_keyboard
                        .inline_keyboard
                        .get_mut(1)
                        .and_then(|row| row.get_mut(0))
                    {
                        button.text = new_button_text.to_string();
                        button.kind =
                            InlineKeyboardButtonKind::CallbackData(new_button_text.to_string());
                    }

                    // Edit the message with the new keyboard
                    bot.edit_message_text(chat.id, *id, menu_msg)
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_markup(new_keyboard)
                        .await?;
                }
            }
        }
        SubMenuType::SendSellTx => {
            todo!()
        }
    }
    Ok(())
}

pub(crate) async fn handle_send_tx_callback(
    bot: &Bot,
    q: &CallbackQuery,
) -> Result<(), tg_error::TgError> {
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
