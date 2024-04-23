# FROM lukemathwalker/cargo-chef:latest-rust-alpine as chef
# WORKDIR /app
#
# FROM chef AS planner
# COPY ./Cargo.toml ./Cargo.lock ./
# COPY ./src ./src
# RUN cargo chef prepare
#
# FROM chef AS builder
# COPY --from=planner /app/recipe.json .
# RUN cargo chef cook --release
# COPY . .
# RUN cargo build --release
# RUN mv ./target/release/pantheon-server ./app
#
# FROM scratch AS runtime
# WORKDIR /app
# COPY --from=builder /app/app /usr/local/bin/
# ENTRYPOINT ["/usr/local/bin/app"]

# 1. This tells docker to use the Rust official image
FROM rust:1.77

# 2. Copy the files in your machine to the Docker image
WORKDIR /app
COPY ./ ./

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/pantheon-server", "--database-url", "postgresql://postgres:postgres@postgres:5432/postgres", "--cache-url", "redis://redis@host.docker.internal"]
