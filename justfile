#!/usr/bin/env just --justfile

default:
    just --list

up *args:
    docker compose up -d {{args}}

down *args:
    docker compose down {{args}}

exec *args:
    docker compose exec app {{args}}

enter:
    docker compose exec app bash

logs *args:
    docker compose logs -f {{args}}

cargo *args:
    just exec cargo {{args}}

diesel *args:
    just exec diesel {{args}}

run:
    just exec cargo watch -x run
