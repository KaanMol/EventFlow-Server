pub mod error;
// pub mod filter;
// pub mod modifier;
pub mod response;
// pub mod source;
pub mod user;

pub fn to_bson<T>(value: T) -> mongodb::bson::Bson
where
    T: serde::Serialize,
{
    match mongodb::bson::to_bson(&value) {
        Ok(bson) => bson,
        Err(e) => mongodb::bson::Bson::String(format!("{}:#?", e)),
    }
}
