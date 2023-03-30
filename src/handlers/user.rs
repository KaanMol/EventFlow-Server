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
    let mut updated_user = get_user(user.auth_id.clone(), state.clone()).await?;

    let mut added_sources = Vec::new();

    // Find all the sources that are new
    for source in user.sources.clone() {
        if updated_user.sources.contains(&source) == false {
            added_sources.push(source);
        }
    }

    // Remove all the sources that are no longer in the user's list
    for source in updated_user.sources.clone() {
        if user.sources.contains(&source) == false {
            updated_user.sources.retain(|x| x != &source);
        }
    }

    // Check if the new sources are valid, if they are, add them to the user's list
    // TODO: These error messages should be more descriptive and describe *which* source is invalid.
    for new_source in added_sources {
        if new_source.name.len() == 0 {
            // return Err(ResourceError::InvalidInput("name".to_string()));
            continue;
        }

        let regex_source = r"^(https?|webcals)://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)";
        let url_regex = regex::Regex::new(regex_source).unwrap();
        if !url_regex.is_match(&new_source.url) {
            // return Err(ResourceError::InvalidInput("url".to_string()));
            continue;
        }

        updated_user.sources.push(new_source);
    }

    let filter = mongodb::bson::doc! {
        "auth_id": user.auth_id.clone()
    };

    state
        .db
        .collection::<crate::entity::user::User>("users")
        .replace_one(filter, &updated_user, None)
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?;

    get_user(user.auth_id, state).await
}
