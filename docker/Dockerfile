FROM lukemathwalker/cargo-chef:latest-rust-1.68.2 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

# Generate a recipe file for dependencies
FROM chef as planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build project dependencies
FROM chef as cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Application builder
FROM chef as builder
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
# instead of querying real database during the compile time, use SQLX offline mode instead
ENV SQLX_OFFLINE true
RUN cargo build --release

# Build runtime image
FROM gcr.io/distroless/cc-debian11

COPY --from=builder /app/target/release/zero2prod /app/zero2prod
WORKDIR /app
ENTRYPOINT ["./zero2prod"]


# # Generate a recipe file for dependencies
# FROM rust:1.68.2 AS planner

# WORKDIR /app
# RUN cargo install cargo-chef
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

# # build our dependencies
# FROM rust:1.68.2 AS cacher

# WORKDIR /app
# # Install system dependencies
# RUN apt update && apt install lld clang -y

# RUN cargo install cargo-chef
# COPY --from=planner /app/recipe.json recipe.json
# RUN cargo chef cook --release --recipe-path recipe.json

# FROM rust:1.68.2 AS builder

# WORKDIR /app

# # Install system dependencies
# RUN apt update && apt install lld clang -y

# COPY . .

# COPY --from=cacher /app/target target
# COPY --from=cacher /usr/local/cargo /usr/local/cargo

# # instead of querying real database during the compile time, use SQLX offline mode instead
# ENV SQLX_OFFLINE true
# RUN cargo build --release

# FROM gcr.io/distroless/cc-debian11

# COPY --from=builder /app/target/release/zero2prod /app/zero2prod
# WORKDIR /app

# ENTRYPOINT ["./zero2prod"]
