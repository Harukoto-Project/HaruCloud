# HaruCloud Sync

自宅の MinIO（S3 互換）を介して、複数 PC のフォルダを同期するためのデスクトップアプリです。Tauri 2 と Rust で実装しています。

現リリースラインは **ベータ（先行公開）** です。本番データへの適用はバックアップを取ったうえでご検討ください。

## 必要環境

- Rust（stable、`rustup` 推奨）
- Windows: WebView2 ランタイム
- `cargo tauri` CLI（`cargo install tauri-cli`）

## 開発

リポジトリルートで:

```sh
cargo tauri dev
```

## ビルド（exe）

```sh
cargo tauri build
```

成果物は `src-tauri/target/release/` および `src-tauri/target/release/bundle/` を参照してください。

### アプリ更新（GitHub Releases）

`tauri.conf.json` の `plugins.updater.endpoints` は GitHub 上の `latest.json` を指すようになっています。実際のオーナー／リポジトリ名に合わせて編集してください。

リリースビルド時は **秘密鍵**が必要です（`.env` は読み込まれません）。例:

- `TAURI_SIGNING_PRIVATE_KEY_PATH` に `src-tauri/.tauri/updater.key` のパスを指定する  
  （または `TAURI_SIGNING_PRIVATE_KEY` に鍵の内容を渡す）

ビルド後、`bundle` 出力に `latest.json` と `.sig` が生成されます。GitHub Release の資産に **インストーラー本体・署名・latest.json** をアップロードし、かつ各リリースで `latest.json` が「そのリリースのファイル」を指すようにしてください。

## 設定

アプリ内で MinIO の endpoint・認証・バケット、および `folder_id` とローカルパスを設定します。設定は OS のアプリデータ領域に保存されます。

## ライセンス

[MIT License](LICENSE)（Copyright © 2023-2026 Harukoto Project）
