#=== development
FROM rust:latest as development
WORKDIR /app
RUN cargo install cargo-watch
# dockerの中でやるのめんどいから、ローカルにインストールする
# 以下環境構築時に、cargo installで、sqlx-cliを実行してください。
#RUN cargo install sqlx-cli --no-default-features --features postgres
COPY . .
CMD ["cargo", "watch", "-w", "src", "-i", "target/*", "-x", "run"]

#=== build
FROM development as build-stage
RUN cargo build --release

#=== production
FROM rust:slim as production

WORKDIR /app

RUN apt-get update && \
  apt-get install -y libpq-dev && \
  rm -rf /var/lib/apt/lists/* && \
  cargo install diesel_cli --no-default-features --features postgres

COPY --from=build-stage /app/target/release/pointbreak /usr/local/bin/

COPY . .
RUN chmod +x entrypoint.sh

EXPOSE 3000

ENTRYPOINT ["./entrypoint.sh"]
CMD ["/usr/local/bin/pointbreak"]
