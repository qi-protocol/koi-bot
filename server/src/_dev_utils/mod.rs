mod dev_db;

use tokio::sync::OnceCell;

use crate::model::ModelManager;

/// For dev encironment only
/// Will be called from the main()
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();
    INIT.get_or_init(|| async {
        log::info!("Initializing dev utils");
        dev_db::init_dev_db().await.unwrap()
    })
    .await;
}

/// For test encironment only
pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();
    let mm = INIT
        .get_or_init(|| async {
            log::info!("Initializing test utils");
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;
    mm.clone()
}
