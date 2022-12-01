mod database;

use actix_web::{HttpServer, get, web::{self, Data}, Responder, App};
use database::Database;
use icalendar::{Calendar, Component, DatePerhapsTime, EventLike};
use rusqlite::{Connection, Result, NO_PARAMS};

#[derive(thiserror::Error, Debug, Clone)]
pub enum Errors {
    #[error("Couldn't parse the Calendar. iCal link is probably invalid.")]
    CalendarParseError,

    #[error("Invalid iCal link: {0}")]
    InvalidLinkError(String),
}

enum Filters {
    Is,
    IsNot,
    Contains,
    DoesNotContain
}

enum Fields {
    Summary,
    Description,
}

struct EventOperation {
    pub field: Fields, // e.g. "SUMMARY"
    pub operation: Filters, // e.g. "IS", "CONTAINS", "STARTS_WITH", "ENDS_WITH"
    pub value: String, // e.g. "New Event Name"
    pub new_value: String, // e.g. "New Event Name"
}

impl EventOperation {
    pub fn new(field: Fields, operation: Filters, value: &str, new_value: &str) -> Self {
        Self {
            field: field,
            operation: operation,
            value: value.to_string(),
            new_value: new_value.to_string(),
        }
    }
}

struct EventComparison {
    pub field: Fields, // e.g. "SUMMARY"
    pub comparator: Filters, // e.g. "CONTAINS"
    pub value: String, // e.g. "Event Name"
}

impl EventComparison {
    pub fn new(field: Fields, comparator: Filters, value: &str) -> Self {
        Self {
            field: field,
            comparator: comparator,
            value: value.to_string()
        }
    }
}

#[derive(Debug, Clone)]
struct CalendarEvent {
    pub name: String,
    pub start_date: DatePerhapsTime,
    pub end_date: DatePerhapsTime,
    pub description: String,
}


impl CalendarEvent {
    pub fn new(name: String, start_date: DatePerhapsTime, end_date: DatePerhapsTime, description: String) -> Self {
        CalendarEvent {
            name,
            start_date,
            end_date,
            description,
        }
    }
}

struct Calendar2 {
    ical_urls: Vec<String>,
    events: Vec<CalendarEvent>,
}

