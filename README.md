zero2prod based on the book "Zero to prod with Rust"
====================================================

# Dependencies

Web framework

    $ cargo add actix-we
    $ cargo add tokio --features macros,rt-multi-thread 

Configuration compatible with 12 factor app rules

    $ cargo add figment --features yaml,en
    $ cargo add clap --features derive
    $ cargo add serde --features derive
    $ cargo add serde-yam

Easier results with errors

    $ cargo add anyhow


Logger

    $ cargo add env_logger
    $ cargo add log

Testing with HTTP client supporting JSON serialization. Available in dev mode (tests)

    $ cargo add --dev reqwest --features json

SQL database integration. `sqlx` library provides sync and async queries, and compile time checking

    # only for postgres
    $ cargo install sqlx-cli --no-default-features --features native-tls,postgres

`sqlx` validates SQL queries at compile time, that's why it needs `DATABASE_URL` env variable defined.

Postgresql client (required to check if postgresql is ready to accept commands)

    $ sudo apt install -y Postgresql-client

uuid generator

    $ cargo add uuid --features v4

chrono for timestamps in current timezone

    $ cargo add chrono --features clock

# Configure the database

## Step by step

    $ export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter

### Add new migration

    $ sqlx migrate add create_subscriptions_table

### Apply all migrations

    $ sqlx migrate run

## Being a part of ./scripts/init_db.sh

Run database (docker based) and all migration in one step:

    $ ./scripts/init_db.sh

Run migration scripts and skip running already started database:

    $ SKIP_DOCKER=true ./scripts/init_db.sh

# How to run it?

Run with configuration file

    $ cargo run -- --cfg config/local_cfg.yaml

Run with configuration file and some attribute overwritten

    $ cargo run -- --cfg config/local_cfg.yaml --log_level debug --port 8090 --host 0.0.0.0
