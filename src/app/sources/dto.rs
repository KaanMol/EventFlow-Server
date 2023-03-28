use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EventsSourceDto {
    pub name: String,
    pub url: String,
}

impl From<crate::entity::user::EventSource> for EventsSourceDto {
    fn from(event_source: crate::entity::user::EventSource) -> Self {
        EventsSourceDto {
            name: event_source.name,
            url: event_source.url,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateEventSourceDto {
    pub name: String,
    pub url: String,
}
