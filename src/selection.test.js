import assert from "node:assert/strict";
import test from "node:test";
import { formatBytes, selectionOf } from "./selection.js";

const GB = 1024 ** 3;
const MB = 1024 ** 2;

// TEST-selection-001
test("una selección vacía no hereda el total encontrado", () => {
  const items = [
    { id: "caches-1", bytes: 2 * GB },
    { id: "caches-2", bytes: 400 * MB },
  ];
  const result = selectionOf(items, new Set());
  assert.equal(result.count, 0);
  assert.equal(result.bytes, 0);
  assert.equal(result.total, 2 * GB + 400 * MB);
  assert.equal(formatBytes(result.bytes), "0 B");
});

// TEST-selection-002
test("marcar y desmarcar cambia el tamaño seleccionado", () => {
  const items = [
    { id: "caches-1", bytes: 2 * GB },
    { id: "caches-2", bytes: 80 * MB },
  ];
  const onlyLarge = selectionOf(items, new Set(["caches-1"]));
  const both = selectionOf(items, new Set(["caches-1", "caches-2"]));
  assert.equal(onlyLarge.bytes, 2 * GB);
  assert.equal(both.bytes, 2 * GB + 80 * MB);
  assert.notEqual(formatBytes(onlyLarge.bytes), formatBytes(both.bytes));
});

// TEST-selection-003
test("ochenta MB sobre un total en GB cambian el texto", () => {
  const before = formatBytes(15 * GB);
  const after = formatBytes(15 * GB - 80 * MB);
  assert.notEqual(before, after);
  assert.match(before, /GB$/);
  assert.match(after, /GB$/);
});
