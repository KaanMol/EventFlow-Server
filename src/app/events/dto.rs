use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
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
            start: event.start,
            end: event.end,
            all_day: event.all_day,
            location: event.location,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateEventDto {
    pub title: String,
    pub description: String,
    pub start: String,
    pub end: String,
    pub all_day: bool,
    pub location: String,
}