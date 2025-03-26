use crate::graphql_client::get_events;
use chrono::{NaiveDateTime, TimeZone, Utc};
use icalendar::EventLike;
use icalendar::{Calendar, Component, Event};
use log::error;

pub async fn generate_ical() -> Result<String, Box<dyn std::error::Error>> {
    let events = get_events().await?;
    let mut calendar = Calendar::new();

    for event in events {
        let start = parse_datetime(&event.start).map_err(|e| {
            error!("Failed to parse start datetime '{}': {}", event.start, e);
            "Failed to parse start datetime"
        })?;

        let end = parse_datetime(&event.end).map_err(|e| {
            error!("Failed to parse end datetime '{}': {}", event.end, e);
            "Failed to parse end datetime"
        })?;

        let ical_event = Event::new()
            .summary(&event.title)
            .starts(Utc.from_utc_datetime(&start))
            .ends(Utc.from_utc_datetime(&end))
            .description(event.description.as_deref().unwrap_or(""))
            .uid(&event.id)
            .location(event.location.as_deref().unwrap_or(""))
            .done();
        calendar.push(ical_event);
    }

    Ok(calendar.to_string())
}

fn parse_datetime(datetime_str: &str) -> Result<NaiveDateTime, Box<dyn std::error::Error>> {
    let formats = [
        "%Y-%m-%dT%H:%M:%S%.fZ",
        "%Y-%m-%d %H:%M:%S UTC",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S",
    ];
    for fmt in formats.iter() {
        if let Ok(dt) = NaiveDateTime::parse_from_str(datetime_str, fmt) {
            return Ok(dt);
        }
    }
    Err(format!("Couldn't parse datetime string: {}", datetime_str).into())
}
