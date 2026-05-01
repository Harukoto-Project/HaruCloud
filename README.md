# HaruCloud Sync

自宅の MinIO（S3 互換）を介して、複数 PC のフォルダを同期するためのデスクトップアプリです。Tauri 2 と Rust で実装しています。

**v1.0.0 より正式版**です。重要なデータはバックアップを推奨します。

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

成果物はワークスペース直下の `target/release/` および `target/release/bundle/`（環境によっては `src-tauri/target/...`）を参照してください。

## 設定

アプリ内で MinIO の endpoint・認証・バケット、および `folder_id` とローカルパスを設定します。設定は OS のアプリデータ領域に保存されます。

## ライセンス

[MIT License](LICENSE)（Copyright © 2023-2026 Harukoto Project）
