mod bot;
#[allow(dead_code)]
mod keyboards;
mod requests;
mod tg_error;
mod utils;

use log::LevelFilter;

#[tokio::main]
async fn main() -> Result<(), tg_error::TgError> {
    pretty_env_logger::formatted_builder()
        .filter(None, LevelFilter::Info)
        .init();
    log::info!("Starting buttons bot...");

    let bot = bot::TgBot::new();
    let _ = bot.init().await;

    Ok(())
}
