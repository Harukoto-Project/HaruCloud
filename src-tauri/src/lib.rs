use std::path::{Path, PathBuf};

use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use aws_sdk_s3::{
  config::Builder as S3ConfigBuilder,
  error::DisplayErrorContext,
  Client,
};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct MinioConfig {
  endpoint: String,
  access_key: String,
  secret_key: String,
  bucket: String,
  region: String,
  secure: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct FolderMapping {
  folder_id: String,
  local_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct AppConfig {
  minio: MinioConfig,
  mappings: Vec<FolderMapping>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SyncProgressPayload {
  done: u32,
  total: u32,
  folder_id: String,
  current_file: String,
}

fn config_file_path(app: &AppHandle) -> Result<PathBuf, String> {
  let dir = app
    .path()
    .app_config_dir()
    .map_err(|e| format!("設定ディレクトリを取得できませんでした: {e}"))?;
  std::fs::create_dir_all(&dir).map_err(|e| format!("設定ディレクトリを作成できませんでした: {e}"))?;
  Ok(dir.join("config.json"))
}

fn read_config_from_disk(app: &AppHandle) -> Result<AppConfig, String> {
  let path = config_file_path(app)?;
  if !path.exists() {
    return Ok(AppConfig::default());
  }

  let raw = std::fs::read_to_string(path).map_err(|e| format!("設定ファイル読み込みに失敗: {e}"))?;
  serde_json::from_str(&raw).map_err(|e| format!("設定ファイルの形式が不正です: {e}"))
}

fn validate_config(cfg: &AppConfig) -> Result<(), String> {
  if cfg.minio.endpoint.trim().is_empty() {
    return Err("MinIO endpoint を入力してください".to_string());
  }
  if cfg.minio.access_key.trim().is_empty() {
    return Err("MinIO access key を入力してください".to_string());
  }
  if cfg.minio.secret_key.trim().is_empty() {
    return Err("MinIO secret key を入力してください".to_string());
  }
  if cfg.minio.bucket.trim().is_empty() {
    return Err("MinIO bucket を入力してください".to_string());
  }
  if cfg.minio.region.trim().is_empty() {
    return Err("MinIO region を入力してください".to_string());
  }

  for mapping in &cfg.mappings {
    if mapping.folder_id.trim().is_empty() {
      return Err("folder_id が空の行があります".to_string());
    }
    if mapping.local_path.trim().is_empty() {
      return Err(format!("folder_id={} の local_path が空です", mapping.folder_id));
    }
  }
  Ok(())
}

fn minio_tls_protocol_hint(detail: &str) -> &'static str {
  if detail.contains("InvalidContentType")
    || detail.contains("corrupt message")
    || detail.contains("InvalidMessage")
  {
    " MinIO が平文 HTTP のときは TLS をオフにするか、エンドポイントを http:// で指定してください。"
  } else {
    ""
  }
}

fn minio_region_mismatch_hint(detail: &str) -> String {
  if !detail.contains("AuthorizationHeaderMalformed") || !detail.contains("region is wrong") {
    return String::new();
  }
  let needle = "expecting '";
  let Some(start) = detail.find(needle) else {
    return " フォームの「region」を MinIO のバケットリージョンと一致させてください。".to_string();
  };
  let tail = &detail[start + needle.len()..];
  let Some(end) = tail.find('\'') else {
    return " フォームの「region」を MinIO のバケットリージョンと一致させてください。".to_string();
  };
  let region = &tail[..end];
  format!(" フォームの「region」を「{region}」に変更して保存し、再試行してください。")
}

fn minio_error_hints(detail: &str) -> String {
  let mut out = String::new();
  out.push_str(minio_tls_protocol_hint(detail));
  out.push_str(&minio_region_mismatch_hint(detail));
  out
}

async fn build_s3_client(cfg: &MinioConfig) -> Result<Client, String> {
  let endpoint_input = cfg.endpoint.trim();
  let endpoint_url = if endpoint_input.starts_with("http://") || endpoint_input.starts_with("https://") {
    endpoint_input.to_string()
  } else if cfg.secure {
    format!("https://{endpoint_input}")
  } else {
    format!("http://{endpoint_input}")
  };

  let credentials = Credentials::new(
    cfg.access_key.trim(),
    cfg.secret_key.trim(),
    None,
    None,
    "harucloud-sync",
  );

  let shared_cfg = aws_config::defaults(BehaviorVersion::latest())
    .region(Region::new(cfg.region.trim().to_string()))
    .credentials_provider(credentials)
    .load()
    .await;

  let s3_cfg = S3ConfigBuilder::from(&shared_cfg)
    .endpoint_url(endpoint_url)
    .force_path_style(true)
    .build();

  Ok(Client::from_conf(s3_cfg))
}

#[tauri::command]
fn load_config(app: AppHandle) -> Result<AppConfig, String> {
  read_config_from_disk(&app)
}

#[tauri::command]
fn save_config(app: AppHandle, config: AppConfig) -> Result<(), String> {
  validate_config(&config)?;
  let path = config_file_path(&app)?;
  let data = serde_json::to_string_pretty(&config).map_err(|e| format!("設定のJSON変換に失敗: {e}"))?;
  std::fs::write(path, data).map_err(|e| format!("設定の保存に失敗: {e}"))?;
  Ok(())
}

#[tauri::command]
async fn test_minio_connection(config: MinioConfig) -> Result<String, String> {
  let client = build_s3_client(&config).await?;
  client
    .list_objects_v2()
    .bucket(config.bucket.trim())
    .max_keys(1)
    .send()
    .await
    .map_err(|e| {
      let d = format!("{}", DisplayErrorContext(&e));
      format!("MinIO接続テスト失敗: {}{}", d, minio_error_hints(&d))
    })?;

  Ok(format!("接続成功: bucket={}", config.bucket.trim()))
}

fn walk_files(base: &Path) -> Result<Vec<PathBuf>, String> {
  let mut files = Vec::new();
  let mut dirs = vec![base.to_path_buf()];

  while let Some(dir) = dirs.pop() {
    let entries = std::fs::read_dir(&dir).map_err(|e| format!("ディレクトリ読み込み失敗 {}: {e}", dir.display()))?;
    for entry in entries {
      let entry = entry.map_err(|e| format!("ディレクトリエントリ読み込み失敗: {e}"))?;
      let path = entry.path();
      if path.is_dir() {
        dirs.push(path);
      } else if path.is_file() {
        files.push(path);
      }
    }
  }

  Ok(files)
}

#[tauri::command]
async fn manual_sync(app: AppHandle, config: AppConfig) -> Result<Vec<String>, String> {
  validate_config(&config)?;
  let client = build_s3_client(&config.minio).await?;
  let mut logs = Vec::new();

  let mut work: Vec<(String, PathBuf, Vec<PathBuf>)> = Vec::new();
  let mut total_files = 0usize;

  for mapping in &config.mappings {
    let local_root = Path::new(mapping.local_path.trim());
    if !local_root.exists() {
      logs.push(format!(
        "[skip] folder_id={} local_path が存在しません: {}",
        mapping.folder_id,
        local_root.display()
      ));
      continue;
    }

    let files = walk_files(local_root)?;
    total_files += files.len();
    work.push((
      mapping.folder_id.trim().to_string(),
      local_root.to_path_buf(),
      files,
    ));
  }

  let app_handle = app.clone();
  let emit = move |done: u32, total: u32, folder_id: &str, current_file: &str| {
    let _ = app_handle.emit(
      "sync-progress",
      SyncProgressPayload {
        done,
        total,
        folder_id: folder_id.to_string(),
        current_file: current_file.to_string(),
      },
    );
  };

  emit(
    0,
    total_files as u32,
    "",
    if total_files == 0 {
      "同期するファイルがありません"
    } else {
      "アップロードを開始します"
    },
  );

  let mut done = 0u32;

  for (folder_id, local_root, files) in work {
    let mut uploaded = 0usize;

    for path in files {
      let rel = path
        .strip_prefix(&local_root)
        .map_err(|e| format!("相対パス計算失敗: {e}"))?;
      let key = format!(
        "folders/{}/{}",
        folder_id.trim(),
        rel.to_string_lossy().replace('\\', "/")
      );
      let body = tokio::fs::read(&path)
        .await
        .map_err(|e| format!("ファイル読み込み失敗 {}: {e}", path.display()))?;
      client
        .put_object()
        .bucket(config.minio.bucket.trim())
        .key(&key)
        .body(body.into())
        .send()
        .await
        .map_err(|e| {
          let d = format!("{}", DisplayErrorContext(&e));
          format!("アップロード失敗 key={key}: {}{}", d, minio_error_hints(&d))
        })?;
      uploaded += 1;
      done += 1;
      let fname = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
      emit(done, total_files as u32, folder_id.as_str(), &fname);
    }

    logs.push(format!(
      "[ok] folder_id={} {}件をMinIOへアップロード",
      folder_id, uploaded
    ));
  }

  if total_files > 0 {
    emit(
      total_files as u32,
      total_files as u32,
      "",
      "完了",
    );
  }

  Ok(logs)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_log::Builder::default().level(log::LevelFilter::Info).build())
    .invoke_handler(tauri::generate_handler![
      load_config,
      save_config,
      test_minio_connection,
      manual_sync
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
