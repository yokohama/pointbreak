#=== development
FROM rust:latest as development
WORKDIR /app

RUN cargo install cargo-watch && \
  cargo install sqlx-cli --no-default-features --features postgres

COPY . .

ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["cargo", "watch", "-w", "src", "-i", "target/*", "-x", "run --bin pointbreak"]

#=== build
FROM development as build-stage
RUN cargo build --release

#=== production
FROM rust:slim as production
WORKDIR /app

RUN apt-get update && \
  apt-get install -y libpq-dev && \
  rm -rf /var/lib/apt/lists/* && \
  cargo install sqlx-cli --no-default-features --features postgres

COPY --from=build-stage /app/target/release/pointbreak /usr/local/bin/

COPY . .
COPY entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

EXPOSE 3000

ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["/usr/local/bin/pointbreak"]
