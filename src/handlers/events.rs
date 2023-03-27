use crate::app::AppState;

use super::error::ResourceError;
use futures::stream::TryStreamExt;

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

    get_one(new_event.inserted_id.as_object_id().unwrap(), state).await
}

pub async fn exists_by_original(
    identity: String,
    original_id: String,
    state: AppState,
) -> Result<bool, super::error::ResourceError> {
    // Filter on user and original id
    let filter = mongodb::bson::doc! {
        "user_id": identity,
        "original": original_id
    };

    let count = state
        .db
        .collection::<crate::entity::event::EventEntity>("events")
        .count_documents(filter, None)
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?;

    Ok(count > 0)
}

pub async fn get_one(
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

pub async fn get_all(
    user_identity: String,
    state: actix_web::web::Data<crate::app::State>,
) -> Result<Vec<crate::entity::event::EventEntity>, super::error::ResourceError> {
    let mut cursor = state
        .db
        .collection::<crate::entity::event::EventEntity>("events")
        .find(
            mongodb::bson::doc! {
                "user_id": user_identity.clone()
            },
            None,
        )
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?;

    let mut results: Vec<crate::entity::event::EventEntity> = Vec::new();

    while let Ok(Some(event)) = cursor
        .try_next()
        .await
        .or_else(|_| Err(ResourceError::Unknown))
    {
        results.push(event);
    }

    Ok(results)
}
