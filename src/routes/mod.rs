use super::handlers::error::ResourceError;

pub mod calendar;
pub mod filter;
pub mod modifiers;
pub mod source;
pub mod user;

pub fn parse_id(id: &String) -> Result<mongodb::bson::oid::ObjectId, ResourceError> {
    mongodb::bson::oid::ObjectId::parse_str(id).map_err(|_| ResourceError::FailedParse(id.clone()))
}
