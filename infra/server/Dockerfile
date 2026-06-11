# ==================================================================================================================== #
# Build

FROM rust:1.95-slim AS builder

WORKDIR /app

# Copy all packages
COPY Cargo.toml Cargo.lock ./
COPY services ./services
COPY frontend ./frontend
COPY crates ./crates

COPY config ./config
COPY migrations ./migrations

# Build the server package
RUN cargo build --release --package breachlet-server

# ==================================================================================================================== #
# Runner

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY config/default.toml /etc/breachlet/default.toml

COPY --from=builder /app/target/release/breachlet-server /usr/local/bin/breachlet-server

RUN useradd -r -s /bin/false breachlet-server
USER breachlet-server

EXPOSE 3000

CMD ["breachlet-server"]

# ==================================================================================================================== #
