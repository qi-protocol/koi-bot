use std::fmt;
use teloxide::dispatching::dialogue::InMemStorageError;

#[derive(Debug)]
#[allow(dead_code)]
pub enum TgError {
    AnyhowError(anyhow::Error),
    Parse(String),
    TeloxideRequest(teloxide::RequestError),
    TeloxideInMemStorageError(InMemStorageError),
    UnmatchedQuery(teloxide::types::CallbackQuery),
    NoQueryData(teloxide::types::CallbackQuery),
    NoQueryMessage(teloxide::types::CallbackQuery),
    UserNotFound(teloxide::types::Message),
}

impl fmt::Display for TgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Parse(ref err) => write!(f, "Parse error: {}", err),
            Self::TeloxideRequest(ref err) => {
                write!(f, "Telegram request error: {}", err)
            }
            Self::TeloxideInMemStorageError(ref err) => {
                write!(f, "InMemStorage error: {}", err)
            }
            Self::UnmatchedQuery(ref cb_query) => {
                write!(f, "Could not match callback query: {:?}", cb_query)
            }
            Self::NoQueryData(ref cb_query) => {
                write!(f, "Could not get query data: {:?}", cb_query)
            }
            Self::NoQueryMessage(ref cb_query) => {
                write!(f, "Could not get query message: {:?}", cb_query)
            }
            Self::UserNotFound(ref msg) => {
                write!(f, "Could not find user for message: {:?}", msg)
            }
            Self::AnyhowError(ref err) => write!(f, "Anyhow error: {}", err),
        }
    }
}

impl From<teloxide::RequestError> for TgError {
    fn from(err: teloxide::RequestError) -> Self {
        Self::TeloxideRequest(err)
    }
}

impl From<InMemStorageError> for TgError {
    fn from(err: InMemStorageError) -> Self {
        Self::TeloxideInMemStorageError(err)
    }
}

impl From<anyhow::Error> for TgError {
    fn from(err: anyhow::Error) -> Self {
        Self::AnyhowError(err)
    }
}
