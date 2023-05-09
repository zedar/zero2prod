#!/usr/bin/env bash
# print every command to the terminal
set -x
# -e exit when a command fails, -o pipefail sets the exit code of a pipeline 
set -eo pipefail

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
