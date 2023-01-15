use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ResourceError {
    #[error("The resource with id `{0}` is not found")]
    NotFoundById(String),
    #[error("The database connection is not available")]
    FailedDatabaseConnection,
    #[error("unknown error")]
    Unknown,
}
