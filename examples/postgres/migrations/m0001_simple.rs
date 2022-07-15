use sqlx::Transaction;
use sqlx_migrator::error::Error;
use sqlx_migrator::migration::Migration;
use sqlx_migrator::operation::Operation;

pub(crate) struct M0001Operation;

#[async_trait::async_trait]
impl Operation for M0001Operation {
    type Database = sqlx::Postgres;

    async fn up(&self, transaction: &mut Transaction<Self::Database>) -> Result<(), Error> {
        sqlx::query("CREATE TABLE sample (id INTEGER PRIMARY KEY, name TEXT)")
            .execute(transaction)
            .await?;
        Ok(())
    }

    async fn down(&self, transaction: &mut Transaction<Self::Database>) -> Result<(), Error> {
        sqlx::query("DROP TABLE sample")
            .execute(transaction)
            .await?;
        Ok(())
    }
}

pub(crate) struct M0001Migration;

#[async_trait::async_trait]
impl Migration for M0001Migration {
    type Database = sqlx::Postgres;

    fn app(&self) -> &str {
        "main"
    }

    fn name(&self) -> &str {
        "m0001_simple"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<Database = Self::Database>>> {
        vec![]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Database = Self::Database>>> {
        vec![Box::new(M0001Operation)]
    }
}
