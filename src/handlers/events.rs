use crate::{app::AppState, entity::event};

use super::error::ResourceError;

pub async fn create(
    event: crate::entity::event::EventEntity,
    state: AppState,
) -> Result<crate::entity::event::EventEntity, super::error::ResourceError> {
    println!("create event: {:?}", event);

    let new_event = state
        .db
        .collection::<crate::entity::event::EventEntity>("events")
        .insert_one(&event, None)
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?;

    get(new_event.inserted_id.as_object_id().unwrap(), state).await
}

pub async fn get(
    event_id: mongodb::bson::oid::ObjectId,
    state: actix_web::web::Data<crate::app::State>,
) -> Result<crate::entity::event::EventEntity, super::error::ResourceError> {
    let event = state
        .db
        .collection::<crate::entity::event::EventEntity>("events")
        .find_one(
            mongodb::bson::doc! {
                "_id": event_id
            },
            None,
        )
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?
        .ok_or_else(|| ResourceError::NotFoundById(event_id.to_string()))?;

    Ok(event)
}
