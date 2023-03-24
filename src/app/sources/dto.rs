use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateEventSourceDto {
    pub name: String,
    pub url: String,
}
