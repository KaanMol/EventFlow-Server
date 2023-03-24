use actix_web::{body::BoxBody, http::StatusCode};
use thiserror::Error;

use super::response::ApiResponse;

#[derive(Error, Debug, Clone)]
pub enum ResourceError {
    #[error("The resource with id `{0}` is not found")]
    NotFoundById(String),

    #[error("The database connection is not available")]
    FailedDatabaseConnection,

    #[error("Could not parse `{0}`")]
    FailedParse(String),

    #[error("Input is invalid: `{0}`")]
    InvalidInput(String),

    #[error("Network failed")]
    NetworkError,

    #[error("Unknown error")]
    Unknown,
}

// Implementation of the ResponseError trait for the ResourceError struct
// This allows us to use ResourceError as a response type in a handler
impl actix_web::error::ResponseError for ResourceError {
    fn error_response(&self) -> actix_web::HttpResponse<BoxBody> {
        // Determine the status code based on the error
        let status_code = match self {
            ResourceError::NotFoundById(_) => StatusCode::NOT_FOUND,
            ResourceError::FailedParse(_) => StatusCode::BAD_REQUEST,
            ResourceError::FailedDatabaseConnection => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        // Create an ApiResponse based on the error
        let response: ApiResponse<()> = ApiResponse::from_error(status_code, self.clone());

        // Convert the ApiResponse to an HttpResponse using the implementation of the From trait
        response.into()
    }
}
