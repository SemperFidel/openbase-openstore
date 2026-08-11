use mongodb::{Collection, Database};
use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::options::CollectionOptions;

pub struct CollectionManager {
    database: Database
}

impl CollectionManager {
    fn new(database: Database) -> Self {
        Self { database }
    }

    async fn access_collection(
        &self,
        collection_name: &str,
        collection_options: Option<CollectionOptions>
    ) -> Collection<Document> {
        match collection_options {
            Some(opts) => {
                self.database.collection_with_options(
                    collection_name,
                    opts
                )
            }
            None => self.database.collection(collection_name)
        }
    }

    async fn create_collection(&self, collection_name: &str) -> Result<(), Error> {
        self.database.create_collection(collection_name).await
    }
}