use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EventDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub start: String,
    pub end: String,
    pub all_day: bool,
    pub location: String,
}

impl From<crate::entity::event::EventEntity> for EventDto {
    fn from(event: crate::entity::event::EventEntity) -> Self {
        EventDto {
            id: event.id.unwrap().to_string(),
            title: event.title,
            description: event.description,
            start: event.start.to_string(),
            end: event.end.to_string(),
            all_day: event.all_day,
            location: event.location,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateEventDto {
    pub title: String,
    pub description: String,
    pub start: String,
    pub end: String,
    pub all_day: bool,
    pub location: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DeleteEventDto {
    pub id: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateEventDto {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub all_day: Option<bool>,
    pub location: Option<String>,
}
