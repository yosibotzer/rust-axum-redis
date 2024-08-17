FROM rust:1.80-bookworm AS build

# create a new empty shell project
RUN USER=root cargo new --bin rust-axum-redis
WORKDIR /rust-axum-redis

# copy over your manifests
COPY ./Cargo.toml ./Cargo.toml

COPY ./src ./src
COPY ./config ./config

RUN cargo build --release

# our final base
FROM debian:bookworm-slim AS runtime

# copy the build artifact from the build stage
COPY --from=build /rust-axum-redis/target/release/rust-axum-redis .
COPY --from=build /rust-axum-redis/config ./config

EXPOSE 3000

# set the startup command to run your binary
CMD ["./rust-axum-redis"]