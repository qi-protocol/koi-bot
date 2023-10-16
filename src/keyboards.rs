use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

#[derive(Debug, Clone)]
pub(crate) enum BuyButtons<'a> {
    SendBuyTx,
    SendSellTx,
    MainMenu,
    Close,
    PrivateTx(&'a str),
    Rebate(&'a str),
    Wallet1(&'a str),
    Wallet2(&'a str),
    Wallet3(&'a str),
    Buy,
    Receive,
    BuyAmount,
    EstimatedReceivedAmount,
}

impl<'a> BuyButtons<'a> {
    pub(crate) fn new(text: &'a str) -> Self {
        match text {
            t if t == "Send Buy Tx" || t == add_emoji("Send Buy Tx").as_str() => Self::SendBuyTx,
            t if t == "Send Sell Tx" || t == add_emoji("Send Sell Tx").as_str() => Self::SendSellTx,
            t if t == "Main Menu" || t == add_emoji("Main Menu").as_str() => Self::MainMenu,
            t if t == "Close" || t == add_emoji("Close").as_str() => Self::Close,
            t if t == "Private Tx" || t == add_emoji("Private Tx").as_str() => {
                Self::PrivateTx(text)
            }
            t if t == "Rebate" || t == add_emoji("Rebate").as_str() => Self::Rebate(text),
            t if t == "Wallet 1" || t == add_emoji("Wallet 1").as_str() => Self::Wallet1(text),
            t if t == "Wallet 2" || t == add_emoji("Wallet 2").as_str() => Self::Wallet2(text),
            t if t == "Wallet 3" || t == add_emoji("Wallet 3").as_str() => Self::Wallet3(text),
            "Buy" => Self::Buy,
            "Receives" => Self::Receive,
            "Buy Amount" => Self::BuyAmount,
            "Estimated Received Amount" => Self::EstimatedReceivedAmount,
            _ => Self::SendSellTx,
        }
    }

    pub(crate) fn toggle(&self) -> String {
        match self {
            Self::PrivateTx(text) => self.toggle_text(text, "Private Tx"),
            Self::Rebate(text) => self.toggle_text(text, "Rebate"),
            Self::Wallet1(text) => self.toggle_text(text, "Wallet1"),
            Self::Wallet2(text) => self.toggle_text(text, "Wallet2"),
            Self::Wallet3(text) => self.toggle_text(text, "Wallet3"),
            _ => format!("{:?}", self),
        }
    }

    fn toggle_text(&self, current: &str, default: &str) -> String {
        if current == default {
            add_emoji(default)
        } else {
            default.to_string()
        }
    }
}

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
/// Note: any change to this function will affect the handle_send_tx function() and handle_private_tx_callback()
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

    // 1st row
    keyboard = keyboard.append_row(vec![
        // no need to add emoji in the callback value
        InlineKeyboardButton::callback(add_emoji("Main Menu"), "Main Menu".to_owned()),
        InlineKeyboardButton::callback(add_emoji("Close"), "Close".to_owned()),
    ]);

    // 2nd row
    keyboard = keyboard.append_row(vec![
        match private_tx {
            true => {
                InlineKeyboardButton::callback(add_emoji("Private Tx"), add_emoji("Private Tx"))
            }
            false => {
                InlineKeyboardButton::callback("Private Tx".to_owned(), "Private Tx".to_owned())
            }
        },
        match rebate {
            true => InlineKeyboardButton::callback(add_emoji("Rebate"), add_emoji("Rebate")),
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
            true => InlineKeyboardButton::callback(add_emoji("Wallet 1"), add_emoji("Wallet 1")),
            false => InlineKeyboardButton::callback("Wallet 1".to_owned(), "Wallet 1".to_owned()),
        },
        match wallet2 {
            true => InlineKeyboardButton::callback(add_emoji("Wallet 2"), add_emoji("Wallet 2")),
            false => InlineKeyboardButton::callback("Wallet 2".to_owned(), "Wallet 2".to_owned()),
        },
        match wallet3 {
            true => InlineKeyboardButton::callback(add_emoji("Wallet 3"), add_emoji("Wallet 3")),
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

fn add_emoji(text: &str) -> String {
    let button = match text {
        "Main Menu" => format!("🏠 {}", text),
        "Close" => format!("❌ {}", text),
        "Private Tx" => format!("✅ {}", text),
        "Rebate" => format!("✅ {}", text),
        "Wallet 1" => format!("✅ {}", text),
        "Wallet 2" => format!("✅{}", text),
        "Wallet 3" => format!("✅ {}", text),
        _ => text.to_string(),
    };
    button
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
