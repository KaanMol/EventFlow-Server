use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub sources: Vec<CalendarEventSource>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", super::to_json(&self))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarEventSource {
    pub name: String,
    pub url: String,
    pub modifiers: Vec<CalendarEventSourceModifier>,
    pub filters: Vec<CalendarEventSourceFilters>,
}

impl fmt::Display for CalendarEventSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", super::to_json(&self))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarEventSourceModifier {
    pub operation: String, // TODO: Convert to enum
    pub field: String,     // TODO: Convert to enum
    pub value: String,
    pub new_value: String,
}

impl fmt::Display for CalendarEventSourceModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", super::to_json(&self))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarEventSourceFilters {
    pub field: String,    // TODO: Convert to enum
    pub operator: String, // TODO: Convert to enum
    pub value: String,
    pub calendar_id: String,
}

impl fmt::Display for CalendarEventSourceFilters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", super::to_json(&self))
    }
}
