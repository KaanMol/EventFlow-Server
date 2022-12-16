use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    sources: Vec<CalenderEventSource>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CalenderEventSource {
    name: String,
    url: String,
    modifiers: Vec<CalenderEventSourceModifier>,
    filter: Vec<CalendarEventSourceFilters>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CalenderEventSourceModifier {
    operation: String,
    field: String,
    value: String,
    new_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CalendarEventSourceFilters {
    field: String,
    operator: String,
    value: String,
    calendar_id: String,
}
