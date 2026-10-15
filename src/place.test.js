import assert from "node:assert/strict";
import test from "node:test";
import { placeToKeep } from "./place.js";

const before = {
  screen: "results",
  active: "caches",
  list: 480,
  sidebar: 120,
  focusId: "caches-4",
};

// TEST-place-001
test("la misma categoría conserva la grilla y el foco", () => {
  const kept = placeToKeep(before, { screen: "results", active: "caches", confirm: false });
  assert.equal(kept.list, 480);
  assert.equal(kept.sidebar, 120);
  assert.equal(kept.focusId, "caches-4");
});

// TEST-place-002
test("otra categoría no hereda el desplazamiento de la lista", () => {
  const kept = placeToKeep(before, { screen: "results", active: "logs", confirm: false });
  assert.equal(kept.list, null);
  assert.equal(kept.sidebar, 120);
  assert.equal(kept.focusId, "");
});

// TEST-place-003
test("salir de resultados o abrir la confirmación no reutiliza el sitio", () => {
  const gone = placeToKeep(before, { screen: "cleaning", active: "caches", confirm: false });
  assert.equal(gone.list, null);
  assert.equal(gone.sidebar, null);
  assert.equal(gone.focusId, "");
  const sheet = placeToKeep(before, { screen: "results", active: "caches", confirm: true });
  assert.equal(sheet.list, 480);
  assert.equal(sheet.focusId, "");
});
