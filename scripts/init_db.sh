#!/usr/bin/env bash
# print every command to the terminal
set -x
# -e exit when a command fails, -o pipefail sets the exit code of a pipeline 
set -eo pipefail

# check if all required tools are installed in the operating system
# check if psql is installed
if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed"
    exit 1
fi
# check if sqlx is installed
if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed"
    exit 1
fi

# check if custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
# check if custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# check if custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
# check if custom database port has been set, otherwise default to '5432'
DB_PORT="${DB_PORT:=5432}"
# check if custom database host has been set, otherwise default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"

# launch postgres using docker
docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000

# ping postgres until it is ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still not available"
    sleep 1
done

>&2 echo "Postgres is up and running ${DB_HOST}:${DB_PORT}"

# create database - the DATABASE_URL is a sqlx required environment variable
DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
