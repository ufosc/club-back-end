# Select build image
FROM rustlang/rust:nightly as dependencies-builder

# Create a new empty shell project
RUN USER=root cargo new --bin club-back-end
WORKDIR /club-back-end

# Copy over your manifests
COPY Cargo.toml Cargo.lock Rocket.toml ./

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Our project
FROM dependencies-builder as builder

# Copy your source tree
COPY ./src ./src

# Build enviroment info
ENV DATABASE_URL=postgres://osc:stallman@db/osc \
	ROCKET_ENV=production

# Build for release
RUN rm ./target/release/deps/club_back-end*
RUN cargo build --release

# Set the startup command to run your binary
CMD ["/club-back-end/target/release/club-back-end"]
