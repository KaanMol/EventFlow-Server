use actix_web::{delete, get, post, put, web::Json};
use bson::oid::ObjectId;

use crate::{
    app::{
        events::dto::{CreateEventDto, DeleteEventDto, EventDto},
        AppState, UserClaims,
    },
    common::Response,
    handlers::{error::ResourceError, response::ApiResponse},
};

use super::dto::UpdateEventDto;

#[utoipa::path(
	context_path = "/events",
	tag = "Events",
	request_body = CreateEventDto,
    responses(
        (status = 200, description = "Created calendar", body = EventDto),
		(status = 401, description = "Authorization token missing or invalid", body = String)
    )
)]
#[post("")]
pub async fn create(
    state: AppState,
    body: Json<CreateEventDto>,
    user_claims: UserClaims,
) -> Response<EventDto> {
    // Parse the dates from the request body
    let start_date = chrono::DateTime::parse_from_rfc2822(&body.start)
        .map_err(|_| ResourceError::FailedParse("start date".to_string()))?;

    let end_date = chrono::DateTime::parse_from_rfc2822(&body.end)
        .map_err(|_| ResourceError::FailedParse("end date".to_string()))?;

    // Create the event in the database
    let new_event = crate::handlers::events::create(
        crate::entity::event::EventEntity {
            id: None,
            title: body.title.clone(),
            description: body.description.clone(),
            start: start_date.with_timezone(&chrono::Utc),
            end: end_date.with_timezone(&chrono::Utc),
            all_day: body.all_day,
            location: body.location.clone(),
            user_id: user_claims.into_inner().cid,
            event_uid: None,
        },
        state,
    )
    .await?;

    // Return the response
    Ok(ApiResponse::from_data(new_event.into()))
}

#[utoipa::path(
	context_path = "/events",
	tag = "Events",
	request_body = CreateEventDto,
    responses(
        (status = 200, description = "Array of events", body = [EventDto]),
		(status = 401, description = "Authorization token missing or invalid", body = String)
    )
)]
#[get("")]
pub async fn read_all(state: AppState, user_claims: UserClaims) -> Response<Vec<EventDto>> {
    // Get all the events from the database
    let raw_events = crate::handlers::events::get_all(user_claims.into_inner().cid, state).await?;

    // Convert the raw events into DTOs
    let events: Vec<EventDto> = raw_events.into_iter().map(|event| event.into()).collect();

    // Return the response
    Ok(ApiResponse::from_data(events))
}

#[delete("")]
pub async fn delete(
    body: Json<DeleteEventDto>,
    state: AppState,
    user_claims: UserClaims,
) -> Response<()> {
    let id =
        ObjectId::parse_str(&body.id).map_err(|_| ResourceError::FailedParse("id".to_string()))?;

    let event = crate::handlers::events::get_one(id, state.clone()).await?;

    if event.user_id != user_claims.into_inner().cid {
        return Err(ResourceError::Unknown); // TODO: Return an unauthorized error
    }

    crate::handlers::events::delete(id, state.clone()).await?;

    // Return the response
    Ok(ApiResponse::from_data(()))
}

#[put("")]
pub async fn update(
    body: Json<UpdateEventDto>,
    state: AppState,
    user_claims: UserClaims,
) -> Response<EventDto> {
    let id =
        ObjectId::parse_str(&body.id).map_err(|_| ResourceError::FailedParse("id".to_string()))?;

    let original_event = crate::handlers::events::get_one(id, state.clone()).await?;

    if original_event.user_id != user_claims.into_inner().cid {
        return Err(ResourceError::Unknown); // TODO: Return an unauthorized error
    }

    let title = match &body.title {
        Some(title) => title.clone(),
        None => original_event.title,
    };

    let description = match &body.description {
        Some(description) => description.clone(),
        None => original_event.description,
    };

    let start_date = match &body.start {
        Some(start) => {
            // FIXME: Should time convertion from a user given string be done in a dedicated function?
            let start_date = chrono::DateTime::parse_from_rfc2822(&start)
                .map_err(|_| ResourceError::FailedParse("start date".to_string()))?;
            start_date.with_timezone(&chrono::Utc)
        }
        None => original_event.start,
    };

    let end_date = match &body.end {
        Some(end) => {
            let end_date = chrono::DateTime::parse_from_rfc2822(&end)
                .map_err(|_| ResourceError::FailedParse("end date".to_string()))?;
            end_date.with_timezone(&chrono::Utc)
        }
        None => original_event.end,
    };

    let all_day = match &body.all_day {
        Some(all_day) => *all_day,
        None => original_event.all_day,
    };

    let location = match &body.location {
        Some(location) => location.clone(),
        None => original_event.location,
    };

    let updated_event = crate::handlers::events::update(
        crate::entity::event::EventEntity {
            id: Some(id),
            title,
            description,
            start: start_date,
            end: end_date,
            all_day,
            location,
            user_id: original_event.user_id,
            event_uid: original_event.event_uid,
        },
        state,
    )
    .await?;

    Ok(ApiResponse::from_data(updated_event.into()))
}
