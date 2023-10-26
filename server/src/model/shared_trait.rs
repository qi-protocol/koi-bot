use sqlb::HasFields;
use sqlx::{postgres::PgRow, FromRow};

use crate::ctx::Ctx;

use super::ModelManager;
use super::{Error, Result};

pub trait DatabaseController {
    const TABLE: &'static str;
}

/// Creates an entity in the database
/// C: DatabaseController
/// E: Entity
pub async fn create<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: E) -> Result<i64>
where
    MC: DatabaseController,
    E: HasFields,
{
    let db = mm.db();

    let fields = data.not_none_fields();
    let (id,) = sqlb::insert()
        .table(MC::TABLE)
        .data(fields)
        .returning(&["id"])
        .fetch_one::<_, (i64,)>(db)
        .await?;

    Ok(id)
}

/// Gets and item from the database by id
/// C: DatabaseController
/// E: Entity
pub async fn get<C, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
    C: DatabaseController,
    E: for<'a> FromRow<'a, PgRow> + Send + Sync + Unpin,
    E: HasFields,
{
    let db = mm.db();
    let entity: E = sqlb::select()
        .table(C::TABLE)
        .columns(E::field_names())
        .and_where("id", "=", id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: C::TABLE,
            id,
        })?;

    Ok(entity)
}

/// Lists items from the database
/// C: DatabaseController
/// E: Entity
pub async fn list<C, E>(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<E>>
where
    C: DatabaseController,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    E: HasFields,
{
    let db = mm.db();

    let entities: Vec<E> = sqlb::select()
        .table(C::TABLE)
        .columns(E::field_names())
        .order_by("id")
        .fetch_all(db)
        .await?;

    Ok(entities)
}

pub async fn update<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64, data: E) -> Result<()>
where
    MC: DatabaseController,
    E: HasFields,
{
    let db = mm.db();

    let fields = data.not_none_fields();
    let count = sqlb::update()
        .table(MC::TABLE)
        .and_where("id", "=", id)
        .data(fields)
        .exec(db)
        .await?;

    if count == 0 {
        Err(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })
    } else {
        Ok(())
    }
}

pub async fn delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
    MC: DatabaseController,
{
    let db = mm.db();

    let count = sqlb::delete()
        .table(MC::TABLE)
        .and_where("id", "=", id)
        .exec(db)
        .await?;

    if count == 0 {
        Err(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })
    } else {
        Ok(())
    }
}
