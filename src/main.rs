use icalendar::{Calendar, Component, DatePerhapsTime};

#[derive(thiserror::Error, Debug, Clone)]
pub enum Errors {
    #[error("Couldn't parse the Calendar. iCal link is probably invalid.")]
    CalendarParseError,

    #[error("Invalid iCal link: {0}")]
    InvalidLinkError(String),
}

#[derive(Debug)]
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

#[tokio::main]
async fn main() {
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
    
    for calendar in calendars.iter() {
        let mut eventsVector: Vec<CalendarEvent> = Vec::new();

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

        println!("events: {:?}", eventsVector);
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