use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reply::json};

use crate::db::DB;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoRequest {
    pub title: String,
    pub description: String,
}

pub async fn fetch_items(db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    let items = db.fetch_items().await?;

    Ok(json(&items))
}

pub async fn add_item(item: TodoRequest, db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    db.add_item(&item).await?;

    Ok(StatusCode::CREATED)
}

pub async fn update_item(item: TodoRequest, db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    db.update_item(&item).await?;

    Ok(StatusCode::OK)
}

pub async fn delete_item(title: String, db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    db.delete_item(&title).await?;

    Ok(StatusCode::OK)
}
