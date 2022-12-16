use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarEvent {
    id: String,
    title: String,
    description: String,
    start: chrono::NaiveDateTime,
    end: chrono::NaiveDateTime,
    all_day: bool,
    location: String,
    repeat: Repeat,
    metadata: Metadata,
    original_event: OriginalEvent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repeat {
    repeat: String,
    repeat_every: u32,
    repeat_until: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    color: String,
    icon: String,
    tags: Vec<String>,
    travel_time_minutes: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalEvent {
    title: String,
    description: String,
    start: chrono::NaiveDateTime,
    end: chrono::NaiveDateTime,
    all_day: bool,
    location: String,
}
