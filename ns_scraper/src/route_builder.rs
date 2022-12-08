use chrono::NaiveTime;

use crate::{
    route::{ArrivalDeparture, Coordinate, Step},
    route_finder::RouteFinder,
};

pub struct RouteFinderBuilder {
    from: Option<Coordinate>,
    to: Option<Coordinate>,
    date: Option<chrono::NaiveDate>,
    time: Option<ArrivalDeparture>,
}

impl Default for RouteFinderBuilder {
    fn default() -> Self {
        let now = chrono::Local::now().naive_local();

        Self {
            from: None,
            to: None,
            date: Some(now.date()),
            time: Some(ArrivalDeparture::DepartAt(now.time())),
        }
    }
}

impl RouteFinderBuilder {
    pub fn new() -> Self {
        RouteFinderBuilder::default()
    }

    pub fn from(mut self, from: Coordinate) -> Self {
        self.from.insert(from.into());
        self
    }

    pub fn to(mut self, to: Coordinate) -> Self {
        self.to.insert(to.into());
        self
    }

    pub fn date(mut self, date: chrono::NaiveDate) -> Self {
        self.date.insert(date.into());
        self
    }

    pub fn arrive_at(mut self, time: chrono::NaiveTime) -> Self {
        self.time.insert(ArrivalDeparture::ArriveAt(time).into());
        self
    }

    pub fn depart_at(mut self, time: chrono::NaiveTime) -> Self {
        self.time.insert(ArrivalDeparture::DepartAt(time).into());
        self
    }

    pub fn build(self) -> Result<RouteFinder, String> {
        let from = self.from.ok_or("from is required")?;
        let to = self.to.ok_or("to is required")?;
        let date = self.date.ok_or("date is required")?;
        let time = self.time.ok_or("arrival or departure time is required")?;

        Ok(RouteFinder {
            from,
            to,
            date,
            time,
        })
    }
}

#[derive(Debug, Clone)]
pub struct StepBuilder {
    travel_type: Option<String>,
    location: Option<String>,
    summary: Option<String>,
    depart_time: Option<NaiveTime>,
}

impl StepBuilder {
    pub fn new() -> StepBuilder {
        StepBuilder {
            travel_type: None,
            location: None,
            summary: None,
            depart_time: None,
        }
    }

    pub fn travel_type(&mut self, travel_type: impl Into<String>) {
        self.travel_type.insert(travel_type.into());
    }

    pub fn location(&mut self, location: impl Into<String>) {
        self.location.insert(location.into());
    }

    pub fn summary(&mut self, summary: impl Into<String>) {
        self.summary.insert(summary.into());
    }

    pub fn depart_time(&mut self, depart_time: NaiveTime) {
        self.depart_time.insert(depart_time.into());
    }

    pub fn build(self) -> Step {
        Step {
            travel_type: self.travel_type.expect("travel_type is required"),
            location: self.location.expect("location is required"),
            summary: self.summary.expect("summary is required"),
            depart_time: self.depart_time.expect("depart_time is required"),
        }
    }
}
