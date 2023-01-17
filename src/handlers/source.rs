use crate::{
    entity::{self, user},
    AppState,
};

use super::error::ResourceError;

pub async fn create_source(
    id: mongodb::bson::oid::ObjectId,
    new_source: user::CalendarEventSource,
    state: actix_web::web::Data<AppState>,
) -> Result<user::CalendarEventSource, super::error::ResourceError> {
    let filter = mongodb::bson::doc! {
        "_id": id
    };

    let update = mongodb::bson::doc! {
        "$push": {
            "sources": super::to_bson(&new_source)
        }
    };

    state
        .db
        .collection::<entity::user::User>("users")
        .update_one(filter, update, None)
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?;

    Ok(new_source)
}
