use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventSourceDto {
    pub name: String,
    pub url: String,
    pub modifiers: Vec<EventSourceModifierDto>,
    pub filters: Vec<EventSourceFilterDto>,
}

impl From<crate::entity::user::CalendarEventSource> for EventSourceDto {
    fn from(event_source: crate::entity::user::CalendarEventSource) -> Self {
        EventSourceDto {
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventSourceModifierDto {
    pub operation: String,
    pub field: String,
    pub value: String,
    pub new_value: String,
}

impl From<crate::entity::user::CalendarEventSourceModifier> for EventSourceModifierDto {
    fn from(modifier: crate::entity::user::CalendarEventSourceModifier) -> Self {
        EventSourceModifierDto {
            operation: modifier.operation,
            field: modifier.field,
            value: modifier.value,
            new_value: modifier.new_value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventSourceFilterDto {
    pub field: String,
    pub operator: String,
    pub value: String,
    pub calendar_id: String,
}

impl From<crate::entity::user::CalendarEventSourceFilters> for EventSourceFilterDto {
    fn from(filter: crate::entity::user::CalendarEventSourceFilters) -> Self {
        EventSourceFilterDto {
            field: filter.field,
            operator: filter.operator,
            value: filter.value,
            calendar_id: filter.calendar_id,
        }
    }
}
