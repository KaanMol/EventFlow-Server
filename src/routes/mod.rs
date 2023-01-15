use super::handlers::error::ResourceError;

pub mod calendar;
pub mod filter;
pub mod modifiers;
pub mod source;
pub mod user;

pub fn parse_id(id: &String) -> Result<mongodb::bson::oid::ObjectId, ResourceError> {
    mongodb::bson::oid::ObjectId::parse_str(id).map_err(|_| ResourceError::NotFoundById(id.clone()))
}

fn to_json<T>(value: T) -> String
where
    T: serde::Serialize,
{
    match serde_json::to_string(&value) {
        Ok(json) => json,
        Err(e) => format!("{}:#?", e),
    }
}

fn to_bson<T>(value: T) -> mongodb::bson::Bson
where
    T: serde::Serialize,
{
    match mongodb::bson::to_bson(&value) {
        Ok(bson) => bson,
        Err(e) => mongodb::bson::Bson::String(format!("{}:#?", e)),
    }
}
