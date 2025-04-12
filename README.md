# CL Events iCal Subscription Service

A Rust-based microservice that fetches event data from the [neuland.app-backend](https://github.com/neuland-ingolstadt/neuland.app-backend) GraphQL API and serves it as an iCalendar subscription feed (`.ics` format) over HTTP.

It is designed to serve the campus events of the University of Applied Sciences Ingolstadt (THI) to common calendar applications, which makes it easier for students to stay up-to-date with the latest events.

> [!TIP]
> Check it out by subscribing to the calendar feeds at:
> - Campus Life Events: https://ical.neuland.app/cl-events.ics
> - Neuland Events: https://ical.neuland.app/neuland-events.ics

## Features

- **GraphQL Integration:** Uses GraphQL queries defined in [`src/gql/queries.graphql`](src/gql/queries.graphql) to retrieve events.
- **Event Transformation:** Converts raw GraphQL event data into an internal event representation (see [`src/graphql_client.rs`](src/graphql_client.rs)).
- **Recurring Events:** Parses RRULE data to display recurring Neuland events for the next 6 months.
- **iCalendar Generation:** Builds valid iCal feeds from the event data (implemented in [`src/ical_service.rs`](src/ical_service.rs)).
- **HTTP Server:** Exposes the generated calendars on endpoints (`/cl-events.ics` and `/neuland-events.ics`) using Actix-Web.

## Prerequisites

- Rust 1.85 or later
- Cargo
- Docker (optional, for containerized deployments)

## Running Locally

1. **Clone the Repository:**

   ```sh
   git clone https://github.com/yourusername/neuland.app-ical-service.git
   cd neuland.app-ical-service
   ```

2. **Build and Run:**

   Use Cargo to build and run the service:

   ```sh
   cargo run
   ```

3. **Accessing the Service:**

   Visit [http://localhost:7077/cl-events.ics](http://localhost:7077/cl-events.ics) or [http://localhost:7077/neuland-events.ics](http://localhost:7077/neuland-events.ics) in your browser or use a tool like cURL:

   ```sh
   curl http://localhost:7077/cl-events.ics
   curl http://localhost:7077/neuland-events.ics
   ```

> [!NOTE]  
> Common calender applications do not support subscription feeds from local URLs. You can use a service like [ngrok](https://ngrok.com/) to expose your local server to the internet.

## Contributing

Contributions are welcome! Please follow the standard Git workflow:

1. Fork the repository.
2. Create a feature branch.
3. Commit your changes with clear descriptions.
4. Submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
