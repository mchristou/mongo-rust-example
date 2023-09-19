use db::DB;
use error::{Error, Result};
use warp::{Filter, Rejection};

use crate::request::request as Request;

mod db;
mod error;
mod request;
mod todo;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let db = DB::init().await?;

    let todo = warp::path("todo");
    let filter = warp::any().map(move || db.clone());

    let add_item = todo
        .and(warp::post())
        .and(warp::body::json())
        .and(filter.clone())
        .and_then(Request::add_item);

    let update_item = todo
        .and(warp::put())
        .and(warp::path::param())
        .and(warp::body::json())
        .and(filter.clone())
        .and_then(Request::update_item);

    let delete_item = todo
        .and(warp::delete())
        .and(warp::path::param())
        .and(filter.clone())
        .and_then(Request::delete_item);

    let get_item = todo
        .and(warp::get())
        .and(filter.clone())
        .and_then(Request::get_list);

    let routes = add_item.or(update_item).or(delete_item).or(get_item);
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
