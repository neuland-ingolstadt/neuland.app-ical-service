use crate::graphql_client::get_events;
use icalendar::EventLike;
use icalendar::{Calendar, Component, Event};

pub async fn generate_ical() -> Result<String, Box<dyn std::error::Error>> {
    let events = get_events().await?;
    let mut calendar = Calendar::new();

    for event in events {
        let ical_event = Event::new()
            .summary(&event.title)
            .starts(event.start)
            .ends(event.end)
            .description(event.description.as_deref().unwrap_or(""))
            .uid(&event.id)
            .location(event.location.as_deref().unwrap_or(""))
            .url(event.url.as_deref().unwrap_or(""))
            .done();

        calendar.push(ical_event);
    }

    Ok(calendar.to_string())
}
