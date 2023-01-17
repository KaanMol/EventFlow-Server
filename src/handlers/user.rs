use crate::{
    entity::{self},
    AppState,
};

use super::error::ResourceError;

pub async fn get_user(
    user_id: mongodb::bson::oid::ObjectId,
    state: actix_web::web::Data<AppState>,
) -> Result<entity::user::User, super::error::ResourceError> {
    let user = state
        .db
        .collection::<entity::user::User>("users")
        .find_one(
            mongodb::bson::doc! {
                "_id": &user_id
            },
            None,
        )
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?
        .ok_or_else(|| ResourceError::NotFoundById(user_id.to_string()))?;

    Ok(user)
}

pub async fn create_user(
    user: entity::user::User,
    state: actix_web::web::Data<AppState>,
) -> Result<entity::user::User, super::error::ResourceError> {
    let result = state
        .db
        .collection::<crate::entity::user::User>("users")
        .insert_one(&user, None)
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?;

    let id = result
        .inserted_id
        .as_object_id()
        .ok_or_else(|| ResourceError::Unknown)?;

    get_user(id, state).await
}
