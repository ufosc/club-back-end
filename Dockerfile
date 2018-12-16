FROM rustlang/rust:nightly

WORKDIR /club-backend

COPY . .

RUN cargo build

RUN cargo install diesel_cli --no-default-features --features postgres && \
		diesel setup

EXPOSE 3001

CMD cargo run
