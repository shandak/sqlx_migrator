use sqlx::Transaction;
use sqlx_migrator::error::Error;
use sqlx_migrator::migration::Migration;
use sqlx_migrator::operation::Operation;
pub(crate) struct M0001Operation;

#[async_trait::async_trait]
impl Operation for M0001Operation {
    type Database = sqlx::Sqlite;

    async fn up(&self, _transaction: &mut Transaction<Self::Database>) -> Result<(), Error> {
        Ok(())
    }

    async fn down(&self, _transaction: &mut Transaction<Self::Database>) -> Result<(), Error> {
        Ok(())
    }
}

pub(crate) struct M0001Migration;

#[async_trait::async_trait]
impl Migration for M0001Migration {
    type Database = sqlx::Sqlite;

    fn name(&self) -> String {
        String::from("M001")
    }

    /// Parents of migration
    fn parents(&self) -> Vec<Box<dyn Migration<Database = Self::Database>>> {
        vec![]
    }

    /// Operation performed for migration
    fn operations(&self) -> Vec<Box<dyn Operation<Database = Self::Database>>> {
        vec![Box::new(M0001Operation)]
    }
}
