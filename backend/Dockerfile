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
FROM rust:latest-slim-stretch
COPY --from=build-stage /app/target/release/pointgreak .
EXPOSE 3000
CMD ["/usr/local/bin/pointbreak"]
