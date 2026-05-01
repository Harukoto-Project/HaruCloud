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

成果物はワークスペース直下の `target/release/` および `target/release/bundle/`（環境によっては `src-tauri/target/...`）を参照してください。

### アプリ更新（GitHub Releases）

`tauri.conf.json` の `plugins.updater.endpoints` は、ホストする更新用 JSON（例: GitHub Release 添付の `latest.json`）の URL です。オーナー／リポジトリ名は実際のものに合わせて編集してください。

リリースビルド時は **秘密鍵**が必要です（`.env` は読み込まれません）。例:

- `TAURI_SIGNING_PRIVATE_KEY_PATH` に `src-tauri/.tauri/updater.key` のパスを指定する  
  （または `TAURI_SIGNING_PRIVATE_KEY` に鍵の内容を渡す）

ビルドが最後まで終わると **`.sig`（署名）** が、署名対象ファイルの隣に生成されます。パスはログ末尾の `Finished … updater signatures at:` に表示されます（多くの場合 `target/release/bundle/` 配下の NSIS `.exe` や、アップデーター用 zip など）。

**`latest.json` は Tauri が自動では作りません。** 次の形式で JSON を自分で作成し、GitHub Release に添付します。`platforms.windows-x86_64.signature` には **`url` と同じバイナリ**用の `.sig` の全文を入れます。`…/releases/latest/download/latest.json` を使うときは、添付ファイル名を **`latest.json` と完全一致**にしてください（違うと 404 で「Could not fetch a valid release JSON」になります）。

```json
{
  "version": "0.1.0-beta.2",
  "notes": "",
  "pub_date": "2026-05-01T00:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "（.sig ファイルの中身）",
      "url": "https://github.com/OWNER/REPO/releases/download/v0.1.0-beta.2/HaruCloud%20Sync_0.1.0-beta.2_x64-setup.exe"
    }
  }
}
```

## 設定

アプリ内で MinIO の endpoint・認証・バケット、および `folder_id` とローカルパスを設定します。設定は OS のアプリデータ領域に保存されます。

## ライセンス

[MIT License](LICENSE)（Copyright © 2023-2026 Harukoto Project）
