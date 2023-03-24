use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventRuleDto {
    pub operation: String,
    pub field: String,
    pub value: String,
    pub new_value: String,
}

impl From<crate::entity::user::CalendarEventSourceModifier> for EventRuleDto {
    fn from(modifier: crate::entity::user::CalendarEventSourceModifier) -> Self {
        EventRuleDto {
            operation: modifier.operation,
            field: modifier.field,
            value: modifier.value,
            new_value: modifier.new_value,
        }
    }
}
