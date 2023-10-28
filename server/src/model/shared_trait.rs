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
pub async fn create<C, E>(mm: &ModelManager, data: E) -> Result<i64>
where
    C: DatabaseController,
    E: HasFields,
{
    let db = mm.db();

    let fields = data.not_none_fields();
    let (id,) = sqlb::insert()
        .table(C::TABLE)
        .data(fields)
        .returning(&["id"])
        .fetch_one::<_, (i64,)>(db)
        .await?;

    Ok(id)
}

/// Gets and item from the database by id
/// C: DatabaseController
/// E: Entity
pub async fn get<C, E>(mm: &ModelManager, id: i64) -> Result<E>
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
pub async fn list<C, E>(mm: &ModelManager) -> Result<Vec<E>>
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

pub async fn update<C, E>(mm: &ModelManager, id: i64, data: E) -> Result<()>
where
    C: DatabaseController,
    E: HasFields,
{
    let db = mm.db();

    let fields = data.not_none_fields();
    let count = sqlb::update()
        .table(C::TABLE)
        .and_where("id", "=", id)
        .data(fields)
        .exec(db)
        .await?;

    if count == 0 {
        Err(Error::EntityNotFound {
            entity: C::TABLE,
            id,
        })
    } else {
        Ok(())
    }
}

pub async fn delete<C>(mm: &ModelManager, id: i64) -> Result<()>
where
    C: DatabaseController,
{
    let db = mm.db();

    let count = sqlb::delete()
        .table(C::TABLE)
        .and_where("id", "=", id)
        .exec(db)
        .await?;

    if count == 0 {
        Err(Error::EntityNotFound {
            entity: C::TABLE,
            id,
        })
    } else {
        Ok(())
    }
}
