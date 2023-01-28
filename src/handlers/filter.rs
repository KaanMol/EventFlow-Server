use crate::{
    entity::{self},
    AppState,
};

use super::error::ResourceError;

pub async fn create_filter(
    user_identity: String,
    url: String,
    new_filter: entity::user::CalendarEventSourceFilters,
    state: actix_web::web::Data<AppState>,
) -> Result<entity::user::CalendarEventSourceFilters, super::error::ResourceError> {
    let filter = mongodb::bson::doc! {
        "identities": {
            "$elemMatch": {
                "$in": [user_identity]
            }
        },
        "sources.url": url.clone(),
    };

    let update = mongodb::bson::doc! {
        "$push": {
            "sources.$.filters": crate::handlers::to_bson(&new_filter)
        }
    };

    let result = state
        .db
        .collection::<entity::user::User>("users")
        .update_one(filter, update, None)
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?;

    if result.matched_count == 0 {
        return Err(ResourceError::FailedDatabaseConnection);
    }

    Ok(new_filter)
}
