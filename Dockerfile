# Stage 1: Builder
FROM rust:1.85-bullseye as builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

LABEL org.opencontainers.image.source="https://github.com/neuland-ingolstadt/neuland.app-ical-service" \
      org.opencontainers.image.description="A Rust-based service that fetches event data from a GraphQL API and serves it as an iCalendar subscription feed." \
      org.opencontainers.image.licenses="MIT"

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=builder /app/target/release/neuland-app-ical-service /app/neuland-app-ical-service
EXPOSE 7077
CMD ["./neuland-app-ical-service"]