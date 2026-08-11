use std::sync::Arc;
use mongodb::{Client, Database};
use mongodb::error::Error;
use mongodb::options::DatabaseOptions;
use mongodb::results::DatabaseSpecification;

pub struct DatabaseManager {
    client: Arc<Client>
}

impl DatabaseManager {

    async fn list_databases(&self) -> Result<Vec<DatabaseSpecification>, Error> {
        self.client.list_databases().await
    }

    async fn access_database(
        &self,
        database_name: &str,
        database_options: Option<DatabaseOptions>
    ) -> Database {
        match database_options {
            Some(opts) => {
                self.client.database_with_options(
                    database_name,
                    opts
                )
            }
            None => self.client.database(database_name)
        }
    }

    async fn drop_database(&self, database_name: &str) -> Result<(), Error> {
        let db = self.client.database(database_name);
        db.drop().await
    }
    

}