FROM rustlang/rust:nightly

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install cargo-watch

RUN rustup component add rustfmt

RUN rustup component add clippy

WORKDIR /club-backend

EXPOSE 3001

VOLUME ["/usr/local/cargo"]
