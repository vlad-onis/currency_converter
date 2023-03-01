FROM rust:1.66 as build

RUN apt-get update && apt-get install -y make autoconf automake libtool unzip

# create a new empty shell project
WORKDIR /app

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./.env ./.env

# copy your source tree
COPY ./src ./src

# # build for release
RUN cargo build --release

# our final base
FROM rust:1.66-slim-buster

# copy the build artifact from the build stage
COPY --from=build /app/target/release/currency_converter .
COPY --from=build /app/.env .

# set the startup command to run your binary
CMD ["./currency_converter"]