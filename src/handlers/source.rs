use super::error::ResourceError;

pub async fn create_source(
    user_identity: String,
    new_source: crate::entity::user::EventSource,
    state: actix_web::web::Data<crate::app::State>,
) -> Result<crate::entity::user::EventSource, super::error::ResourceError> {
    if new_source.name.len() == 0 {
        return Err(ResourceError::InvalidInput("Name is empty".to_string()));
    }

    if new_source.url.len() == 0 {
        return Err(ResourceError::InvalidInput("Url is empty".to_string()));
    }

    let url_regex = regex::Regex::new(r"^(https?|webcals)://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)").unwrap();
    if !url_regex.is_match(&new_source.url) {
        return Err(ResourceError::InvalidInput(
            "Url is in an invalid format".to_string(),
        ));
    }

    let filter = mongodb::bson::doc! {
        "identities": {
            "$elemMatch": {
                "$in": [user_identity]
            }
        }
    };

    let update = mongodb::bson::doc! {
        "$push": {
            "sources": super::to_bson(&new_source)
        }
    };

    state
        .db
        .collection::<crate::entity::user::User>("users")
        .update_one(filter, update, None)
        .await
        .map_err(|_| ResourceError::FailedDatabaseConnection)?;

    // TODO: Add check that the source has actually modified the user, if not, return an error.

    Ok(new_source)
}
