const mappingBody = document.getElementById("mappingsBody");
const logOutput = document.getElementById("logOutput");

function appendLog(message) {
  const now = new Date().toLocaleTimeString();
  logOutput.textContent = `[${now}] ${message}\n${logOutput.textContent}`;
}

function invoke(cmd, args) {
  const fn = window.__TAURI__?.core?.invoke;
  if (typeof fn !== "function") {
    return Promise.reject(new Error("Tauri APIが未初期化です"));
  }
  return fn.call(window.__TAURI__.core, cmd, args);
}

function listen(event, handler) {
  const fn = window.__TAURI__?.event?.listen;
  if (typeof fn !== "function") {
    return Promise.reject(new Error("Tauri event APIが未初期化です"));
  }
  return fn.call(window.__TAURI__.event, event, handler);
}

function setSyncProgress(payload) {
  const wrap = document.getElementById("syncProgressWrap");
  const label = document.getElementById("syncProgressLabel");
  const frac = document.getElementById("syncProgressFraction");
  const fill = document.getElementById("syncProgressFill");
  const track = document.getElementById("syncProgressTrack");
  const total = Number(payload.total) || 0;
  const done = Number(payload.done) || 0;
  const pct = total > 0 ? Math.min(100, Math.round((done / total) * 100)) : done > 0 ? 100 : 0;
  fill.style.width = `${pct}%`;
  track.setAttribute("aria-valuenow", String(pct));
  frac.textContent = total > 0 ? `${done} / ${total}` : "";
  const fid = payload.folderId || "";
  const cf = payload.currentFile || "";
  if (fid && cf) {
    label.textContent = `${fid}: ${cf}`;
  } else {
    label.textContent = cf || "…";
  }
  wrap.hidden = false;
}

function resetSyncProgress() {
  const wrap = document.getElementById("syncProgressWrap");
  const label = document.getElementById("syncProgressLabel");
  const frac = document.getElementById("syncProgressFraction");
  const fill = document.getElementById("syncProgressFill");
  const track = document.getElementById("syncProgressTrack");
  wrap.hidden = true;
  label.textContent = "";
  frac.textContent = "";
  fill.style.width = "0%";
  track.setAttribute("aria-valuenow", "0");
}

function createRow(folderId = "", localPath = "") {
  const tr = document.createElement("tr");
  tr.innerHTML = `
    <td><input type="text" class="folder-id" value="${folderId}" placeholder="docs" /></td>
    <td><input type="text" class="local-path" value="${localPath}" placeholder="D:\\\\Docs" /></td>
    <td><button type="button" class="remove-row">削除</button></td>
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

async function bootstrap() {
  try {
    const cfg = await invoke("load_config");
    writeConfigToForm(cfg);
    appendLog("設定を読み込みました");
  } catch (e) {
    if (mappingBody.querySelectorAll("tr").length === 0) {
      createRow();
    }
    appendLog(`設定読み込み失敗: ${e}`);
  }
}

window.addEventListener("load", () => {
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

  document.getElementById("syncRun").addEventListener("click", async () => {
    const btn = document.getElementById("syncRun");
    if (btn.disabled) {
      return;
    }
    btn.disabled = true;
    let unlisten = null;
    try {
      resetSyncProgress();
      document.getElementById("syncProgressWrap").hidden = false;
      unlisten = await listen("sync-progress", (e) => {
        setSyncProgress(e.payload);
      });
      const cfg = readConfigFromForm();
      const result = await invoke("manual_sync", { config: cfg });
      appendLog(result.join("\n"));
    } catch (e) {
      appendLog(`同期失敗: ${e}`);
    } finally {
      if (typeof unlisten === "function") {
        unlisten();
      }
      btn.disabled = false;
      setTimeout(() => resetSyncProgress(), 500);
    }
  });

  void bootstrap();
});
