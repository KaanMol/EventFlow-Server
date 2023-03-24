use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    // FIXME: The ID in the user object is formatted wrong.
    // Currently id is an object which looks like: "_id": { "$oid": "63c530ee0a74a9e466187037" }
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub name: String,
    pub identities: Vec<String>,
    pub sources: Vec<CalendarEventSource>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", super::to_json(self))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarEventSource {
    pub name: String,
    pub url: String,
}

impl fmt::Display for CalendarEventSourceFilters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", super::to_json(self))
    }
}
