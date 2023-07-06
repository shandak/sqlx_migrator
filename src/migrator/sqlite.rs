use sqlx::{Pool, Sqlite};
use std::borrow::Cow;

use super::{DatabaseOperation, Migrate, Migrator};
use crate::error::Error;
use crate::migration::{AppliedMigrationSqlRow, Migration};

/// create migrator table
#[must_use]
pub(crate) fn create_migrator_table_query(table_name: &str) -> Cow<'static, str> {
    format!(
        "CREATE TABLE IF NOT EXISTS {} (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        app TEXT NOT NULL,
        name TEXT NOT NULL,
        applied_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        UNIQUE (app, name)
    )",
        table_name
    )
    .into()
}

/// Drop table
#[must_use]
pub(crate) fn drop_table_query(table_name: &str) -> Cow<'static, str> {
    format!("DROP TABLE IF EXISTS {}", table_name).into()
}

/// fetch rows
pub(crate) async fn fetch_rows(
    pool: &Pool<Sqlite>,
    table_name: &str,
) -> Result<Vec<AppliedMigrationSqlRow>, Error> {
    Ok(sqlx::query_as(&*format!(
        "SELECT id, app, name, applied_time FROM {}",
        table_name
    ))
    .fetch_all(pool)
    .await?)
}

/// add migration query
#[must_use]
pub(crate) fn add_migration_query(table_name: &str) -> Cow<'static, str> {
    format!("INSERT INTO {}(app, name) VALUES ($1, $2)", table_name).into()
}

/// delete migration query
#[must_use]
pub(crate) fn delete_migration_query(table_name: &str) -> Cow<'static, str> {
    format!("DELETE FROM {} WHERE app = $1 AND name = $2", table_name).into()
}

#[async_trait::async_trait]
impl DatabaseOperation<Sqlite> for Migrator<Sqlite> {
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
        migration: &Box<dyn Migration<Sqlite>>,
        connection: &mut <Sqlite as sqlx::Database>::Connection,
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
        migration: &Box<dyn Migration<Sqlite>>,
        connection: &mut <Sqlite as sqlx::Database>::Connection,
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
        _connection: &mut <Sqlite as sqlx::Database>::Connection,
    ) -> Result<(), Error> {
        Ok(())
    }

    async fn unlock(
        &self,
        _connection: &mut <Sqlite as sqlx::Database>::Connection,
    ) -> Result<(), Error> {
        Ok(())
    }
}

impl Migrate<Sqlite> for Migrator<Sqlite> {}
