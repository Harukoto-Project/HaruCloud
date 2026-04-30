const { invoke } = window.__TAURI__.core;

const mappingBody = document.getElementById("mappingsBody");
const logOutput = document.getElementById("logOutput");

function appendLog(message) {
  const now = new Date().toLocaleTimeString();
  logOutput.textContent = `[${now}] ${message}\n${logOutput.textContent}`;
}

function createRow(folderId = "", localPath = "") {
  const tr = document.createElement("tr");
  tr.innerHTML = `
    <td><input type="text" class="folder-id" value="${folderId}" placeholder="docs" /></td>
    <td><input type="text" class="local-path" value="${localPath}" placeholder="D:\\\\Docs" /></td>
    <td><button class="remove-row">削除</button></td>
  `;
  tr.querySelector(".remove-row").addEventListener("click", () => tr.remove());
  mappingBody.appendChild(tr);
}

function readConfigFromForm() {
  const mappings = Array.from(mappingBody.querySelectorAll("tr")).map((tr) => ({
    folderId: tr.querySelector(".folder-id").value.trim(),
    localPath: tr.querySelector(".local-path").value.trim(),
  }));

  return {
    minio: {
      endpoint: document.getElementById("endpoint").value.trim(),
      region: document.getElementById("region").value.trim(),
      accessKey: document.getElementById("accessKey").value.trim(),
      secretKey: document.getElementById("secretKey").value.trim(),
      bucket: document.getElementById("bucket").value.trim(),
      secure: document.getElementById("secure").checked,
    },
    mappings,
  };
}

function writeConfigToForm(config) {
  const minio = config.minio || {};
  document.getElementById("endpoint").value = minio.endpoint || "";
  document.getElementById("region").value = minio.region || "us-east-1";
  document.getElementById("accessKey").value = minio.accessKey || "";
  document.getElementById("secretKey").value = minio.secretKey || "";
  document.getElementById("bucket").value = minio.bucket || "";
  document.getElementById("secure").checked = !!minio.secure;

  mappingBody.innerHTML = "";
  if (Array.isArray(config.mappings) && config.mappings.length > 0) {
    config.mappings.forEach((row) => createRow(row.folderId, row.localPath));
  } else {
    createRow();
  }
}

document.getElementById("addMapping").addEventListener("click", () => {
  createRow();
});

document.getElementById("saveConfig").addEventListener("click", async () => {
  try {
    await invoke("save_config", { config: readConfigFromForm() });
    appendLog("設定を保存しました");
  } catch (e) {
    appendLog(`保存失敗: ${e}`);
  }
});

document.getElementById("testConnection").addEventListener("click", async () => {
  try {
    const cfg = readConfigFromForm();
    const result = await invoke("test_minio_connection", { config: cfg.minio });
    appendLog(result);
  } catch (e) {
    appendLog(`接続テスト失敗: ${e}`);
  }
});

document.getElementById("manualSync").addEventListener("click", async () => {
  try {
    const cfg = readConfigFromForm();
    const result = await invoke("manual_sync", { config: cfg });
    appendLog(result.join("\n"));
  } catch (e) {
    appendLog(`手動同期失敗: ${e}`);
  }
});

async function bootstrap() {
  try {
    const cfg = await invoke("load_config");
    writeConfigToForm(cfg);
    appendLog("設定を読み込みました");
  } catch (e) {
    createRow();
    appendLog(`設定読み込み失敗: ${e}`);
  }
}

bootstrap();
