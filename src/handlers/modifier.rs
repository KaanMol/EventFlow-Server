use crate::{
    entity::{self},
    AppState,
};

use super::error::ResourceError;

pub async fn create_modifier(
    user_identity: String,
    url: String,
    new_modifier: entity::user::CalendarEventSourceModifier,
    state: actix_web::web::Data<AppState>,
) -> Result<entity::user::CalendarEventSourceModifier, super::error::ResourceError> {
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
            "sources.$.modifiers": crate::handlers::to_bson(&new_modifier)
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

    Ok(new_modifier)
}
