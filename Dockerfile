FROM rust
RUN apt-get update -q && apt-get install -yq libsqlite3-dev libseccomp-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/sn0int
COPY . .
RUN cargo build --release --verbose
RUN strip target/release/sn0int

FROM debian
RUN apt-get update -q && apt-get install -yq libsqlite3-dev libseccomp-dev \
    && rm -rf /var/lib/apt/lists/*
COPY --from=0 /usr/src/wqa/target/release/wqa /usr/local/bin/wqa
VOLUME ["/data", "/cache"]
ENV XDG_DATA_HOME=/data \
    XDG_CACHE_HOME=/cache
ENTRYPOINT ["wqa"]
