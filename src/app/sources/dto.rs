use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, ToSchema)]
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

impl From<EventsSourceDto> for crate::entity::user::EventSource {
    fn from(event_source: EventsSourceDto) -> Self {
        crate::entity::user::EventSource {
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
