use sqlx::{postgres::PgRow, FromRow};

use crate::ctx::Ctx;

use super::ModelManager;
use super::{Error, Result};

pub trait DatabaseBackendController {
    const TABLE: &'static str;
}

pub async fn get<C, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
    C: DatabaseBackendController,
    E: for<'a> FromRow<'a, PgRow> + Send + Sync + Unpin,
{
    let db = mm.db();
    let query = format!("SELECT * FROM {} WHERE id = $1", C::TABLE);
    let entity = sqlx::query_as(&query)
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: C::TABLE,
            id,
        })?;

    Ok(entity)
}
