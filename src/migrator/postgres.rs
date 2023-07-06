use sqlx::{Pool, Postgres};
use std::borrow::Cow;

use super::{DatabaseOperation, Migrate, Migrator};
use crate::error::Error;
use crate::migration::{AppliedMigrationSqlRow, Migration};

/// Create migrator table query
#[must_use]
pub(crate) fn create_migrator_table_query(table_name: &str) -> Cow<'static, str> {
    format!(
        "CREATE TABLE IF NOT EXISTS {} (
        id INT PRIMARY KEY NOT NULL GENERATED ALWAYS AS IDENTITY,
        app TEXT NOT NULL,
        name TEXT NOT NULL,
        applied_time TIMESTAMPTZ NOT NULL DEFAULT now(),
        UNIQUE (app, name)
    )",
        table_name
    )
    .into()
}

/// Drop table query
#[must_use]
pub(crate) fn drop_table_query(table_name: &str) -> Cow<'static, str> {
    format!("DROP TABLE IF EXISTS {}", table_name).into()
}

/// Fetch rows
pub(crate) async fn fetch_rows(
    pool: &Pool<Postgres>,
    table_name: &str,
) -> Result<Vec<AppliedMigrationSqlRow>, Error> {
    Ok(sqlx::query_as(&*format!(
        "SELECT id, app, name, applied_time FROM {}",
        table_name
    ))
    .fetch_all(pool)
    .await?)
}

/// Add migration query
#[must_use]
pub(crate) fn add_migration_query(table_name: &str) -> Cow<'static, str> {
    format!("INSERT INTO {}(app, name) VALUES ($1, $2)", table_name).into()
}

/// Delete migration query
#[must_use]
pub(crate) fn delete_migration_query(table_name: &str) -> Cow<'static, str> {
    format!("DELETE FROM {} WHERE app = $1 AND name = $2", table_name).into()
}

/// get lock id
pub(crate) async fn lock_id(pool: &Pool<Postgres>) -> Result<i64, Error> {
    let (database_name,): (String,) = sqlx::query_as("SELECT CURRENT_DATABASE()")
        .fetch_one(pool)
        .await?;
    Ok(i64::from(crc32fast::hash(database_name.as_bytes())))
}

/// get lock database query
/// # Errors
/// Failed to lock database
pub(crate) fn lock_database_query() -> &'static str {
    "SELECT pg_advisory_lock($1)"
}

/// get lock database query
/// # Errors
/// Failed to lock database
pub(crate) fn unlock_database_query() -> &'static str {
    "SELECT pg_advisory_unlock($1)"
}

#[async_trait::async_trait]
impl DatabaseOperation<Postgres> for Migrator<Postgres> {
    async fn ensure_migration_table_exists(&self) -> Result<(), Error> {
        sqlx::query(&create_migrator_table_query(&self.table_name))
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn drop_migration_table_if_exists(&self) -> Result<(), Error> {
        sqlx::query(&drop_table_query(&self.table_name))
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn add_migration_to_db_table(
        &self,
        migration: &Box<dyn Migration<Postgres>>,
        connection: &mut <Postgres as sqlx::Database>::Connection,
    ) -> Result<(), Error> {
        sqlx::query(&add_migration_query(&self.table_name))
            .bind(migration.app())
            .bind(migration.name())
            .execute(connection)
            .await?;
        Ok(())
    }

    async fn delete_migration_from_db_table(
        &self,
        migration: &Box<dyn Migration<Postgres>>,
        connection: &mut <Postgres as sqlx::Database>::Connection,
    ) -> Result<(), Error> {
        sqlx::query(&delete_migration_query(&self.table_name))
            .bind(migration.app())
            .bind(migration.name())
            .execute(connection)
            .await?;
        Ok(())
    }

    async fn fetch_applied_migration_from_db(&self) -> Result<Vec<AppliedMigrationSqlRow>, Error> {
        fetch_rows(&self.pool, &self.table_name).await
    }

    async fn lock(
        &self,
        connection: &mut <Postgres as sqlx::Database>::Connection,
    ) -> Result<(), Error> {
        let lock_id = lock_id(&self.pool).await?;
        sqlx::query(lock_database_query())
            .bind(lock_id)
            .execute(connection)
            .await?;
        Ok(())
    }

    async fn unlock(
        &self,
        connection: &mut <Postgres as sqlx::Database>::Connection,
    ) -> Result<(), Error> {
        let lock_id = lock_id(&self.pool).await?;
        sqlx::query(unlock_database_query())
            .bind(lock_id)
            .execute(connection)
            .await?;
        Ok(())
    }
}

impl Migrate<Postgres> for Migrator<Postgres> {}
