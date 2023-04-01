// use crate::entities::{self};

use super::error::ResourceError;

pub async fn get_user(
    user_identity: String,
    state: actix_web::web::Data<crate::app::State>,
) -> Result<crate::entity::user::User, super::error::ResourceError> {
    // TODO: Cargo Clippy complains about ok_or_else() being used instead of ok_or(), this is not something with a high priority to fix, but handy to know.
    let user = state
        .db
        .collection::<crate::entity::user::User>("users")
        .find_one(
            mongodb::bson::doc! {
                "auth_id": user_identity.clone()
            },
            None,
        )
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?
        .ok_or(ResourceError::NotFoundById(user_identity))?;

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

    get_user(user.auth_id, state).await
}
