use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use log::info;
mod graphql;
mod graphql_client;
mod ical_service;

#[get("/cl-events.ics")]
async fn calendar() -> impl Responder {
    match ical_service::generate_ical().await {
        Ok(ics) => HttpResponse::Ok().content_type("text/calendar").body(ics),
        Err(e) => {
            log::error!("Error generating calendar: {e:?}");
            HttpResponse::InternalServerError().body("Error generating calendar")
        }
    }
}

#[get("/neuland-events.ics")]
async fn neuland_calendar() -> impl Responder {
    match ical_service::generate_neuland_ical().await {
        Ok(ics) => HttpResponse::Ok()
            .content_type("text/calendar")
            .append_header((
                "Content-Disposition",
                "attachment; filename=\"neuland-events.ics\"",
            ))
            .body(ics),
        Err(e) => {
            log::error!("Error generating Neuland calendar: {e:?}");
            HttpResponse::InternalServerError().body("Error generating Neuland calendar")
        }
    }
}

#[get("/health")]
async fn health_check() -> impl Responder {
    let cl_result = ical_service::generate_ical().await;
    let neuland_result = ical_service::generate_neuland_ical().await;

    let cl_status = if cl_result.is_ok() { "ok" } else { "error" };
    let neuland_status = if neuland_result.is_ok() {
        "ok"
    } else {
        "error"
    };

    if cl_result.is_ok() && neuland_result.is_ok() {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "healthy",
            "services": {
                "cl_events": cl_status,
                "neuland_events": neuland_status
            }
        }))
    } else {
        HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "unhealthy",
            "services": {
                "cl_events": cl_status,
                "neuland_events": neuland_status
            }
        }))
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
            .service(health_check)
    })
    .bind("0.0.0.0:7077")?
    .run()
    .await
}
