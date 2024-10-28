FROM rust:1.82

RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

ENTRYPOINT ["sleep", "infinity"]