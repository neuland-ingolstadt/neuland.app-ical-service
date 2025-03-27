use cached::proc_macro::cached;
use chrono::{DateTime, Duration, Utc};
use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Serialize};

use crate::graphql::{fetch_events, FetchEvents};

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
        .post("https://api.neuland.app/graphql")
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
