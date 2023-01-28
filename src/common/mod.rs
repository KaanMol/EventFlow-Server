use crate::handlers::{error::ResourceError, response::ApiResponse};
pub mod database;

pub type Response<T> = std::result::Result<ApiResponse<T>, ResourceError>;
