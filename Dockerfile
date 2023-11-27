# Multi-stage Docker build
# Build stage
FROM rust as planner
WORKDIR /app
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

# caching stage
FROM rust as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
COPY --from=planner /app/crates crates
RUN cargo chef cook --release --recipe-path recipe.json

# build stage
FROM rust:bookworm as builder
WORKDIR /app
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

# runtime!
# We can use slim , as it has the incompatible version of open ssl 
# and get an auth error
FROM  rust:bookworm as runtime
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev
COPY --from=builder /app/target/release/khalani-solver .
COPY config config
USER 1000
CMD ./khalani-solver --private-key $PRIVATE_KEY --config-file $CONFIG_FILE