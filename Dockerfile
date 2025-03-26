# Stage 1: Builder
FROM rust:1.85-bullseye as builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=builder /app/target/release/neuland-app-ical-service /app/neuland-app-ical-service
EXPOSE 7077
CMD ["./neuland-app-ical-service"]
