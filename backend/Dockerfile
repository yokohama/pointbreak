# development
FROM rust:latest as development
WORKDIR /app
RUN cargo install cargo-watch
COPY . .
CMD ["cargo", "watch", "-w", "src", "-i", "target/*", "-x", "run"]

# build stage
FROM development as build-stage
RUN cargo build --release

# production
FROM rust:slim as production
# libpq をインストール
RUN apt-get update && \
    apt-get install -y libpq5 && \
    rm -rf /var/lib/apt/lists/*
COPY --from=build-stage /app/target/release/pointbreak /usr/local/bin/
EXPOSE 3000
CMD ["/usr/local/bin/pointbreak"]
