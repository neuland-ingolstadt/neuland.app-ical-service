use cached::proc_macro::cached;
use chrono::{DateTime, Duration, Utc};
use graphql_client::{GraphQLQuery, Response};
use rrule::RRuleSet;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::graphql::{fetch_events, neuland_events, FetchEvents, NeulandEvents};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
}

#[cached(time = 600, result = true)]
pub async fn get_events() -> Result<Vec<Event>, reqwest::Error> {
    let client = reqwest::Client::new();
    let variables = fetch_events::Variables {};
    let request_body = FetchEvents::build_query(variables);

    let response = client
        .post("https://api.dev.neuland.app/graphql")
        .json(&request_body)
        .send()
        .await?;

    let response_data: Response<fetch_events::ResponseData> = response.json().await?;

    let events = response_data.data.map_or_else(Vec::new, |data| {
        data.cl_events
            .into_iter()
            .enumerate()
            .map(|(index, e)| {
                let title = e
                    .titles
                    .de
                    .or(e.titles.en)
                    .unwrap_or_else(|| format!("Event {}", index));
                let start = e.start_date_time.expect("Missing start date");
                let end = e.end_date_time.unwrap_or(start + Duration::hours(2));
                let description = Some(format!(
                    "Organizer: {}\nWebsite: {}\nInstagram: {}",
                    e.host.name,
                    e.host.website.map(|u| u.to_string()).unwrap_or_default(),
                    e.host.instagram.map(|u| u.to_string()).unwrap_or_default()
                ));
                Event {
                    id: e.id,
                    title,
                    start,
                    end,
                    description,
                    location: e.location,
                    url: e.event_url,
                }
            })
            .collect()
    });

    Ok(events)
}

#[cached(time = 600, result = true)]
pub async fn get_neuland_events() -> Result<Vec<Event>, reqwest::Error> {
    let client = reqwest::Client::new();
    let variables = neuland_events::Variables {};
    let request_body = NeulandEvents::build_query(variables);

    let response = client
        .post("https://api.dev.neuland.app/graphql")
        .json(&request_body)
        .send()
        .await?;

    let response_data: Response<neuland_events::ResponseData> = response.json().await?;

    let mut events = Vec::new();

    if let Some(data) = response_data.data {
        for e in data.neuland_events {
            let id = e.id;
            let title = e
                .title
                .de
                .or(e.title.en)
                .unwrap_or_else(|| "Neuland Event".to_string());
            let description = e.description.map(|d| d.de.or(d.en).unwrap_or_default());
            let location = e.location;

            let start = match e.start_time {
                Some(start_time) => start_time,
                None => {
                    log::debug!("Skipping event {} due to missing start time", id);
                    continue;
                }
            };

            let end = e.end_time.unwrap_or(start + Duration::hours(2));

            events.push(Event {
                id: id.clone(),
                title: title.clone(),
                start,
                end,
                description: description.clone(),
                location: location.clone(),
                url: None,
            });

            if let Some(rrule_str) = e.rrule {
                if let Ok(rrule_set) = RRuleSet::from_str(&rrule_str) {
                    // Generate occurrences for the next 6 months
                    let now = Utc::now();
                    let six_months_later = now + Duration::days(180);

                    for (i, occurrence) in rrule_set
                        .into_iter()
                        .take_while(|dt| dt.with_timezone(&Utc) <= six_months_later)
                        .enumerate()
                    {
                        // Skip the first occurrence if it's the same as the original event
                        if i == 0
                            && (occurrence.with_timezone(&Utc) - start).num_seconds().abs() < 60
                        {
                            continue;
                        }

                        let duration = end - start;

                        let occurrence_start = occurrence.with_timezone(&Utc);
                        let occurrence_end = occurrence_start + duration;

                        events.push(Event {
                            id: format!("{}-occurrence-{}", id, i),
                            title: title.clone(),
                            start: occurrence_start,
                            end: occurrence_end,
                            description: description.clone(),
                            location: location.clone(),
                            url: None,
                        });
                    }
                }
            }
        }
    }

    Ok(events)
}
