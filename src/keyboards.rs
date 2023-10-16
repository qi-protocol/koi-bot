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

/// Create the Buy keyboard layout
/// Note: any change to this function will affect the handle_send_tx function()
fn create_buy_keyboard(
    private_tx: bool,
    rebate: bool,
    wallet1: bool,
    wallet2: bool,
    wallet3: bool,
) -> anyhow::Result<InlineKeyboardMarkup> {
    if [wallet1, wallet2, wallet3].iter().filter(|&&x| x).count() != 1 {
        return Err(anyhow::anyhow!("Only one wallet can be selected"));
    };

    let mut keyboard = InlineKeyboardMarkup::default();
    let add_emoji = |text: &str| match text {
        "Main Menu" => format!("🏠 {}", text),
        "Close" => format!("❌ {}", text),
        "Private Tx" => format!("✅ {}", text),
        "Rebate" => format!("✅ {}", text),
        "Wallet 1" => format!("✅ {}", text),
        "Wallet 2" => format!("✅{}", text),
        "Wallet 3" => format!("✅ {}", text),
        _ => text.to_string(),
    };

    // 1st row
    keyboard = keyboard.append_row(vec![
        InlineKeyboardButton::callback(add_emoji("Main Menu"), "Main Menu".to_owned()),
        InlineKeyboardButton::callback(add_emoji("Close"), "Close".to_owned()),
    ]);

    // 2nd row
    keyboard = keyboard.append_row(vec![
        match private_tx {
            true => {
                InlineKeyboardButton::callback(add_emoji("Private Tx"), "Private Tx".to_owned())
            }
            false => {
                InlineKeyboardButton::callback("Private Tx".to_owned(), "Private Tx".to_owned())
            }
        },
        match rebate {
            true => InlineKeyboardButton::callback(add_emoji("Rebate"), "Rebate".to_owned()),
            false => InlineKeyboardButton::callback("Rebate".to_owned(), "Rebate".to_owned()),
        },
    ]);

    // 3rd row
    keyboard = keyboard.append_row(vec![InlineKeyboardButton::callback(
        "=Select Wallet=".to_owned(),
        "=Select Wallet=".to_owned(),
    )]);

    // 4th row
    // Default selection to wallet 1
    keyboard = keyboard.append_row(vec![
        match wallet1 {
            true => InlineKeyboardButton::callback(add_emoji("Wallet 1"), "Wallet 1".to_owned()),
            false => InlineKeyboardButton::callback("Wallet 1".to_owned(), "Wallet 1".to_owned()),
        },
        match wallet2 {
            true => InlineKeyboardButton::callback(add_emoji("Wallet 2"), "Wallet 2".to_owned()),
            false => InlineKeyboardButton::callback("Wallet 2".to_owned(), "Wallet 2".to_owned()),
        },
        match wallet3 {
            true => InlineKeyboardButton::callback(add_emoji("Wallet 3"), "Wallet 3".to_owned()),
            false => InlineKeyboardButton::callback("Wallet 3".to_owned(), "Wallet 3".to_owned()),
        },
    ]);

    // 5th row
    keyboard = keyboard.append_row(vec![
        InlineKeyboardButton::callback("Buy".to_owned(), "Buy".to_owned()),
        InlineKeyboardButton::callback("Receives".to_owned(), "Receives".to_owned()),
    ]);

    // 6th row
    keyboard = keyboard.append_row(vec![InlineKeyboardButton::callback(
        "Buy Amount".to_owned(),
        "Buy Amount".to_owned(),
    )]);

    // 7th row
    keyboard = keyboard.append_row(vec![InlineKeyboardButton::callback(
        "Estimated Received Amount".to_owned(),
        "Estimated Received Amount".to_owned(),
    )]);

    // 8th row
    // Last one will always be Send Buy Tx
    keyboard = keyboard.append_row(vec![InlineKeyboardButton::callback(
        "Send Buy Tx".to_owned(),
        "Send Buy Tx".to_owned(),
    )]);

    Ok(keyboard)
}

pub(crate) fn buy_keyboard(
    private_tx: bool,
    rebate: bool,
    wallet1: bool,
    wallet2: bool,
    wallet3: bool,
) -> anyhow::Result<InlineKeyboardMarkup> {
    match create_buy_keyboard(private_tx, rebate, wallet1, wallet2, wallet3) {
        Ok(keyboard) => Ok(keyboard),
        _ => Err(anyhow::anyhow!("Error creating keyboard")),
    }
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
