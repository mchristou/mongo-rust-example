use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoRequest {
    pub title: String,
    pub description: String,
}

pub mod request {
    use crate::{request::TodoRequest, todo::Todo, DB};
    use warp::{http::StatusCode, reject, reply::json, Reply};

    pub async fn get_list(db: DB) -> Result<impl warp::Reply, warp::Rejection> {
        let items = db.fetch_items().await.unwrap();

        Ok(json(&items))
    }

    pub async fn add_item(item: TodoRequest, db: DB) -> Result<impl warp::Reply, warp::Rejection> {
        db.add_item(&item).await.unwrap();

        Ok(StatusCode::CREATED)
    }

    pub async fn update_item(
        id: u32,
        item: TodoRequest,
        db: DB,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        db.update_item(id, &item).await.unwrap();

        Ok(StatusCode::OK)
    }

    pub async fn delete_item(id: u32, db: DB) -> Result<impl warp::Reply, warp::Rejection> {
        db.delete_item(id).await.unwrap();

        Ok(StatusCode::OK)
    }
}
