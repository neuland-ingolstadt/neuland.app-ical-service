# CL Events iCal Subscription Service

A Rust-based microservice that fetches event data from the [neuland.app-backend](https://github.com/neuland-ingolstadt/neuland.app-backend) GraphQL API and serves it as an iCalendar subscription feed (`.ics` format) over HTTP.

It is designed to serve the campus events of the University of Applied Sciences Ingolstadt (THI) to common calendar applications, which makes it easier for students to stay up-to-date with the latest events.

## Features

- **GraphQL Integration:** Uses a GraphQL query defined in [`src/graphql.rs`](src/graphql.rs) to retrieve events.
- **Event Transformation:** Converts raw GraphQL event data into an internal event representation (see [`src/graphql_client.rs`](src/graphql_client.rs)).
- **iCalendar Generation:** Builds a valid iCal feed from the event data (implemented in [`src/ical_service.rs`](src/ical_service.rs)).
- **HTTP Server:** Exposes the generated calendar on an endpoint (`/calendar.ics`) using Actix-Web.

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

   Visit [http://localhost:7077/calendar.ics](http://localhost:7077/calendar.ics) in your browser or use a tool like cURL:

   ```sh
   curl http://localhost:7077/calendar.ics
   ```

> [!NOTE]  
> Common calender applications do not support subscription feeds from local URLs. You can use a service like [ngrok](https://ngrok.com/) to expose your local server to the internet.

- **Manual Testing:**  
  Run the service locally and access the `/calendar.ics` endpoint via a web browser or `curl`.

- **Automated Tests:**  
  You can add unit tests to key functions (e.g., date parsing in [`src/ical_service.rs`](src/ical_service.rs)) by using Rust’s built-in testing framework. Run tests with:

  ```sh
  cargo test
  ```

## Contributing

Contributions are welcome! Please follow the standard Git workflow:

1. Fork the repository.
2. Create a feature branch.
3. Commit your changes with clear descriptions.
4. Submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
