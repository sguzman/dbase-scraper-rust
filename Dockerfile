FROM alpine:latest
RUN mkdir app
WORKDIR ./app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN apk add --no-cache libgcc openssl-dev \
    && apk add --no-cache --virtual .build-rust rust cargo \
    && cargo build --release --jobs 4 --verbose

ADD . src
RUN cargo build --package dbase-scraper-rust --bin dbase-scraper-rust --verbose --jobs 2 --all-features --release . \
    && mv ./target/release/dbase-scraper-rust /root \
    && rm -rf /root/.cargo/ /root/.rustup target/  \
    && apk del --purge .build-rust

WORKDIR /root

ENTRYPOINT ["./dbase-scraper-rust"]