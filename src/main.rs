mod bot;
mod consts;
#[allow(dead_code)]
mod handlers;
#[allow(dead_code)]
mod keyboards;
mod requests;
#[allow(dead_code)]
mod storages;
mod tg_error;

use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), tg_error::TgError> {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] [{}] - {}",
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    log::info!("Starting buttons bot...");

    let bot = bot::TgBot::new();
    let _ = bot.init().await;

    Ok(())
}
