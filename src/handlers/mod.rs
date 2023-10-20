pub(crate) mod callback_handlers;
pub(crate) mod dialogue_handlers;

use crate::tg_error;
use teloxide::{
    prelude::Requester,
    types::{CallbackQuery, ChatId, InlineKeyboardMarkup, Message, MessageId},
    Bot,
};
use tokio::time::{sleep, Duration};
/// Helper function to delete 10 previous messages
pub(crate) async fn delete_previous_messages(
    bot: &Bot,
    chat_id: i64,
    last_message_id: i32,
    number_of_deletes: i32,
) -> Result<(), tg_error::TgError> {
    log::info!("last message id: {}", last_message_id);
    for message_id in (last_message_id - number_of_deletes..=last_message_id).rev() {
        log::info!("last message id: {}", message_id);
        sleep(Duration::from_millis(10)).await;
        let _ = bot
            .delete_message(ChatId(chat_id), MessageId(message_id))
            .await;
    }
    Ok(())
}

#[derive(Debug)]
pub(crate) enum SubMenuType {
    SendBuyTx,
    SendSellTx,
}

/// Gets the last vec in the larger vec in the InlineKeyboardMarkup. See https://docs.rs/teloxide/latest/teloxide/types/struct.InlineKeyboardMarkup.html
/// Gets the last button in the last vec, which should either be "Send Buy Tx" or "Send Sell Tx"
pub(crate) fn find_sub_menu_type_from_callback(q: &CallbackQuery) -> anyhow::Result<SubMenuType> {
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

pub(crate) fn find_sub_menu_type_from_message(msg: &Message) -> anyhow::Result<SubMenuType> {
    msg.reply_markup()
        .and_then(|keyboard| keyboard.inline_keyboard.last())
        .and_then(|last_vec| last_vec.last())
        .and_then(|last_button| match last_button.text.as_str() {
            "Send Buy Tx" => Some(SubMenuType::SendBuyTx),
            "Send Sell Tx" => Some(SubMenuType::SendSellTx),
            _ => None,
        })
        .ok_or_else(|| anyhow::anyhow!("No valid sub menu found"))
}

pub(crate) fn find_keyboard_from_callback(
    q: &CallbackQuery,
) -> anyhow::Result<&InlineKeyboardMarkup> {
    q.message
        .as_ref()
        .and_then(|msg| msg.reply_markup())
        .and_then(|keyboard| Some(keyboard))
        .ok_or_else(|| anyhow::anyhow!("No valid sub menu found"))
}

pub(crate) fn find_keyboard_from_message(msg: &Message) -> anyhow::Result<&InlineKeyboardMarkup> {
    msg.reply_markup()
        .ok_or_else(|| anyhow::anyhow!("No valid sub menu found"))
}

pub(crate) fn matching_sub_menu(_bot: &Bot, q: &CallbackQuery) -> Option<SubMenuType> {
    find_sub_menu_type_from_callback(q).ok()
}
