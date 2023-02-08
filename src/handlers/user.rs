// use crate::entities::{self};

use super::error::ResourceError;

pub async fn get_user(
    user_identity: String,
    state: actix_web::web::Data<crate::app::State>,
) -> Result<crate::entity::user::User, super::error::ResourceError> {
    let user = state
        .db
        .collection::<crate::entity::user::User>("users")
        .find_one(
            mongodb::bson::doc! {
                "identities": {
                    "$elemMatch": {
                        "$in": [user_identity.clone()]
                    }
                }
            },
            None,
        )
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?
        .ok_or_else(|| ResourceError::NotFoundById(user_identity))?;

    Ok(user)
}

pub async fn create_user(
    user: crate::entity::user::User,
    state: actix_web::web::Data<crate::app::State>,
) -> Result<crate::entity::user::User, super::error::ResourceError> {
    state
        .db
        .collection::<crate::entity::user::User>("users")
        .insert_one(&user, None)
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?;

    get_user(user.identities.get(0).unwrap().to_string(), state).await
}
