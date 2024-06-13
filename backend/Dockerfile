# ビルドイメージ
FROM rust:latest as builder

ENV LANG C.UTF-8

# 作業ディレクトリの設定
ENV APP_NAME pointbreak
ENV APP_ROOT /opt/app
WORKDIR $APP_ROOT

COPY Cargo.toml Cargo.lock ./
COPY src ./src

# リリースビルド
RUN cargo build --release

# 実行用イメージ
#FROM debian:buster-slim
FROM rust:latest

# ライブラリのインストール
RUN <<EOF
  apt-get update
  apt-get install -y libssl-dev ca-certificates
  rm -rf /var/lib/apt/lists/*
EOF

# アプリケーションのバイナリをコピー
COPY --from=builder /opt/app/target/release/pointbreak /usr/local/bin/pointbreak

EXPOSE 3000
CMD ["pointbreak"]
