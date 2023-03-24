use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::app::{filters::dto::EventFilterDto, rules::dto::EventRuleDto};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventsSourceDto {
    pub name: String,
    pub url: String,
    pub modifiers: Vec<EventRuleDto>,
    pub filters: Vec<EventFilterDto>,
}

impl From<crate::entity::user::CalendarEventSource> for EventsSourceDto {
    fn from(event_source: crate::entity::user::CalendarEventSource) -> Self {
        EventsSourceDto {
            name: event_source.name,
            url: event_source.url,
            modifiers: event_source
                .modifiers
                .into_iter()
                .map(|m| m.into())
                .collect(),
            filters: event_source.filters.into_iter().map(|f| f.into()).collect(),
        }
    }
}
