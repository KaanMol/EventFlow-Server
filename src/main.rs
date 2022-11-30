use icalendar::{Calendar};

#[derive(thiserror::Error, Debug, Clone)]
pub enum Errors {
    #[error("Couldn't parse the Calendar. iCal link is probably invalid.")]
    CalendarParseError,

    #[error("Invalid iCal link: {0}")]
    InvalidLinkError(String),
}

#[tokio::main]

async fn main() {
    println!("running");

    let mut calendars: Vec<Calendar> = Vec::new();
    let calendar_urls = vec![
               

    ];image.png

    for calendar in calendar_urls {
        let calendar: Result<Calendar, Errors> = get_calendar(String::from(calendar))
        .await;

        match calendar {
            Ok(calendar) => calendars.push(calendar),
            Err(e) => println!("Error: {:?}", e.to_string()),
        }
    }

    println!("calendars: {:?}", calendars);
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