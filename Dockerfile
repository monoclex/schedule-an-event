# NOTE:
# https://rocket.rs/v0.4/guide/quickstart/#quickstart
# rocket.rs requires a nightly version of rust
#
# NOTE:
# https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
# this dockerfile was mainly inspired from that URL.
# i'm not sure how good in terms of code quality it is, but it works so... /shrug

# build the code
FROM rustlang/rust:nightly as cargo-build

# to get rust code to run on alpine, we need "musl" stuff apparently
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

# install NPM
# https://github.com/nodejs/docker-node/blob/2f7f7ebce71a98dd8fe2182a0adf8f6db952ca80/14/buster/Dockerfile#L3
RUN groupadd --gid 1000 node \
  && useradd --uid 1000 --gid node --shell /bin/bash --create-home node

ENV NODE_VERSION 14.3.0

RUN ARCH= && dpkgArch="$(dpkg --print-architecture)" \
  && case "${dpkgArch##*-}" in \
    amd64) ARCH='x64';; \
    ppc64el) ARCH='ppc64le';; \
    s390x) ARCH='s390x';; \
    arm64) ARCH='arm64';; \
    armhf) ARCH='armv7l';; \
    i386) ARCH='x86';; \
    *) echo "unsupported architecture"; exit 1 ;; \
  esac \
  # gpg keys listed at https://github.com/nodejs/node#release-keys
  && set -ex \
  && for key in \
    94AE36675C464D64BAFA68DD7434390BDBE9B9C5 \
    FD3A5288F042B6850C66B31F09FE44734EB7990E \
    71DCFD284A79C3B38668286BC97EC7A07EDE3FC1 \
    DD8F2338BAE7501E3DD5AC78C273792F7D83545D \
    C4F0DFFF4E8C1A8236409D08E73BC641CC11F4C8 \
    B9AE9905FFD7803F25714661B63B535A4C206CA9 \
    77984A986EBC2AA786BC0F66B01FBB92821C587A \
    8FCCA13FEF1D0C2E91008E09770F7A9A5AE15600 \
    4ED778F539E3634C779C87C6D7062848A1AB005C \
    A48C2BEE680E841632CD4E44F07496B3EB3C1762 \
    B9E2F5981AA6E0CD28160D9FF13993A75599653C \
  ; do \
    gpg --batch --keyserver hkp://p80.pool.sks-keyservers.net:80 --recv-keys "$key" || \
    gpg --batch --keyserver hkp://ipv4.pool.sks-keyservers.net --recv-keys "$key" || \
    gpg --batch --keyserver hkp://pgp.mit.edu:80 --recv-keys "$key" ; \
  done \
  && curl -fsSLO --compressed "https://nodejs.org/dist/v$NODE_VERSION/node-v$NODE_VERSION-linux-$ARCH.tar.xz" \
  && curl -fsSLO --compressed "https://nodejs.org/dist/v$NODE_VERSION/SHASUMS256.txt.asc" \
  && gpg --batch --decrypt --output SHASUMS256.txt SHASUMS256.txt.asc \
  && grep " node-v$NODE_VERSION-linux-$ARCH.tar.xz\$" SHASUMS256.txt | sha256sum -c - \
  && tar -xJf "node-v$NODE_VERSION-linux-$ARCH.tar.xz" -C /usr/local --strip-components=1 --no-same-owner \
  && rm "node-v$NODE_VERSION-linux-$ARCH.tar.xz" SHASUMS256.txt.asc SHASUMS256.txt \
  && ln -s /usr/local/bin/node /usr/local/bin/nodejs \
  # smoke tests
  && node --version \
  && npm --version
# end install node

# install parcel bundler
RUN npm install -g parcel-bundler

# this next section is so that way the build/test pipeline with docker can be faster
# by caching rust dependencies
# bump https://github.com/rust-lang/cargo/issues/2644 for this feature to be built into cargo
#
# cagro-build-deps doesn't work for us, so we just do a "dry build" of sorts.

WORKDIR /src/schedule-an-event
COPY Cargo.toml .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo +nightly build --release && rm src/main.rs

COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# run the code
FROM alpine:latest
WORKDIR /app

# copy the executable made in build to the
COPY --from=cargo-build \
	/src/schedule-an-event/target/x86_64-unknown-linux-musl/release/schedule-an-event \
	/app/schedule-an-event

# copy Rocket.toml for configuration as well
COPY --from=cargo-build \
	/src/schedule-an-event/Rocket.toml \
	/app/Rocket.toml

ENTRYPOINT ["/app/schedule-an-event"]