use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use tg_api::bot;

#[tokio::main]
pub async fn main() -> Result<(), bot::TgError> {
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
