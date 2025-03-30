use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use log::info;
mod graphql;
mod graphql_client;
mod ical_service;

#[get("/cl-events.ics")]
async fn calendar() -> impl Responder {
    match ical_service::generate_ical().await {
        Ok(ics) => HttpResponse::Ok().content_type("text/calendar").body(ics),
        Err(e) => {
            log::error!("Error generating calendar: {:?}", e);
            HttpResponse::InternalServerError().body("Error generating calendar")
        }
    }
}

#[get("/neuland.ics")]
async fn neuland_calendar() -> impl Responder {
    match ical_service::fetch_google_calendar().await {
        Ok(ics) => HttpResponse::Ok().content_type("text/calendar").body(ics),
        Err(e) => {
            log::error!("Error fetching Google calendar: {:?}", e);
            HttpResponse::InternalServerError().body("Error fetching calendar")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server at http://localhost:7077");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(calendar)
            .service(neuland_calendar)
    })
    .bind("0.0.0.0:7077")?
    .run()
    .await
}
