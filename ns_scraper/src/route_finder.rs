use chrono::{format, NaiveDate, NaiveTime};

use crate::route::{ArrivalDeparture, Coordinate, Route, Step};

#[derive(Debug)]
pub struct RouteFinder {
    pub from: Coordinate,
    pub to: Coordinate,
    pub date: NaiveDate,
    pub time: ArrivalDeparture,
}

impl RouteFinder {
    // TODO: Return a Result with proper error handling
    pub fn find(self) -> Route {
        // TODO: Should we be storing the browser somewhere?
        let browser = headless_chrome::Browser::default().expect("Could not create browser");

        // 1. Build the request URL
        let from_param = format!(
            "vertrek={},{}&vertrektype=adres",
            self.from.lat, self.from.lon
        );

        let to_param = format!(
            "aankomst={},{}&aankomsttype=adres",
            self.to.lat, self.to.lon
        );

        let time_param = match self.time {
            ArrivalDeparture::ArriveAt(time) => {
                format!(
                    "type=aankomst&tijd={}T{}",
                    self.date.format("%Y-%m-%d"),
                    time.format("%H:%M")
                )
            }
            ArrivalDeparture::DepartAt(time) => {
                format!(
                    "type=vertrek&tijd={}T{}",
                    self.date.format("%Y-%m-%d"),
                    time.format("%H:%M")
                )
            }
        };

        let url = format!("https://www.ns.nl/reisplanner/#/?{from_param}&{to_param}&{time_param}&firstMileModality=PUBLIC_TRANSPORT&lastMileModality=PUBLIC_TRANSPORT&entireTripModality=OWN_CAR&disabledTransportModalities=");

        println!("{}", url);

        // 2. Execute the request
        let tab = browser.wait_for_initial_tab().unwrap();
        tab.navigate_to(url.as_str()).unwrap();

        // 3. Extract trip information
        let departure_time_str = tab
            .wait_for_element(".rio-jp-travel-option-active .rio-jp-travel-option-departure time")
            .unwrap()
            .get_inner_text()
            .unwrap();

        let arrival_time_str = tab
            .wait_for_element(".rio-jp-travel-option-active .rio-jp-travel-option-arrival time")
            .unwrap()
            .get_inner_text()
            .unwrap();

        let steps_elms = tab
            .wait_for_elements(
                ".rio-jp-trip-container-active rio-jp-trip-details > .ng-star-inserted",
            )
            .unwrap();

        // 4. Parse the trip information
        let departure_time = NaiveTime::parse_from_str(&departure_time_str, "%H:%M").unwrap();
        let arrival_time = NaiveTime::parse_from_str(&arrival_time_str, "%H:%M").unwrap();

        let mut steps: Vec<Step> = vec![];

        let mut depart_time: Option<NaiveTime> = None;
        let mut location: Option<String> = None;

        for (_, step_elm) in steps_elms.iter().enumerate().take(steps_elms.len() - 1) {
            if step_elm.find_element("rio-jp-stop").is_ok() {
                // The step is a stop (busstop, station, location)
                let step_departure_time_str = step_elm
                    .find_element(".rio-jp-departure-time > rio-jp-time")
                    .unwrap()
                    .get_inner_text()
                    .unwrap()
                    .replace('\n', " ");

                let step_departure_time =
                    NaiveTime::parse_from_str(&step_departure_time_str, "%H:%M").unwrap();

                let step_location_stop_str = step_elm
                    .find_element(".rio-jp-stop-label .rio-jp-stop-name")
                    .unwrap()
                    .get_inner_text()
                    .unwrap()
                    .replace('\n', " ");

                depart_time = Some(step_departure_time);
                location = Some(step_location_stop_str);
            } else if step_elm.find_element("rio-jp-leg").is_ok() {
                // The step is a leg (train ride, bus ride, walk)
                let step_summary_str = step_elm
                    .find_element(".rio-jp-summary")
                    .unwrap()
                    .get_inner_text()
                    .unwrap()
                    .replace('\n', " ");

                let summary = Some(step_summary_str);
                let travel_type = Some("unknown".to_owned()); // TODO: extact travel type

                steps.push(Step {
                    depart_time: depart_time.clone().expect("depart_time not set"),
                    location: location
                        .clone()
                        .expect("location not set")
                        .trim()
                        .to_owned(),
                    summary: summary.clone().expect("summary not set"),
                    travel_type: travel_type.clone().expect("travel_type not set"),
                });
            }
        }

        Route {
            from: self.from,
            to: self.to,
            date: self.date,
            depart_time: departure_time,
            arrive_time: arrival_time,
            travel_time: arrival_time.signed_duration_since(departure_time),
            steps,
        }
    }
}
