use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EventEntity {
    // FIXME: The ID in the user object is formatted wrong.
    // Currently id is an object which looks like: "_id": { "$oid": "63c530ee0a74a9e466187037" }
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub user_id: String,
    pub title: String,
    pub description: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub start: chrono::DateTime<chrono::Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub end: chrono::DateTime<chrono::Utc>,
    pub all_day: bool,
    pub location: String,
    pub event_uid: Option<String>,
}

impl fmt::Display for EventEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", crate::entity::to_json(self))
    }
}
