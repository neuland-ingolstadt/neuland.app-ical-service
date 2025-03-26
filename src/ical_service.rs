use crate::graphql_client::get_events;
use chrono::{DateTime, Utc};
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
            .starts(start) // using UTC directly
            .ends(end) // using UTC directly
            .description(event.description.as_deref().unwrap_or(""))
            .uid(&event.id)
            .location(event.location.as_deref().unwrap_or(""))
            .url(event.url.as_deref().unwrap_or(""))
            .done();

        calendar.push(ical_event);
    }

    Ok(calendar.to_string())
}

fn parse_datetime(datetime_str: &str) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
    // try RFC3339
    if let Ok(dt) = DateTime::parse_from_rfc3339(datetime_str) {
        return Ok(dt.with_timezone(&Utc));
    }
    // fallback: handle strings ending with " UTC"
    if let Some(stripped) = datetime_str.strip_suffix(" UTC") {
        let new_str = format!("{}Z", stripped.replace(" ", "T"));
        let dt = DateTime::parse_from_rfc3339(&new_str)?;
        return Ok(dt.with_timezone(&Utc));
    }
    Err(format!("Couldn't parse datetime string: {}", datetime_str).into())
}
