use actix_web::{get, post, web::Json};

use crate::{
    app::{
        events::dto::{CreateEventDto, EventDto},
        AppState, UserClaims,
    },
    common::Response,
    entity::event,
    handlers::response::ApiResponse,
};

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
pub async fn create(state: AppState, body: Json<CreateEventDto>) -> Response<EventDto> {
    let new_event = crate::handlers::events::create(
        crate::entity::event::EventEntity {
            id: None,
            title: body.title.clone(),
            description: body.description.clone(),
            start: body.start.clone(),
            end: body.end.clone(),
            all_day: body.all_day.clone(),
            location: body.location.clone(),
        },
        state,
    )
    .await?;

    // Return the response
    Ok(ApiResponse::from_data(new_event.into()))
}
