// use icalendar::{Component, DatePerhapsTime, EventLike};
// use regex::Regex;

// #[derive(Debug, Clone)]
// pub struct DateTime {
//     pub date: chrono::NaiveDateTime,
//     pub timezone: chrono_tz::Tz,
// }

// #[derive(Debug, Clone)]
// pub struct CalendarEvent {
//     pub name: String,
//     pub start_date: DateTime,
//     pub end_date: DateTime,
//     pub description: String,
// }

// impl CalendarEvent {
//     pub fn new(
//         name: String,
//         start_date: DateTime,
//         end_date: DateTime,
//         description: String,
//     ) -> Self {
//         Self {
//             name,
//             start_date,
//             end_date,
//             description,
//         }
//     }
// }

// pub enum Filter {
//     Is,
//     IsNot,
//     Contains,
//     DoesNotContain,
//     RegularExpression,
// }

// pub enum Field {
//     Summary,
//     Description,
// }

// pub struct EventComparison {
//     pub field: Field,       // e.g. "SUMMARY"
//     pub comparator: Filter, // e.g. "CONTAINS"
//     pub value: String,      // e.g. "Event Name"
// }

// pub struct EventOperation {
//     pub field: Field,      // e.g. "SUMMARY"
//     pub operation: Filter, // e.g. "IS", "CONTAINS", "STARTS_WITH", "ENDS_WITH"
//     pub value: String,     // e.g. "New Event Name"
//     pub new_value: String, // e.g. "New Event Name"
// }

// pub struct Calendar {
//     events: Vec<CalendarEvent>,
// }

// impl Calendar {
//     pub fn new() -> Self {
//         Self { events: Vec::new() }
//     }

//     pub fn add_events(&mut self, events: Vec<CalendarEvent>) {
//         self.events.extend(events);
//     }

//     pub fn add_event(&mut self, event: CalendarEvent) {
//         self.events.push(event);
//     }

//     pub fn filter(&self, comparisons: Vec<EventComparison>) -> Self {
//         let mut filtered_events: Vec<CalendarEvent> = Vec::new();

//         for comparison in comparisons.iter() {
//             let vector = self.events.clone();

//             let mut filtered = vector
//                 .into_iter()
//                 .filter(|event| {
//                     let event = event.clone();
//                     let value = match comparison.field {
//                         Field::Summary => event.name,
//                         Field::Description => event.description,
//                     };

//                     match comparison.comparator {
//                         Filter::Is => value == comparison.value,
//                         Filter::IsNot => value != comparison.value,
//                         Filter::Contains => value.contains(&comparison.value),
//                         Filter::DoesNotContain => !value.contains(&comparison.value),
//                         Filter::RegularExpression => match Regex::new(&comparison.value) {
//                             Ok(e) => e.is_match(&value),
//                             Err(_) => false,
//                         },
//                     }
//                 })
//                 .collect::<Vec<CalendarEvent>>();

//             filtered_events.append(&mut filtered);
//         }

//         Self {
//             events: filtered_events,
//         }
//     }

//     pub fn operations(&self, operations: Vec<EventOperation>) {
//         let mut events = self.events.clone();

//         for operation in operations.iter() {
//             for event in events.iter_mut() {
//                 match operation.field {
//                     Field::Summary => {
//                         event.name = do_operation(&event.name, operation);
//                     }
//                     Field::Description => {
//                         event.description = do_operation(&event.description, operation);
//                     }
//                 };
//             }
//         }
//     }

//     pub fn to_ical(&self) -> String {
//         let mut calendar = icalendar::Calendar::new();

//         for event in self.events.iter() {
//             let mut calendar_event = icalendar::Event::new();

//             let start_date = DatePerhapsTime::DateTime(icalendar::CalendarDateTime::WithTimezone {
//                 date_time: event.start_date.date,
//                 tzid: event.start_date.timezone.to_string(),
//             });

//             let end_date = DatePerhapsTime::DateTime(icalendar::CalendarDateTime::WithTimezone {
//                 date_time: event.end_date.date,
//                 tzid: event.end_date.timezone.to_string(),
//             });

//             calendar_event.summary(&event.name);
//             calendar_event.description(&event.description);
//             calendar_event.starts(start_date);
//             calendar_event.ends(end_date);
//             calendar.push(calendar_event);
//         }

//         calendar.to_string()
//     }
// }

// fn do_operation(field: &String, operation: &EventOperation) -> String {
//     let field = field.clone();

//     match operation.operation {
//         Filter::Is => {
//             if field == operation.value {
//                 operation.new_value.clone()
//             } else {
//                 field
//             }
//         }
//         Filter::IsNot => {
//             if field == operation.value {
//                 field
//             } else {
//                 operation.new_value.clone()
//             }
//         }
//         Filter::Contains => {
//             if field.contains(&operation.value) {
//                 operation.new_value.clone()
//             } else {
//                 field
//             }
//         }
//         Filter::DoesNotContain => {
//             if !field.contains(&operation.value) {
//                 operation.new_value.clone()
//             } else {
//                 field
//             }
//         }
//         Filter::RegularExpression => {
//             if match Regex::new(&operation.value) {
//                 Ok(e) => e.is_match(&field),
//                 Err(_) => false,
//             } {
//                 operation.new_value.clone()
//             } else {
//                 field
//             }
//         }
//     }
// }

