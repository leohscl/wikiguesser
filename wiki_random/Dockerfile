FROM rust:1.62-slim-bullseye

RUN apt-get --yes update && apt-get --yes install postgresql postgresql-contrib
RUN apt-get --yes update && apt-get --yes install libpq-dev
RUN apt-get --yes update && apt-get --yes install pkg-config libssl-dev
RUN cargo install diesel_cli --no-default-features --features postgres
