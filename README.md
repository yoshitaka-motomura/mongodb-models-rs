# mongodb-models-rs

## Description
rustのmongodbドライバーである[mongodb](https://crates.io/crates/mongodb)を使ってActiveRecordのようなモデルを作成したチュートリアル
テスト用ファイルを読み込んでください。

## Usage

```
docker-compose up -d
cargo run

// down
docker-compose down

// remove volume
docker-compose down -v
```

## Directory Structure
```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── docker-compose.yml // setup mongodb
├── resouces
│   └── airports.json // テストデータ
└── src
    ├── config.rs // mongodbnの接続設定
    ├── db.rs // mongodbの接続
    ├── main.rs // メイン
    └── models
        ├── airport.rs // Airportモデル
        └── mod.rs
```