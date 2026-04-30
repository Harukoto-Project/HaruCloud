# コミットメッセージ規約

このドキュメントでは、本プロジェクトにおけるコミットメッセージの書き方を定義します。  
[Conventional Commits](https://www.conventionalcommits.org/ja/) の仕様に準拠しています。

---

## 基本フォーマット

```
<type>(<scope>): <subject>

[body]

[footer]
```

- **type**（必須）: 変更の種類
- **scope**（任意）: 変更の影響範囲（例: `auth`, `api`, `ui`）
- **subject**（必須）: 変更内容の簡潔な説明
- **body**（任意）: 変更の詳細・背景
- **footer**（任意）: 破壊的変更・Issue 参照など

---

## type 一覧

| type | 説明 |
|------|------|
| `feat` | 新機能の追加 |
| `fix` | バグ修正 |
| `docs` | ドキュメントのみの変更 |
| `style` | コードの動作に影響しないスタイル変更（空白、フォーマット等） |
| `refactor` | バグ修正・機能追加を伴わないコード変更 |
| `perf` | パフォーマンス改善 |
| `test` | テストの追加・修正 |
| `build` | ビルドシステムや外部依存に関する変更 |
| `ci` | CI/CD の設定変更 |
| `chore` | その他の雑務（リリーススクリプト更新等） |
| `revert` | 過去のコミットの取り消し |

---

## ルール

### subject

- 50文字以内を目安にする
- 命令形・現在形で書く（日本語可）
- 末尾にピリオドを付けない
- 変更内容が一目でわかる具体的な記述にする

### body

- 72文字ごとに改行する
- **何を** 変更したかではなく **なぜ** 変更したかを説明する
- subject との間に空行を入れる

### footer

- **破壊的変更**がある場合は `BREAKING CHANGE:` から始める
- Issue を閉じる場合は `Closes #123` の形式で記述する

---

## 例

### 通常のコミット

```
feat(auth): Discord OAuth2 ログインを追加
```

```
fix(api): レート制限超過時に 429 を返すよう修正

従来は 500 を返していたため、クライアント側で適切なリトライ処理が
行えていなかった。RFC 6585 に従い 429 Too Many Requests を返す。
```

```
docs: README にインストール手順を追記
```

### 破壊的変更を含むコミット

```
feat(config)!: 設定ファイルのキー名を変更

BREAKING CHANGE: `database.host` を `db.host` に変更した。
既存の設定ファイルは移行が必要。
```

### Issue を閉じるコミット

```
fix(bot): コマンドのタイムアウトエラーを修正

Closes #42
```

---

## 絵文字（Gitmoji）を使う場合（任意）

type の前にアイコンを付けることで視認性が上がります。強制ではありません。

| Emoji | type |
|-------|------|
| ✨ | `feat` |
| 🐛 | `fix` |
| 📝 | `docs` |
| 🎨 | `style` |
| ♻️ | `refactor` |
| ⚡️ | `perf` |
| ✅ | `test` |
| 🔧 | `chore` / `build` |
| 👷 | `ci` |
| ⏪ | `revert` |

例:
```
✨ feat(ui): ダークモードを実装
🐛 fix(ws): 接続切断時のメモリリークを修正
```

---

## コミットメッセージのチェック（任意）

[commitlint](https://commitlint.js.org/) を使うと、CI でフォーマットを自動チェックできます。

```bash
npm install --save-dev @commitlint/cli @commitlint/config-conventional
```

`commitlint.config.js`:
```js
module.exports = {
  extends: ['@commitlint/config-conventional'],
};
```

---

注意: 一定のファイルごとにし、変更をまとめてコミットしないでください

## 参考

- [Conventional Commits 1.0.0](https://www.conventionalcommits.org/ja/v1.0.0/)
- [Gitmoji](https://gitmoji.dev/)
- [commitlint](https://commitlint.js.org/)