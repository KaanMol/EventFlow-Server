use crate::entity::user::EventSource;

pub async fn create_source(
    auth_id: String,
    source: crate::entity::user::EventSource,
    state: actix_web::web::Data<crate::app::State>,
) -> Result<crate::entity::user::User, super::error::ResourceError> {
    let mut user = crate::handlers::user::get_user(auth_id.clone(), state.clone()).await?;

    user.sources.push(source.clone());

    crate::handlers::user::update_user(user, state.clone()).await
}

pub async fn delete_source(
    auth_id: String,
    source: EventSource,
    state: actix_web::web::Data<crate::app::State>,
) -> Result<crate::entity::user::User, super::error::ResourceError> {
    let mut user = crate::handlers::user::get_user(auth_id.clone(), state.clone()).await?;

    let mut sources = Vec::new();
    for user_source in user.sources {
        if user_source.url != source.url && user_source.name != source.name {
            sources.push(user_source);
        }
    }
    user.sources = sources;

    crate::handlers::user::update_user(user, state.clone()).await
}

pub async fn sync_sources(
    user_id: String,
    state: actix_web::web::Data<crate::app::State>,
) -> Result<(), super::error::ResourceError> {
    let user = crate::handlers::user::get_user(user_id.clone(), state.clone()).await?;

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
