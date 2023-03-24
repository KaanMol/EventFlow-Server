use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventFilterDto {
    pub field: String,
    pub operator: String,
    pub value: String,
    pub calendar_id: String,
}

impl From<crate::entity::user::CalendarEventSourceFilters> for EventFilterDto {
    fn from(filter: crate::entity::user::CalendarEventSourceFilters) -> Self {
        EventFilterDto {
            field: filter.field,
            operator: filter.operator,
            value: filter.value,
            calendar_id: filter.calendar_id,
        }
    }
}
