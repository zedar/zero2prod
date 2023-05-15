FROM rust:1.68.2 AS builder

WORKDIR /app

# Install system dependencies
RUN apt update && apt install lld clang -y

COPY . .

# instead of querying real database during the compile time, use SQLX offline mode instead
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM gcr.io/distroless/cc-debian11

COPY --from=builder /app/target/release/zero2prod /app/zero2prod
WORKDIR /app

ENTRYPOINT ["./zero2prod"]
