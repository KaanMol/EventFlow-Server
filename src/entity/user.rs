use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub sources: Vec<CalenderEventSource>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", super::to_json(&self))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalenderEventSource {
    pub name: String,
    pub url: String,
    pub modifiers: Vec<CalenderEventSourceModifier>,
    pub filters: Vec<CalendarEventSourceFilters>,
}

impl fmt::Display for CalenderEventSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", super::to_json(&self))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalenderEventSourceModifier {
    operation: String,
    field: String,
    value: String,
    new_value: String,
}

impl fmt::Display for CalenderEventSourceModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", super::to_json(&self))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarEventSourceFilters {
    field: String,
    operator: String,
    value: String,
    calendar_id: String,
}

impl fmt::Display for CalendarEventSourceFilters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", super::to_json(&self))
    }
}
