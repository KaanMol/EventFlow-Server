use icalendar::{Component, DatePerhapsTime, EventLike};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct DateTime {
    pub date: chrono::NaiveDateTime,
    pub timezone: chrono_tz::Tz,
}

#[derive(Debug, Clone)]
pub struct CalendarEvent {
    pub name: String,
    pub start_date: DateTime,
    pub end_date: DateTime,
    pub description: String,
}

impl CalendarEvent {
    pub fn new(
        name: String,
        start_date: DateTime,
        end_date: DateTime,
        description: String,
    ) -> Self {
        Self {
            name,
            start_date,
            end_date,
            description,
        }
    }
}

pub enum Filter {
    Is,
    IsNot,
    Contains,
    DoesNotContain,
    RegularExpression,
}

pub enum Field {
    Summary,
    Description,
}

pub struct EventComparison {
    pub field: Field,       // e.g. "SUMMARY"
    pub comparator: Filter, // e.g. "CONTAINS"
    pub value: String,      // e.g. "Event Name"
}

pub struct EventOperation {
    pub field: Field,      // e.g. "SUMMARY"
    pub operation: Filter, // e.g. "IS", "CONTAINS", "STARTS_WITH", "ENDS_WITH"
    pub value: String,     // e.g. "New Event Name"
    pub new_value: String, // e.g. "New Event Name"
}

pub struct Calendar {
    events: Vec<CalendarEvent>,
}

impl Calendar {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn add_events(&mut self, events: Vec<CalendarEvent>) {
        self.events.extend(events);
    }

    pub fn add_event(&mut self, event: CalendarEvent) {
        self.events.push(event);
    }

    pub fn filter(&self, comparisons: Vec<EventComparison>) -> Self {
        let mut filtered_events: Vec<CalendarEvent> = Vec::new();

        for comparison in comparisons.iter() {
            let vector = self.events.clone();

            let mut filtered = vector
                .into_iter()
                .filter(|event| {
                    let event = event.clone();
                    let value = match comparison.field {
                        Field::Summary => event.name,
                        Field::Description => event.description,
                    };

                    match comparison.comparator {
                        Filter::Is => value == comparison.value,
                        Filter::IsNot => value != comparison.value,
                        Filter::Contains => value.contains(&comparison.value),
                        Filter::DoesNotContain => !value.contains(&comparison.value),
                        Filter::RegularExpression => match Regex::new(&comparison.value) {
                            Ok(e) => e.is_match(&value),
                            Err(_) => false,
                        },
                    }
                })
                .collect::<Vec<CalendarEvent>>();

            filtered_events.append(&mut filtered);
        }

        Self {
            events: filtered_events,
        }
    }

    pub fn operations(&self, operations: Vec<EventOperation>) {
        let mut events = self.events.clone();

        for operation in operations.iter() {
            for event in events.iter_mut() {
                match operation.field {
                    Field::Summary => {
                        event.name = do_operation(&event.name, operation);
                    }
                    Field::Description => {
                        event.description = do_operation(&event.description, operation);
                    }
                };
            }
        }
    }

    pub fn to_ical(&self) -> String {
        let mut calendar = icalendar::Calendar::new();

        for event in self.events.iter() {
            let mut calendar_event = icalendar::Event::new();

            let start_date = DatePerhapsTime::DateTime(icalendar::CalendarDateTime::WithTimezone {
                date_time: event.start_date.date,
                tzid: event.start_date.timezone.to_string(),
            });

            let end_date = DatePerhapsTime::DateTime(icalendar::CalendarDateTime::WithTimezone {
                date_time: event.end_date.date,
                tzid: event.end_date.timezone.to_string(),
            });

            calendar_event.summary(&event.name);
            calendar_event.description(&event.description);
            calendar_event.starts(start_date);
            calendar_event.ends(end_date);
            calendar.push(calendar_event);
        }

        calendar.to_string()
    }
}

fn do_operation(field: &String, operation: &EventOperation) -> String {
    let field = field.clone();

    match operation.operation {
        Filter::Is => {
            if field == operation.value {
                operation.new_value.clone()
            } else {
                field
            }
        }
        Filter::IsNot => {
            if field == operation.value {
                field
            } else {
                operation.new_value.clone()
            }
        }
        Filter::Contains => {
            if field.contains(&operation.value) {
                operation.new_value.clone()
            } else {
                field
            }
        }
        Filter::DoesNotContain => {
            if !field.contains(&operation.value) {
                operation.new_value.clone()
            } else {
                field
            }
        }
        Filter::RegularExpression => {
            if match Regex::new(&operation.value) {
                Ok(e) => e.is_match(&field),
                Err(_) => false,
            } {
                operation.new_value.clone()
            } else {
                field
            }
        }
    }
}
