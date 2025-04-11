use crate::graphql_client::{get_events, get_neuland_events};
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

pub async fn generate_neuland_ical() -> Result<String, Box<dyn std::error::Error>> {
    let events = get_neuland_events().await?;
    let mut calendar = Calendar::new();

    calendar.name("Neuland Events");
    calendar.description("Club Events by Neuland Ingolstadt e.V.");
    calendar.timezone("Europe/Berlin");

    for event in events {
        let mut ical_event = Event::new();
        ical_event.summary(&event.title);
        ical_event.starts(event.start);
        ical_event.ends(event.end);
        ical_event.uid(&event.id);

        if let Some(desc) = &event.description {
            ical_event.description(desc);
        }

        if let Some(loc) = &event.location {
            if !loc.is_empty() {
                ical_event.location(loc);
            }
        }

        if let Some(url) = &event.url {
            ical_event.url(url);
        }

        calendar.push(ical_event.done());
    }

    Ok(calendar.to_string())
}
