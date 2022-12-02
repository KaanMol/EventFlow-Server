mod calendar;
mod database;

use actix_web::{
    get, put, post,
    web::{self, Data},
    App, HttpServer, Responder,
};
use icalendar::{Calendar, Component};
use rusqlite::Result;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Errors {
    #[error("Couldn't parse the Calendar. iCal link is probably invalid.")]
    CalendarParseError,

    #[error("Invalid iCal link: {0}")]
    InvalidLinkError(String),
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let database = database::Database::connect();
    let mut calendar2 = calendar::Calendar::new();

    println!("running");

    let mut calendars: Vec<Calendar> = Vec::new();
    let calendar_urls = vec![
    ];

    println!("getting calendars");

    for calendar in calendar_urls {
        let calendar: Result<Calendar, Errors> = get_calendar(String::from(calendar)).await;

        match calendar {
            Ok(calendar) => calendars.push(calendar),
            Err(e) => println!("Error: {:?}", e.to_string()),
        }
    }

    for calendar in calendars.iter() {
        for events in calendar.components.iter() {
            if let None = events.as_event() {
                continue;
            }

            // TODO: Fix this mess
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

            calendar2.add_event(calendar::CalendarEvent {
                name: title.to_string(),
                start_date,
                end_date,
                description: description.to_string(),
            });
        }
    }

    let comparisons = vec![
        calendar::EventComparison {
            field: calendar::Field::Summary,
            comparator: calendar::Filter::Is,
            value: "Servicemedewerker".to_string(),
        },
        calendar::EventComparison {
            field: calendar::Field::Summary,
            comparator: calendar::Filter::Is,
            value: "Niet".to_string(),
        },
        calendar::EventComparison {
            field: calendar::Field::Summary,
            comparator: calendar::Filter::Contains,
            value: "6461PS002W".to_string(),
        },
    ];

    let operations = vec![
        calendar::EventOperation {
            field: calendar::Field::Summary,
            operation: calendar::Filter::Is,
            value: "Niet".to_string(),
            new_value: "Servicemedewerker (Niet beschikbaar)".to_string(),
        },
        calendar::EventOperation {
            field: calendar::Field::Summary,
            operation: calendar::Filter::Is,
            value: "6461PS002W - Introduction to Psychology WG".to_string(),
            new_value: "Inleiding in de Psychologie Werkgroep".to_string(),
        },
    ];

    let filtered_calendar = calendar2.filter(comparisons).operations(operations);
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let actix_data = Data::new(filtered_calendar);
    let app = move || {
        App::new()
            .service(create_user)
            .app_data(Data::clone(&actix_data))
            .service(calendar_route)
            .service(get_icals)
            .service(set_ical)
    };

    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
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


#[put("/seticals/{user_id}/{name}/{ical_url}")]
async fn set_ical(path: web::Path<(String,String,String)>) -> impl Responder {
    let user_id = path.0.to_string();
    let name = path.1.to_string();
    let ical_url = path.2.to_string();

    let db = database::Database::connect();
    
    match db.add_ical(name, ical_url, user_id)
    {
        Ok(_) => "ok".to_string(),
        Err(e) => e.to_string() // todo: make this return error code 50X
    }
}

#[get("/calendar")]
async fn calendar_route(calendar: web::Data<calendar::Calendar>) -> impl Responder {
    calendar.to_ical()
}

#[post("/user/{name}")]
async fn create_user(name: web::Path<String>) -> impl Responder {
    println!("Registering user {:?}", name.as_str());
    let db = database::Database::connect();

    match db.create_user(name.to_string()) {
        Ok(e) => e,
        Err(e) => e.to_string() // todo: make this return error code 50X
    }
}

async fn get_calendar(ical: String) -> Result<Calendar, Errors> {
    let calendar_ics = get_ical_by_url(ical.clone())
        .await
        .map_err(|_| Errors::InvalidLinkError(ical))?;

    Ok(calendar_ics
        .parse::<Calendar>()
        .map_err(|_| Errors::CalendarParseError)?)
}

async fn get_ical_by_url(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get(&url).send().await?.text().await?;

    Ok(body)
}
