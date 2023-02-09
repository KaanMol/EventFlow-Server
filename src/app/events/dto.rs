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
