use crate::{entity::user::User, AppState};

pub async fn get_user(
    user_id: mongodb::bson::oid::ObjectId,
    state: actix_web::web::Data<AppState>,
) -> Result<User, super::error::ResourceError> {
    let user = state
        .db
        .collection::<crate::entity::user::User>("users")
        .find_one(
            mongodb::bson::doc! {
                "_id": &user_id
            },
            None,
        )
        .await
        .map_err(|e| super::error::ResourceError::FailedDatabaseConnection)?
        .ok_or_else(|| super::error::ResourceError::NotFoundById(user_id.to_string()))?;

    Ok(user)
}
