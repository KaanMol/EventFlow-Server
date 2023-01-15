use core::fmt;

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

impl fmt::Display for CalendarEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", super::to_json(&self))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repeat {
    repeat: String,
    repeat_every: u32,
    repeat_until: String,
}

impl fmt::Display for Repeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", super::to_json(&self))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    color: String,
    icon: String,
    tags: Vec<String>,
    travel_time_minutes: u32,
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", super::to_json(&self))
    }
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

impl fmt::Display for OriginalEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", super::to_json(&self))
    }
}
