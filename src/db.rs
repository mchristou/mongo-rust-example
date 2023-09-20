use futures::StreamExt;
use mongodb::{
    bson,
    bson::{doc, document::Document},
    options::ClientOptions,
    Client, Collection,
};

use crate::{error::Result, request::TodoRequest, todo_list::TodoList};

const NAME: &str = "todo";
const TITLE: &str = "title";
const DESCRIPTION: &str = "description";
const ITEMS: &str = "items";

#[derive(Clone, Debug)]
pub struct DB {
    client: Client,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some(NAME.to_owned());

        let client = Client::with_options(client_options)?;

        Ok(Self { client })
    }

    pub async fn add_item(&self, request: &TodoRequest) -> Result<()> {
        self.collection()
            .insert_one(self.doc(request), None)
            .await?;

        Ok(())
    }

    pub async fn fetch_items(&self) -> Result<Vec<TodoList>> {
        let mut cursor = self.collection().find(None, None).await?;

        let mut items: Vec<_> = vec![];
        while let Some(Ok(item)) = cursor.next().await {
            items.push(item);
        }

        let res: Vec<_> = items
            .iter()
            .filter_map(|raw| bson::from_bson(raw.into()).ok())
            .collect();

        Ok(res)
    }

    pub async fn update_item(&self, request: &TodoRequest) -> Result<()> {
        let query = doc! {"title": request.title.to_string()};

        self.collection()
            .replace_one(query, self.doc(request), None)
            .await?;

        Ok(())
    }

    pub async fn delete_item(&self, title: &str) -> Result<()> {
        let query = doc! {"title": title};

        self.collection().delete_one(query, None).await?;

        Ok(())
    }

    fn doc(&self, request: &TodoRequest) -> Document {
        doc! {
            TITLE: &request.title,
            DESCRIPTION: &request.description,
        }
    }

    fn collection(&self) -> Collection<Document> {
        self.client.database(NAME).collection(ITEMS)
    }
}
