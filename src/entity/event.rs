use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEntity {
    // FIXME: The ID in the user object is formatted wrong.
    // Currently id is an object which looks like: "_id": { "$oid": "63c530ee0a74a9e466187037" }
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub title: String,
    pub description: String,
    pub start: String,
    pub end: String,
    pub all_day: bool,
    pub location: String,
    pub user_id: String,
    // repeat: Repeat,
    // metadata: EventMetadata,
    // original_event: OriginalEvent,
}

impl fmt::Display for EventEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", crate::entity::to_json(self))
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
        write!(f, "{}", crate::entity::to_json(self))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventMetadata {
    color: String,
    icon: String,
    tags: Vec<String>,
    travel_time_minutes: u32,
}

impl fmt::Display for EventMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", crate::entity::to_json(self))
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
        write!(f, "{}", crate::entity::to_json(self))
    }
}
