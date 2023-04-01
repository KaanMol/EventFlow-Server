use super::error::ResourceError;

pub async fn create_source(
    user_identity: String,
    new_source: crate::entity::user::EventSource,
    state: actix_web::web::Data<crate::app::State>,
) -> Result<crate::entity::user::EventSource, super::error::ResourceError> {
    if new_source.name.is_empty() {
        return Err(ResourceError::InvalidInput("name".to_string()));
    }

    let regex_source = r"^(https?|webcals)://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)";
    let url_regex = regex::Regex::new(regex_source).unwrap();
    if !url_regex.is_match(&new_source.url) {
        return Err(ResourceError::InvalidInput("url".to_string()));
    }

    let filter = mongodb::bson::doc! {
        "auth_id": user_identity
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

pub async fn sync_sources(
    user_id: String,
    state: actix_web::web::Data<crate::app::State>,
) -> Result<(), super::error::ResourceError> {
    let user = crate::handlers::user::get_user(user_id.clone(), state.clone())
        .await
        .map_err(|_| ResourceError::NotFoundById(user_id.clone()))?;

    for source in user.sources {
        let events =
            crate::common::ical::fetch_and_parse_ical_events(user_id.clone(), source.url).await?;

        // FIXME: This should be done in a batch, not one by one.
        for event in events {
            if event.event_uid.is_some() {
                let exists = crate::handlers::events::exists_by_original(
                    user_id.clone(),
                    event.event_uid.clone().unwrap(),
                    state.clone(),
                )
                .await?;

                if exists {
                    continue;
                }
            }

            crate::handlers::events::create(event, state.clone()).await?;
        }
    }

    Ok(())
}
