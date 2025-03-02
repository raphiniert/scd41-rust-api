FROM rust:1.85 AS base

WORKDIR /scd41-api

COPY src src
COPY .env .env
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
RUN cargo build --release

FROM base AS local
CMD ["cargo", "run", "--release"]
