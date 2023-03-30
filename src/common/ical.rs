use icalendar::Component;

use crate::handlers::error::ResourceError;

pub async fn fetch_and_parse_ical_events(
    user_id: String,
    ical_uri: String,
) -> Result<Vec<crate::entity::event::EventEntity>, ResourceError> {
    // Request source
    let client = reqwest::Client::new();
    let ical_body = client
        .get(&ical_uri.clone())
        .send()
        .await
        .map_err(|_| ResourceError::NetworkError)?
        .text()
        .await
        .map_err(|_| ResourceError::FailedParse("response body".to_string()))?;

    let events = parse_ical(user_id, ical_body).await?;

    Ok(events)
}

async fn parse_ical(
    user_id: String,
    ical: String,
) -> Result<Vec<crate::entity::event::EventEntity>, ResourceError> {
    let calendar = ical
        .parse::<icalendar::Calendar>()
        .map_err(|_| ResourceError::FailedParse("calendar body".to_string()))?;

    let mut result = Vec::new();

    for component in calendar.iter() {
        let event = match component.as_event() {
            Some(event) => event,
            None => continue,
        };

        let title = event.get_summary().ok_or(ResourceError::FailedParse(
            "event title is empty".to_string(),
        ))?;

        let description = event.get_description().unwrap_or("");

        let start = event
            .get_start()
            .ok_or(ResourceError::FailedParse(
                "event start is empty".to_string(),
            ))?
            .to_utc()?;

        let end = event
            .get_end()
            .ok_or(ResourceError::FailedParse("event end is empty".to_string()))?
            .to_utc()?;

        let location = event.property_value("LOCATION").unwrap_or("");

        let id = event
            .get_uid()
            .ok_or(ResourceError::FailedParse("event id is empty".to_string()))?;

        let event = crate::entity::event::EventEntity {
            id: None,
            user_id: user_id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            start,
            end,
            all_day: start - end == chrono::Duration::min_value(),
            location: location.to_string(),
            event_uid: Some(id.to_string()),
        };

        result.push(event);
    }

    Ok(result)
}

trait ToUtc {
    fn to_utc(self) -> Result<chrono::DateTime<chrono::Utc>, ResourceError>;
}

impl ToUtc for icalendar::DatePerhapsTime {
    fn to_utc(self) -> Result<chrono::DateTime<chrono::Utc>, ResourceError> {
        // let mut timezone = chrono_tz::UTC;

        let date: chrono::NaiveDateTime = match self {
            // Converts a date to a datetime with a time of 00:00:00
            icalendar::DatePerhapsTime::Date(date) => {
                chrono::NaiveDateTime::new(date, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            }

            // Converts a datetime to a datetime with the timezone set to UTC
            icalendar::DatePerhapsTime::DateTime(date) => match date {
                // If is already UTC, just return the date
                icalendar::CalendarDateTime::Utc(date) => date.naive_utc(),

                // If it has a timezone, parse it and convert it to UTC
                icalendar::CalendarDateTime::WithTimezone { date_time, tzid: _ } => {
                    // timezone = match tzid.parse() {
                    //     Ok(tz) => tz,
                    //     Err(_) => {
                    //         return Err(ResourceError::FailedParse("ical timezone".to_string()))
                    //     }
                    // };

                    date_time
                }

                // If the timezone is floating, just return the date
                icalendar::CalendarDateTime::Floating(date) => date,
            },
        };

        // FIXME: the offset should be calculated from the timezone, not be hardcoded to UTC
        let result: Result<chrono::DateTime<chrono::Utc>, ResourceError> =
            Ok(chrono::DateTime::<chrono::Utc>::from_utc(date, chrono::Utc));
        result
    }
}

trait ToTimezone {
    fn to_timezone(&self) -> Result<chrono_tz::Tz, ResourceError>;
}

impl ToTimezone for &String {
    fn to_timezone(&self) -> Result<chrono_tz::Tz, ResourceError> {
        self.parse()
            .map_err(|_| ResourceError::FailedParse("ical timezone".to_string()))
    }
}
