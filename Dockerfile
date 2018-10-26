FROM rustlang/rust:nightly

WORKDIR /club-backend

COPY . .

ENV DATABASE_URL=postgres://osc:stallman@db/osc \
    ROCKET_ENV=development

RUN cargo build

EXPOSE 3001

CMD cargo run
