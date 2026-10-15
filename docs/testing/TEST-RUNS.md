# Registro de ejecuciones

La definición de cada caso está en [TEST-CASES.md](TEST-CASES.md). Una fila de un plan cerrado no se reemite aquí: aquellas notas no conservan la salida del runner ni la revisión de cada ficha.

## RUN-2026-10-15-01

- **Fecha:** 2026-10-15.
- **Ejecutor:** agente de la sesión que aplicó [PLAN-0009](../plans/PLAN-0009-documentacion-de-pruebas.md).
- **Revisión:** commit `4688269` más cambios locales sin commitear. En los tests solo se añadieron comentarios `TEST-...`. No cambiaron aserciones. El resto del diff es documental.
- **Entorno:** host macOS 27.0.1 arm64, no Docker ([ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md)). `rustc 1.99.0 (b940084d7 2026-09-28)`. Node `v22.17.0`. Sin imagen ni servicio.
- **Alcance:** TEST-safety-001 a TEST-safety-006, TEST-apps-001, TEST-size-001, TEST-selection-001 a TEST-selection-003. Ningún caso omitido ni bloqueado.
- **Comandos:** `cargo test --lib` en `src-tauri`. `npm test` en la raíz (`node --test src/selection.test.js`).
- **Resultado:** aprobada. Rust: 8 passed, 0 failed, 0 ignored. Node: 3 pass, 0 fail, 0 skipped.
- **No cubre:** análisis del disco, movimiento a la Papelera, botón Salir, contraste ni `prefers-reduced-transparency`. Esta ejecución es anterior a TEST-place-001.

Salida de `cargo test --lib`:

```text
running 8 tests
test safety::tests::apple_support_is_private_but_apple_cache_is_not ... ok
test apps::tests::matches_apps_and_cli_tools ... ok
test safety::tests::nested_apple_file_in_third_party_container_is_not_private ... ok
test safety::tests::allows_cache_child_and_rejects_documents ... ok
test safety::tests::rejects_parent_escape_and_allows_installer ... ok
test size::tests::path_size_ignores_container_metadata ... ok
test safety::tests::container_plan_skips_containermanager_metadata ... ok
test safety::tests::container_plan_trashes_real_files_and_unlinks_escaping_links ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

Salida de `npm test`:

```text
# tests 3
# pass 3
# fail 0
# skipped 0
ok 1 - una selección vacía no hereda el total encontrado
ok 2 - marcar y desmarcar cambia el tamaño seleccionado
ok 3 - ochenta MB sobre un total en GB cambian el texto
```

## RUN-2026-10-15-02

- **Fecha:** 2026-10-15.
- **Ejecutor:** agente de la sesión de [PLAN-0010](../plans/PLAN-0010-grilla-sin-saltar.md).
- **Revisión:** commit `37f0ffe` más cambios locales sin commitear. El cambio de comportamiento está en `src/place.js` y `src/main.js`. No se tocó Rust.
- **Entorno:** host macOS 27.0.1 arm64, no Docker ([ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md)). Node `v22.17.0`. Sin imagen ni servicio.
- **Alcance:** TEST-place-001, TEST-place-002 y TEST-place-003. En el mismo comando también pasaron TEST-selection-001 a TEST-selection-003. Ningún caso omitido ni bloqueado.
- **Comando:** `npm test` en la raíz (`node --test src/*.test.js`).
- **Resultado:** aprobada. Node: 6 pass, 0 fail, 0 skipped.
- **No cubre:** el clic real en la ventana. La prueba decide qué desplazamiento se conserva; no mide el scroll del WebView.

Salida relevante:

```text
ok 1 - la misma categoría conserva la grilla y el foco
ok 2 - otra categoría no hereda el desplazamiento de la lista
ok 3 - salir de resultados o abrir la confirmación no reutiliza el sitio
# tests 6
# pass 6
# fail 0
# skipped 0
```