// async fn get_calendar(ical: String) -> Result<Calendar, Errors> {
//     let calendar_ics = get_ical_by_url(ical.clone())
//         .await
//         .map_err(|_| Errors::InvalidLinkError(ical))?;

//     Ok(calendar_ics
//         .parse::<Calendar>()
//         .map_err(|_| Errors::CalendarParseError)?)
// }

// async fn get_ical_by_url(url: String) -> Result<String, Box<dyn std::error::Error>> {
//     let client = reqwest::Client::new();
//     let body = client.get(&url).send().await?.text().await?;

//     Ok(body)
// }

// impl Into<calendar::DateTime> for DatePerhapsTime {
//     fn into(self) -> calendar::DateTime {
//         let mut timezone = chrono_tz::UTC;

//         let date = match self {
//             icalendar::DatePerhapsTime::Date(date) => {
//                 let date = chrono::NaiveDateTime::new(
//                     date,
//                     chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
//                 );
//                 date
//             }
//             icalendar::DatePerhapsTime::DateTime(date) => match date {
//                 icalendar::CalendarDateTime::Floating(date) => date,
//                 icalendar::CalendarDateTime::Utc(date) => date.naive_utc(),
//                 icalendar::CalendarDateTime::WithTimezone { date_time, tzid } => {
//                     timezone = tzid.parse().unwrap();
//                     date_time
//                 }
//             },
//         };

//         calendar::DateTime { date, timezone }
//     }
// }

// // // let database = database::Database::connect().expect("Failed to connect to database");
// // // //let users = database

// // // return Ok("");

// // // let database = database::Database::connect();
// // let mut calendar2 = calendar::Calendar::new();

// // println!("running");

// // let mut calendars: Vec<Calendar> = Vec::new();
// // let calendar_urls = vec!["https://rooster.universiteitleiden.nl/ical?63879a34&eu=czM1MjYyMDg=&h=51dOpLAHe66E7JZLBpxfnKSLWj3m-acHtjbF8CO9x48="];

// // println!("getting calendars");

// // for calendar in calendar_urls {
// //     let calendar: Result<Calendar, Errors> = get_calendar(String::from(calendar)).await;

// //     match calendar {
// //         Ok(calendar) => calendars.push(calendar),
// //         Err(e) => println!("Error: {:?}", e.to_string()),
// //     }
// // }

// // for calendar in calendars.iter() {
// //     for events in calendar.components.iter() {
// //         if let None = events.as_event() {
// //             continue;
// //         }

// //         // TODO: Fix this mess
// //         let title = events.as_event().unwrap().get_summary().unwrap();
// //         let start_date = events.as_event().unwrap().get_start().unwrap();
// //         let start_date = dateperhapstime_to_datetime(start_date);

// //         let end_date = events.as_event().unwrap().get_end().unwrap();
// //         let end_date = dateperhapstime_to_datetime(end_date);

// //         println!("date: {:?}", start_date);

// //         let description = {
// //             if let Some(description) = events.as_event().unwrap().get_description() {
// //                 description
// //             } else {
// //                 ""
// //             }
// //         };

// //         calendar2.add_event(calendar::CalendarEvent {
// //             name: title.to_string(),
// //             start_date,
// //             end_date,
// //             description: description.to_string(),
// //         });
// //     }
// // }

// // let comparisons = vec![
// //     calendar::EventComparison {
// //         field: calendar::Field::Summary,
// //         comparator: calendar::Filter::Is,
// //         value: "Servicemedewerker".to_string(),
// //     },
// //     calendar::EventComparison {
// //         field: calendar::Field::Summary,
// //         comparator: calendar::Filter::Is,
// //         value: "Niet".to_string(),
// //     },
// //     calendar::EventComparison {
// //         field: calendar::Field::Summary,
// //         comparator: calendar::Filter::Contains,
// //         value: "6461PS002W".to_string(),
// //     },
// // ];

// // let operations = vec![
// //     calendar::EventOperation {
// //         field: calendar::Field::Summary,
// //         operation: calendar::Filter::Is,
// //         value: "Niet".to_string(),
// //         new_value: "Servicemedewerker (Niet beschikbaar)".to_string(),
// //     },
// //     calendar::EventOperation {
// //         field: calendar::Field::Summary,
// //         operation: calendar::Filter::Is,
// //         value: "6461PS002W - Introduction to Psychology WG".to_string(),
// //         new_value: "Inleiding in de Psychologie Werkgroep".to_string(),
// //     },
// // ];

// // let filtered_calendar = calendar2.filter(comparisons); // .operations(operations);
// // std::env::set_var("RUST_LOG", "debug");
// // env_logger::init();

// // // let conn = sea_orm::Database::connect(String::from("sqlite://db.sqlite?mode=rwc"))
// // //     .await
// // //     .unwrap();
