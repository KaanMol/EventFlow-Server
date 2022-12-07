mod calendar;
mod database;
// mod tests;
mod routes;

use actix_web::{
    get, post, put,
    web::{self, Data},
    App, HttpServer, Responder,
};
use icalendar::{Calendar, Component, DatePerhapsTime};
// use rusqlite::Result;
use serde::Deserialize;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Errors {
    #[error("Couldn't parse the Calendar. iCal link is probably invalid.")]
    CalendarParseError,

    #[error("Invalid iCal link: {0}")]
    InvalidLinkError(String),
}
#[derive(Debug, Clone)]
pub struct AppState {
    conn: sea_orm::DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let database = database::Database::connect().expect("Failed to connect to database");
    // //let users = database
    let time = ns_scraper::scrape("URL".to_string()).await;
    println!("Expected traveltime: {}", time);
    // return Ok("");

    // let database = database::Database::connect();
    let mut calendar2 = calendar::Calendar::new();

    println!("running");

    let mut calendars: Vec<Calendar> = Vec::new();
    let calendar_urls = vec!["https://rooster.universiteitleiden.nl/ical?63879a34&eu=czM1MjYyMDg=&h=51dOpLAHe66E7JZLBpxfnKSLWj3m-acHtjbF8CO9x48="];

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
            let start_date = dateperhapstime_to_datetime(start_date);

            let end_date = events.as_event().unwrap().get_end().unwrap();
            let end_date = dateperhapstime_to_datetime(end_date);

            println!("date: {:?}", start_date);

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

    let filtered_calendar = calendar2.filter(comparisons); // .operations(operations);
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let conn = sea_orm::Database::connect(String::from("sqlite://users.sqlite?mode=rwc"))
        .await
        .unwrap();

    let state = AppState { conn };

    let actix_data = Data::new(filtered_calendar);
    let app = move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(routes::create_user)
    };

    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
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

fn dateperhapstime_to_datetime(date: DatePerhapsTime) -> calendar::DateTime {
    // TODO: unwraps ...

    let mut timezone = chrono_tz::UTC;

    let date = match date {
        icalendar::DatePerhapsTime::Date(date) => {
            let date =
                chrono::NaiveDateTime::new(date, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            // chrono::Utc.from_local_datetime(&date).unwrap()
            date
        }
        icalendar::DatePerhapsTime::DateTime(date) => match date {
            icalendar::CalendarDateTime::Floating(date) => {
                // chrono::Utc.from_local_datetime(&date).unwrap()
                date
            }
            icalendar::CalendarDateTime::Utc(date) => date.naive_utc(),
            icalendar::CalendarDateTime::WithTimezone { date_time, tzid } => {
                timezone = tzid.parse().unwrap();
                date_time
                // timezone = tzid.parse().unwrap();
                // let local_date_time = timezone.from_local_datetime(&date_time).unwrap();
                // local_date_time.with_timezone(&chrono::Utc)
            }
        },
    };

    calendar::DateTime { date, timezone }
}
