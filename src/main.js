import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { formatBytes, selectionOf } from "./selection.js";

const view = document.querySelector("#view");
const titlebarActions = document.querySelector("#titlebar-actions");
const modalRoot = document.querySelector("#modal-root");

const state = {
  screen: "home",
  progress: null,
  report: null,
  selection: new Set(),
  active: "summary",
  query: "",
  error: "",
  confirm: false,
  cleanProgress: null,
  result: null,
};

const RISK = {
  safe: ["Seguro", "safe"],
  review: ["Revisar", "review"],
  permanent: ["Permanente", "permanent"],
};

listen("scan-progress", (event) => {
  state.progress = event.payload;
  if (state.screen === "scanning") render();
});

listen("clean-progress", (event) => {
  state.cleanProgress = event.payload;
  if (state.screen === "cleaning") render();
});

document.addEventListener("keydown", (event) => {
  if (event.key === "Escape" && state.confirm) {
    state.confirm = false;
    render();
  }
});

function itemsOf(report = state.report) {
  return report ? report.categories.flatMap((category) => category.items) : [];
}

function selectedItems() {
  return itemsOf().filter((item) => state.selection.has(item.id));
}

function selectedBytes() {
  return selectionOf(itemsOf(), state.selection).bytes;
}

function categorySelection(category) {
  return selectionOf(category.items, state.selection);
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function age(secs) {
  if (!secs) return "";
  const days = Math.floor(Date.now() / 1000 - secs) / 86400;
  if (days < 1) return "hoy";
  if (days < 2) return "ayer";
  if (days < 60) return `hace ${Math.floor(days)} días`;
  return new Date(secs * 1000).toLocaleDateString("es");
}

async function startScan() {
  state.screen = "scanning";
  state.error = "";
  state.progress = { stage: "Preparando", detail: "Esto no borra nada", stage_index: 0, stage_count: 7, found_bytes: 0 };
  state.confirm = false;
  render();
  try {
    const report = await invoke("scan");
    state.report = report;
    state.selection = new Set(itemsOf(report).filter((item) => item.default_selected).map((item) => item.id));
    state.active = "summary";
    state.query = "";
    state.screen = "results";
  } catch (error) {
    state.error = String(error);
    state.screen = "home";
  }
  render();
}

async function startClean() {
  const ids = [...state.selection];
  if (!ids.length) return;
  state.confirm = false;
  state.screen = "cleaning";
  state.cleanProgress = { name: "Preparando", done: 0, total: ids.length };
  render();
  try {
    state.result = await invoke("clean", { ids });
    state.screen = "done";
  } catch (error) {
    state.error = String(error);
    state.screen = "results";
  }
  render();
}

async function quit() {
  await invoke("quit");
}

function render() {
  const rescan = state.report && state.screen !== "home"
    ? `<button class="btn btn-quiet" id="rescan">Analizar de nuevo</button>`
    : "";
  titlebarActions.innerHTML = `${rescan}<button class="btn btn-quit" id="quit">Salir</button>`;
  document.querySelector("#rescan")?.addEventListener("click", startScan);
  document.querySelector("#quit")?.addEventListener("click", quit);

  if (state.screen === "home") view.innerHTML = homeHtml();
  else if (state.screen === "scanning") view.innerHTML = scanningHtml();
  else if (state.screen === "cleaning") view.innerHTML = cleaningHtml();
  else if (state.screen === "done") view.innerHTML = doneHtml();
  else view.innerHTML = resultsHtml();

  bind();
  renderModal();
}

function homeHtml() {
  return `
    <section class="home">
      <div class="orb" aria-hidden="true">
        <svg width="46" height="46" viewBox="0 0 46 46" fill="none">
          <path d="M23 6c6 7 11 12.2 11 19.2A11 11 0 1 1 12 25.2C12 18.2 17 13 23 6Z" stroke="currentColor" stroke-width="2.2"/>
          <path d="M18 28c1.2 2.4 3 3.6 5.2 3.6" stroke="currentColor" stroke-width="2.2" stroke-linecap="round"/>
        </svg>
      </div>
      <h1>Cristal</h1>
      <p class="lede">Encuentra cachés, restos de apps borradas y basura de los navegadores. Tú decides qué se mueve a la Papelera.</p>
      <button class="btn btn-primary" id="scan">Analizar esta Mac</button>
      <div class="chips">
        <span class="chip">Primero analiza</span>
        <span class="chip">Tú eliges</span>
        <span class="chip">Va a la Papelera</span>
      </div>
      ${state.error ? `<div class="banner">${escapeHtml(state.error)}</div>` : ""}
    </section>`;
}

function scanningHtml() {
  const progress = state.progress || {};
  return `
    <section class="scanning">
      <div class="ring" aria-hidden="true"></div>
      <p class="stage">${escapeHtml(progress.stage || "Analizando")}</p>
      <p class="detail">${escapeHtml(progress.detail || "")}</p>
      <p class="lede">${formatBytes(progress.found_bytes || 0)} encontrados · paso ${progress.stage_index || 1} de ${progress.stage_count || 7}</p>
      <p class="detail">Todavía no se borra nada.</p>
    </section>`;
}

function cleaningHtml() {
  const progress = state.cleanProgress || {};
  return `
    <section class="scanning">
      <div class="ring" aria-hidden="true"></div>
      <p class="stage">Moviendo a la Papelera</p>
      <p class="detail">${escapeHtml(progress.name || "")}</p>
      <p class="lede">${progress.done || 0} de ${progress.total || 0}</p>
    </section>`;
}

function doneHtml() {
  const result = state.result || { moved: 0, bytes: 0, failures: [], emptied_trash: false, notes: [] };
  const failures = result.failures || [];
  const notes = result.notes || [];
  return `
    <section class="done">
      <div class="orb" aria-hidden="true">
        <svg width="36" height="36" viewBox="0 0 36 36" fill="none">
          <path d="M8 18.5 15 25.5 28 11" stroke="currentColor" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </div>
      <h1>Listo</h1>
      <p class="lede">${result.moved} elementos · ${formatBytes(result.bytes)}. ${result.emptied_trash ? "La Papelera se vació." : "Puedes restaurarlos desde la Papelera."}</p>
      ${notes.map((note) => `<p class="done-note">${escapeHtml(note)}</p>`).join("")}
      ${failures.length ? `<div class="banner">${failures.length} no se pudieron mover. ${escapeHtml(failures[0].path || "")} ${escapeHtml(failures[0].message)}</div>` : ""}
      <div class="chips">
        <button class="btn btn-primary" id="scan">Analizar de nuevo</button>
        <button class="btn" id="open-trash">Abrir Papelera</button>
      </div>
    </section>`;
}

function resultsHtml() {
  const report = state.report;
  const categories = report.categories;
  const active = state.active === "summary" ? null : categories.find((category) => category.id === state.active);
  const shown = active ? filterItems(active.items) : [];
  return `
    <section class="workspace">
      <aside class="sidebar">
        ${navButton("summary", "Resumen", formatBytes(selectedBytes()))}
        ${categories.map((category) => {
          const picked = categorySelection(category);
          return navButton(category.id, category.title, `${formatBytes(picked.bytes)} de ${formatBytes(picked.total)}`);
        }).join("")}
      </aside>
      <div class="pane">
        ${state.error ? `<div class="banner">${escapeHtml(state.error)}</div>` : ""}
        ${active ? categoryHtml(active, shown) : summaryHtml(report)}
        <footer class="footer">
          <div class="meta"><b>${formatBytes(selectedBytes())}</b><br>${selectedItems().length} seleccionados</div>
          <button class="btn btn-primary" id="open-confirm" ${selectedItems().length ? "" : "disabled"}>Revisar y limpiar</button>
        </footer>
      </div>
    </section>`;
}

function summaryHtml(report) {
  const selected = selectedBytes();
  const total = itemsOf(report).reduce((sum, item) => sum + item.bytes, 0);
  return `
    <div class="summary-hero">
      <p class="sub">Selección segura, lista para revisar</p>
      <p class="big">${formatBytes(selected).replace(" ", "&nbsp;")}</p>
      <ul class="notes">
        ${report.notes.map((note) => `<li>${escapeHtml(note)}</li>`).join("")}
        <li>${formatBytes(total)} encontrados en total · ${report.apps_found} apps instaladas · ${(report.elapsed_ms / 1000).toFixed(0)} s</li>
      </ul>
    </div>
    <div class="list">
      ${report.categories.map((category) => {
        const picked = categorySelection(category);
        return `<button class="row cat-row" data-open="${category.id}">
          <div class="grow">
            <div class="name">${escapeHtml(category.title)}</div>
            <div class="sub">${picked.count} de ${category.items.length} seleccionados</div>
          </div>
          <div class="size">${formatBytes(picked.bytes)}</div>
        </button>`;
      }).join("")}
    </div>`;
}

function categoryHtml(category, items) {
  const all = items.length > 0 && items.every((item) => state.selection.has(item.id));
  const some = items.some((item) => state.selection.has(item.id));
  const picked = categorySelection(category);
  return `
    <div class="pane-head">
      <div>
        <h2>${escapeHtml(category.title)}</h2>
        <p>${escapeHtml(category.description)}</p>
        <p class="picked">${picked.count} seleccionados · ${formatBytes(picked.bytes)}</p>
      </div>
      <div class="toolbar">
        <input class="search" id="search" placeholder="Buscar" value="${escapeHtml(state.query)}" />
        <label class="chip"><input class="check ${some && !all ? "mixed" : ""}" id="toggle-category" type="checkbox" ${all ? "checked" : ""} /> Esta lista</label>
      </div>
    </div>
    <div class="list">
      ${items.length ? items.map(itemHtml).join("") : `<div class="empty">Nada coincide con la búsqueda.</div>`}
    </div>`;
}

function itemHtml(item) {
  const [label, kind] = RISK[item.risk] || RISK.review;
  const extra = item.delete_paths.length > 1 ? ` · ${item.delete_paths.length} carpetas` : "";
  const when = age(item.modified_secs);
  const reveal = item.delete_paths[0]
    ? `<button class="linkish" data-reveal="${escapeHtml(item.delete_paths[0])}">Ver</button>`
    : "";
  return `
    <div class="row">
      <input class="check" type="checkbox" data-id="${item.id}" ${state.selection.has(item.id) ? "checked" : ""} />
      <div class="grow">
        <div class="name">${escapeHtml(item.name)}</div>
        <div class="sub">${escapeHtml(item.reason)}</div>
        <div class="path">${escapeHtml(item.display_path)}${extra}${when ? ` · ${when}` : ""} ${reveal}</div>
      </div>
      <span class="pill ${kind}">${label}</span>
      <div class="size">${formatBytes(item.bytes)}</div>
    </div>`;
}

function navButton(id, title, meta) {
  return `<button class="nav-btn ${state.active === id ? "active" : ""}" data-open="${id}"><strong>${escapeHtml(title)}</strong><small>${escapeHtml(meta)}</small></button>`;
}

function filterItems(items) {
  const query = state.query.trim().toLowerCase();
  if (!query) return items;
  return items.filter((item) =>
    `${item.name} ${item.display_path} ${item.reason}`.toLowerCase().includes(query)
  );
}

function bind() {
  document.querySelector("#scan")?.addEventListener("click", startScan);
  document.querySelector("#open-confirm")?.addEventListener("click", () => {
    state.confirm = true;
    render();
  });
  document.querySelector("#open-trash")?.addEventListener("click", () => invoke("open_trash"));
  document.querySelectorAll("[data-open]").forEach((button) => {
    button.addEventListener("click", () => {
      state.active = button.dataset.open;
      state.query = "";
      render();
    });
  });
  document.querySelectorAll("[data-id]").forEach((box) => {
    box.addEventListener("change", () => {
      if (box.checked) state.selection.add(box.dataset.id);
      else state.selection.delete(box.dataset.id);
      render();
    });
  });
  document.querySelectorAll("[data-reveal]").forEach((button) => {
    button.addEventListener("click", (event) => {
      event.preventDefault();
      invoke("reveal", { path: button.dataset.reveal }).catch((error) => {
        state.error = String(error);
        render();
      });
    });
  });
  const search = document.querySelector("#search");
  search?.addEventListener("input", () => {
    state.query = search.value;
    const pane = document.querySelector(".list");
    const category = state.report.categories.find((item) => item.id === state.active);
    if (pane && category) pane.innerHTML = filterItems(category.items).map(itemHtml).join("") || `<div class="empty">Nada coincide con la búsqueda.</div>`;
    bindChecksOnly();
  });
  document.querySelector("#toggle-category")?.addEventListener("change", (event) => {
    const category = state.report.categories.find((item) => item.id === state.active);
    for (const item of filterItems(category.items)) {
      if (event.target.checked) state.selection.add(item.id);
      else state.selection.delete(item.id);
    }
    render();
  });
}

function bindChecksOnly() {
  document.querySelectorAll("[data-id]").forEach((box) => {
    box.addEventListener("change", () => {
      if (box.checked) state.selection.add(box.dataset.id);
      else state.selection.delete(box.dataset.id);
      render();
    });
  });
  document.querySelectorAll("[data-reveal]").forEach((button) => {
    button.addEventListener("click", (event) => {
      event.preventDefault();
      invoke("reveal", { path: button.dataset.reveal });
    });
  });
}

function renderModal() {
  if (!state.confirm) {
    modalRoot.innerHTML = "";
    return;
  }
  const picked = selectedItems();
  const byCategory = state.report.categories
    .map((category) => {
      const items = category.items.filter((item) => state.selection.has(item.id));
      const bytes = items.reduce((sum, item) => sum + item.bytes, 0);
      return { title: category.title, count: items.length, bytes };
    })
    .filter((category) => category.count);
  const largest = [...picked].sort((a, b) => b.bytes - a.bytes).slice(0, 5);
  const hasReview = picked.some((item) => item.risk === "review");
  const emptiesTrash = picked.some((item) => item.kind === "empty_trash");
  modalRoot.innerHTML = `
    <div class="modal-back" id="modal-back">
      <div class="sheet" role="dialog" aria-modal="true" aria-labelledby="confirm-title">
        <h3 id="confirm-title">${emptiesTrash ? "Esto incluye vaciar la Papelera" : "Mover a la Papelera"}</h3>
        <p>${picked.length} elementos · ${formatBytes(selectedBytes())}. ${emptiesTrash ? "Vaciar la Papelera no se puede deshacer." : "Después puedes restaurarlos desde la Papelera."}</p>
        <ul>
          ${byCategory.map((category) => `<li><span>${escapeHtml(category.title)} · ${category.count}</span><b>${formatBytes(category.bytes)}</b></li>`).join("")}
          ${largest.map((item) => `<li><span>${escapeHtml(item.name)}</span><b>${formatBytes(item.bytes)}</b></li>`).join("")}
        </ul>
        ${hasReview ? `<div class="warn">Incluiste elementos marcados como Revisar. Usa Ver si no estás seguro.</div>` : ""}
        <div class="actions">
          <button class="btn" id="cancel-clean">Cancelar</button>
          <button class="btn ${emptiesTrash ? "btn-danger" : "btn-primary"}" id="confirm-clean">${emptiesTrash ? "Limpiar y vaciar" : "Mover a la Papelera"}</button>
        </div>
      </div>
    </div>`;
  document.querySelector("#cancel-clean").addEventListener("click", () => {
    state.confirm = false;
    render();
  });
  document.querySelector("#confirm-clean").addEventListener("click", startClean);
  document.querySelector("#modal-back").addEventListener("click", (event) => {
    if (event.target.id === "modal-back") {
      state.confirm = false;
      render();
    }
  });
}

render();
