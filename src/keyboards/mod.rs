pub(crate) mod buy_buttons;

use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

/// Default layout for the keyboard
fn create_keyboard(actions: Vec<&str>) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    for action in actions.chunks(3) {
        let row = action
            .iter()
            .map(|&action| InlineKeyboardButton::callback(action.to_owned(), action.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

fn add_emoji(text: &str) -> String {
    let button = match text {
        "main menu" => format!("🏠 {}", text),
        "close" => format!("❌ {}", text),
        "private tx" => format!("✅ {}", text),
        "rebate" => format!("✅ {}", text),
        "wallet 1" => format!("✅ {}", text),
        "wallet 2" => format!("✅{}", text),
        "wallet 3" => format!("✅ {}", text),
        _ => text.to_string(),
    };
    button
}

pub(crate) fn menu_keyboard() -> InlineKeyboardMarkup {
    create_keyboard(vec!["Buy", "Sell", "Limit Buy", "Limit Sell"])
}

pub(crate) fn sell_keyboard() -> InlineKeyboardMarkup {
    create_keyboard(vec!["BTC", "ETH", "LTC", "BCH", "Main Menu", "Close"])
}

pub(crate) fn limit_buy_keyboard() -> InlineKeyboardMarkup {
    create_keyboard(vec!["BTC", "ETH", "LTC", "BCH", "Main Menu", "Close"])
}

pub(crate) fn limit_sell_keyboard() -> InlineKeyboardMarkup {
    create_keyboard(vec!["BTC", "ETH", "LTC", "BCH", "Main Menu", "Close"])
}
