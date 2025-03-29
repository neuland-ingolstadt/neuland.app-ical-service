# Stage 1: Nix builder
FROM nixos/nix:latest AS builder

# Copy the entire repository
WORKDIR /build
COPY . .

# Install dependencies needed for the build
RUN nix-channel --update && \
    nix-env -iA nixpkgs.git

# Build the application using Nix
RUN nix build .#default --print-build-logs

# Stage 2: Minimal runtime
FROM scratch

# Copy CA certificates for HTTPS requests
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

# Copy the built binary from the Nix store
COPY --from=builder /build/result/bin/neuland-app-ical-service /neuland-app-ical-service

# Expose the application port
EXPOSE 7077

# Run the application
ENTRYPOINT ["/neuland-app-ical-service"]