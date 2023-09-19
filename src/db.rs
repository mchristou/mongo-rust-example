use futures::StreamExt;
use mongodb::bson::{doc, document::Document};
use mongodb::{options::ClientOptions, Client, Collection};

use crate::{error::Result, request::TodoRequest};

const NAME: &str = "Todo app";
const TITLE: &str = "title";
const DESCRIPTION: &str = "description";
const ITEMS: &str = "items";

#[derive(Clone, Debug)]
pub struct DB {
    client: Client,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
            .await
            .unwrap();
        client_options.app_name = Some(NAME.to_owned());

        let client = Client::with_options(client_options).unwrap();

        Ok(DB { client })
    }

    pub async fn add_item(&self, request: &TodoRequest) -> Result<()> {
        let doc = doc! {
            TITLE: request.title.to_string(),
            DESCRIPTION: request.description.to_string(),
        };

        self.collection().insert_one(doc, None).await.unwrap();

        Ok(())
    }

    pub async fn fetch_items(&self) -> Result<()> {
        todo!()
    }

    pub async fn edit_item(&self, id: u32, request: &TodoRequest) -> Result<()> {
        todo!();
    }

    pub async fn update_item(&self, id: u32, request: &TodoRequest) -> Result<()> {
        todo!();
    }

    pub async fn delete_item(&self, id: u32) -> Result<()> {
        todo!();
    }

    fn collection(&self) -> Collection<Document> {
        self.client.database(NAME).collection(ITEMS)
    }
}
