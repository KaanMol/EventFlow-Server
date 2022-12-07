use chrono::{NaiveDate, NaiveTime};

#[derive(Debug)]
pub struct Coordinate {
    pub lat: f64,
    pub lon: f64,
}

impl Default for Coordinate {
    fn default() -> Self {
        Self { lat: 0.0, lon: 0.0 }
    }
}

impl Coordinate {
    pub fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }
}

#[derive(Debug)]
pub enum ArrivalDeparture {
    ArriveAt(NaiveTime),
    DepartAt(NaiveTime),
}

#[derive(Debug, Default)]
pub struct Step {
    pub travel_type: String,
    pub location: String,
    pub summary: String,
    pub depart_time: NaiveTime,
}

#[derive(Debug)]
pub struct Route {
    pub from: Coordinate,
    pub to: Coordinate,
    pub date: NaiveDate,
    pub depart_time: NaiveTime,
    pub arrive_time: NaiveTime,
    pub travel_time: chrono::Duration,
    pub steps: Vec<Step>,
}

impl Route {
    pub fn new(
        from: Coordinate,
        to: Coordinate,
        date: NaiveDate,
        depart_time: NaiveTime,
        arrive_time: NaiveTime,
        steps: Vec<Step>,
    ) -> Self {
        Self {
            from,
            to,
            date,
            depart_time,
            arrive_time,
            travel_time: arrive_time.signed_duration_since(depart_time),
            steps,
        }
    }
}
