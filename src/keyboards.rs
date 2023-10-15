use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

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

pub(crate) fn menu_keyboard() -> InlineKeyboardMarkup {
    create_keyboard(vec!["Buy", "Sell", "Limit Buy", "Limit Sell"])
}

pub(crate) fn buy_keyboard() -> InlineKeyboardMarkup {
    create_keyboard(vec!["BTC", "ETH", "LTC", "BCH", "Main Menu", "Close"])
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
