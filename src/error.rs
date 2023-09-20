use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Mongo db error {0}")]
    MongoDbError(#[from] mongodb::error::Error),
}

impl warp::reject::Reject for Error {}
