# 環境構築

## ローカルPC
```
$ cargo install sqlx-cli --no-default-features --features postgres
```

## 開発環境構築
```
$ git clone git@github.com:yokohama/pointbreak.git
$ cd pointbreak
$ docker compose up
```
## DBマイグレーション
```
$ sqlx migrate add -r <やりたい事がわかりやすい名前>
```
backend/migrationsに、upとdownができるので、それぞれにSQLを書く。

```
# up
$ make migration-run

# down
$ make migration-revert
```

### seed
```
$ make seed
```

### test
```
$ make test
```
