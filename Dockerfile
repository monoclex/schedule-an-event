# https://rocket.rs/v0.4/guide/quickstart/#quickstart
# rocket.rs requires a nightly version of rust

# build the code
FROM rustlang/rust:nightly as cargo-build

# to get rust code to run on alpine, we need "musl" stuff apparently
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /src/schedule-an-event
COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# run the code
FROM alpine:latest

# copy the executable made in build to the
COPY --from=cargo-build \
	/src/schedule-an-event/target/x86_64-unknown-linux-musl/release/schedule-an-event \
	/usr/local/bin/schedule-an-event

CMD ["schedule-an-event"]