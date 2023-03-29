// use crate::entities::{self};

use crate::entity;

use super::error::ResourceError;

pub async fn get_user(
    auth_id: String,
    state: actix_web::web::Data<crate::app::State>,
) -> Result<crate::entity::user::User, super::error::ResourceError> {
    // TODO: Cargo Clippy complains about ok_or_else() being used instead of ok_or(), this is not something with a high priority to fix, but handy to know.
    let user = state
        .db
        .collection::<crate::entity::user::User>("users")
        .find_one(
            mongodb::bson::doc! {
                "auth_id": auth_id.clone()
            },
            None,
        )
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?
        .ok_or_else(|| ResourceError::NotFoundById(auth_id))?;

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

pub async fn update_user(
    user: crate::entity::user::User,
    state: actix_web::web::Data<crate::app::State>,
) -> Result<crate::entity::user::User, super::error::ResourceError> {
    let filter = mongodb::bson::doc! {
        "auth_id": user.clone().auth_id
    };

    state
        .db
        .collection::<crate::entity::user::User>("users")
        .replace_one(filter, &user, None)
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?;

    get_user(user.auth_id, state).await
}
