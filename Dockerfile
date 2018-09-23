FROM buildpack-deps:stretch

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    DATABASE_URL='sqlite'


WORKDIR /usr/src/backend

COPY . .

# Install rust nightly
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && rustup default nightly && rustup update nightly

# Put the cargo build in a seperate run command so that we don't have to install rustup again if
# the build fails
RUN cargo build

EXPOSE 8000

CMD cargo run