impl Calendar2 {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            ical_urls: Vec::new(),
        }
    }

    pub fn add_ical_url(&mut self, database: database::Database, url: &str) {
        self.ical_urls.push(url.to_string());
        // database.
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {


    let database = database::Database::connect();

    println!("running");

    let mut calendars: Vec<Calendar> = Vec::new();
    let calendar_urls = vec![
    ];

    println!("getting calendars");

    for calendar in calendar_urls {
        let calendar: Result<Calendar, Errors> = get_calendar(String::from(calendar))
        .await;

        match calendar {
            Ok(calendar) => calendars.push(calendar),
            Err(e) => println!("Error: {:?}", e.to_string()),
        }
    }

    let mut eventsVector: Vec<CalendarEvent> = Vec::new();

    for calendar in calendars.iter() {
        for events in calendar.components.iter() {
            if let None = events.as_event() {
                continue;
            }

            let title = events.as_event().unwrap().get_summary().unwrap();
            let start_date = events.as_event().unwrap().get_start().unwrap();
            let end_date = events.as_event().unwrap().get_end().unwrap();
            let description = {
                if let Some(description) = events.as_event().unwrap().get_description() {
                    description
                } else {
                    ""
                }
            };
            eventsVector.push(CalendarEvent::new(title.to_string(), start_date, end_date, description.to_string()));
        }
    }

    // println!("events: {:?}", eventsVector);

    let comparisons = vec![
        EventComparison::new(Fields::Summary, Filters::Is, "Servicemedewerker"),
        EventComparison::new(Fields::Summary, Filters::Is, "Niet"),
        EventComparison::new(Fields::Summary, Filters::Contains, "6461PS002W - Introduction to Psychology WG"),
    ];

    let operations = vec![
        EventOperation::new(Fields::Summary, Filters::Is, "Niet", "Servicemedewerker (Niet beschikbaar)"),
        EventOperation::new(Fields::Summary, Filters::Is, "6461PS002W - Introduction to Psychology WG", "Inleiding in de Psychologie Werkgroep"),

    ];

    let mut filtered_events: Vec<CalendarEvent> = Vec::new();

    for comparison in comparisons.iter() {
        let vector = eventsVector.clone();

        let mut filtered = vector.into_iter().filter(|event| {
            let event = event.clone();
            let value = match comparison.field {
                Fields::Summary => event.name,
                Fields::Description => event.description,
                _ => "".to_string(),
            };

            match comparison.comparator {
                Filters::Is => value == comparison.value,
                Filters::IsNot => value != comparison.value,
                Filters::Contains => value.contains(&comparison.value),
                Filters::DoesNotContain => !value.contains(&comparison.value),
            }
        }).collect::<Vec<CalendarEvent>>();

        filtered_events.append(&mut filtered);
    }
    
    for operation in operations.iter() {
        for event in filtered_events.iter_mut() {
            match operation.field {
                Fields::Summary => {
                    event.name = do_operation(&event.name, operation);
                }
                Fields::Description => {
                    event.description = do_operation(&event.description, operation);
                },
                _ => {}
            };
        }
    }

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let actix_data = Data::new(filtered_events);
    let app = move || App::new()
            .service(create_user)
            .app_data(Data::clone(&actix_data))
            .service(calendar_route)
            .service(get_icals)
            .service(set_ical);
            

    HttpServer::new(app)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/geticals/{user_id}")]
async fn get_icals(user_id: web::Path<String>) -> impl Responder {
    let db = database::Database::connect();
    let icals = db.get_ical_urls(user_id.to_string()).unwrap();

    println!("icals: {} {:?}", icals.len(), icals);

    icals.join("\n")
}


#[get("/seticals/{user_id}/{name}/{ical_url}")] //I am aware this is not restful, I will fix this later TODO
async fn set_ical(path: web::Path<(String,String,String)>) -> impl Responder {
    let user_id = path.0.to_string();
    let name = path.1.to_string();
    let ical_url = path.2.to_string();

    let db = database::Database::connect();
    db.add_ical(name, ical_url, user_id).unwrap();

    "ok"
}

#[get("/calendar")]
async fn calendar_route(calendar: web::Data<Vec<CalendarEvent>>) -> impl Responder {
    calendar_to_ical(&calendar)
}

#[get("/user/{name}")]
async fn create_user(name: web::Path<String>) -> impl Responder {
    println!("name: {:?}", name);
    let db = database::Database::connect();

    db.create_user(name.to_string()).unwrap()
}


fn do_operation(field: &String, operation: &EventOperation) -> String {
    let field = field.clone();

    match operation.operation {
        Filters::Is => {
            if field == operation.value {
                operation.new_value.clone()
            } else {
                field
            }
        },
        Filters::IsNot => {
            if field == operation.value {
                field
            } else {
                operation.new_value.clone()
            }
        },
        Filters::Contains => {
            if field.contains(&operation.value) {
                operation.new_value.clone()
            } else {
                field
            }
        },
        Filters::DoesNotContain => {
            if !field.contains(&operation.value) {
                operation.new_value.clone()
            } else {
                field
            }
        },
    }
}

async fn get_calendar(ical: String) -> Result<Calendar, Errors> {
    let calendar_ics = get_ical_by_url(ical.clone()).await.map_err(|_| Errors::InvalidLinkError(ical))?;
    
    Ok(calendar_ics.parse::<Calendar>().map_err(|_| Errors::CalendarParseError)?)
}

async fn get_ical_by_url(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get(&url).send()
        .await?
        .text()
        .await?;
    
    Ok(body)
}

fn calendar_to_ical(events: &Vec<CalendarEvent>) -> String {
    let mut calendar = Calendar::new();

    for event in events.iter() {
        let mut calendar_event = icalendar::Event::new();
        calendar_event.summary(&event.name);
        calendar_event.description(&event.description);
        calendar_event.starts(event.start_date.clone());
        calendar_event.ends(event.end_date.clone());
        calendar.push(calendar_event);
    }

    calendar.to_string()
}
