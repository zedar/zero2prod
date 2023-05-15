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

Tracing - this is an alternative to the logging crate which allows to record structured events with additional information.

    $ cargo add tracing --features log

Testing with HTTP client supporting JSON serialization. Available in dev mode (tests)

    $ cargo add --dev reqwest --features json

SQL database integration. `sqlx` library provides sync and async queries, and compile time checking.

    # only for postgres
    $ cargo install sqlx-cli --no-default-features --features native-tls,postgres

`sqlx` validates SQL queries at compile time, that's why it needs `DATABASE_URL` env variable defined. The alternative is the `offline` mode.

    $ cargo install sqlx-cli --no-default-features --features native-tls,postgres,offline

In order to make the `offline` mode working we need to use the `sqlx prepare` command that generates query metadata to support `offline` compile-time verification.

    $ cargo sqlx prepare -- --lib

Postgresql client (required to check if postgresql is ready to accept commands)

    $ sudo apt install -y Postgresql-client

uuid generator

    $ cargo add uuid --features v4

chrono for timestamps in current timezone

    $ cargo add chrono --features clock

# Cleaning unsued Dependencies

Install cargo tools

    $ cargo install cargo-udeps

Run dependency analyzer (required +nightly)

    $ cargo +nightly udeps

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

# Build with docker

Build docker image

    $ docker build -t zero2prod -f ./docker/Dockerfile

Change configuration to support locally running Postgres database.
Use `host: host.docker.internal` instead of `localhost`. The option `--network=host` does not work with Mac desktop.

Run application

    $ docker run -it -p 8080:8080 -v {path_to_folder_with_configs}:/config zero2prod --cfg /config/local_docker_cfg.yaml

Example request:

```
$ curl -v -XPOST -H "content-type: application/json" -d '{"email":"bob@0xlab.xyz", "name": "Bob"}' http://localhost:8080/subscriptions
```
