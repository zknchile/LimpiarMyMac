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

## RUN-2026-11-16-01

- **Fecha:** 2026-11-16.
- **Ejecutor:** agente de la sesión que aplica [PLAN-0012](../plans/PLAN-0012-migrar-vista-a-gpui.md).
- **Revisión:** cambios locales sin commitear. Las aserciones de selección y de sitio se trasladaron a Rust. El JavaScript sigue. No se reutiliza la salida de `npm test` de octubre como evidencia de este traslado.
- **Entorno:** host macOS 27.2.1 arm64, no Docker. `rustc 1.99.0 (b940084d7 2026-09-28)`. Node `v22.17.0`. Xcode 27.2. Sin imagen ni servicio.
- **Alcance:** TEST-selection-001 a TEST-selection-003 y TEST-place-001 a TEST-place-003, más TEST-safety-001 a TEST-safety-006, TEST-apps-001 y TEST-size-001, porque `cargo test --lib` no los separa. Ningún caso omitido ni bloqueado.
- **Comandos:** `cargo test --lib` en `src-tauri`. `npm test` en la raíz, para comprobar que la copia JavaScript sigue pasando.
- **Resultado:** aprobada. Rust: 14 passed, 0 failed, 0 ignored. Node: 6 pass, 0 fail, 0 skipped.
- **No cubre:** el clic de Salir, un análisis del disco ni el movimiento a la Papelera. La ventana GPUI se abrió en esta misma sesión; eso no es un caso de este catálogo.

Salida de `cargo test --lib`:

```text
running 14 tests
test place::tests::another_category_does_not_inherit_list_offset ... ok
test place::tests::leaving_results_or_opening_confirm_does_not_reuse_place ... ok
test place::tests::same_category_keeps_list_and_focus ... ok
test apps::tests::matches_apps_and_cli_tools ... ok
test selection::tests::eighty_mb_on_a_gb_total_changes_the_text ... ok
test safety::tests::apple_support_is_private_but_apple_cache_is_not ... ok
test safety::tests::nested_apple_file_in_third_party_container_is_not_private ... ok
test selection::tests::empty_selection_does_not_inherit_total ... ok
test selection::tests::toggling_changes_selected_size ... ok
test safety::tests::allows_cache_child_and_rejects_documents ... ok
test safety::tests::rejects_parent_escape_and_allows_installer ... ok
test size::tests::path_size_ignores_container_metadata ... ok
test safety::tests::container_plan_skips_containermanager_metadata ... ok
test safety::tests::container_plan_trashes_real_files_and_unlinks_escaping_links ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

Salida de `npm test`:

```text
# tests 6
# pass 6
# fail 0
# skipped 0
ok 1 - la misma categoría conserva la grilla y el foco
ok 2 - otra categoría no hereda el desplazamiento de la lista
ok 3 - salir de resultados o abrir la confirmación no reutiliza el sitio
ok 4 - una selección vacía no hereda el total encontrado
ok 5 - marcar y desmarcar cambia el tamaño seleccionado
ok 6 - ochenta MB sobre un total en GB cambian el texto
```

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

## RUN-2026-11-16-02

- **Fecha:** 2026-11-16.
- **Ejecutor:** agente de la sesión que cierra [PLAN-0012](../plans/PLAN-0012-migrar-vista-a-gpui.md).
- **Revisión:** cambios locales sin commitear. Se quitó la vista WebKit y el arnés de clic. Las aserciones de Rust no cambiaron. No se reutiliza [RUN-2026-11-16-01](#run-2026-11-16-01): aquella ejecución todavía tenía la copia JavaScript.
- **Entorno:** host macOS 27.2.1 arm64, no Docker ([ADR-0003](../decisions/ADR-0003-vista-gpui.md)). `rustc 1.99.0 (b940084d7 2026-09-28)`. Xcode 27.2. `DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer`. Sin Node, imagen ni servicio.
- **Alcance:** TEST-safety-001 a TEST-safety-006, TEST-apps-001, TEST-size-001, TEST-selection-001 a TEST-selection-003 y TEST-place-001 a TEST-place-003. Ningún caso omitido ni bloqueado.
- **Comando:** `cargo test --lib` en `src-tauri`.
- **Resultado:** aprobada. 14 passed, 0 failed, 0 ignored.
- **No cubre:** el clic de la ventana, un análisis del disco ni el movimiento a la Papelera. Esas comprobaciones de [PLAN-0012](../plans/PLAN-0012-migrar-vista-a-gpui.md) no son casos de este catálogo.

Salida de `cargo test --lib`:

```text
running 14 tests
test place::tests::another_category_does_not_inherit_list_offset ... ok
test place::tests::leaving_results_or_opening_confirm_does_not_reuse_place ... ok
test place::tests::same_category_keeps_list_and_focus ... ok
test apps::tests::matches_apps_and_cli_tools ... ok
test safety::tests::apple_support_is_private_but_apple_cache_is_not ... ok
test safety::tests::allows_cache_child_and_rejects_documents ... ok
test safety::tests::nested_apple_file_in_third_party_container_is_not_private ... ok
test safety::tests::rejects_parent_escape_and_allows_installer ... ok
test selection::tests::eighty_mb_on_a_gb_total_changes_the_text ... ok
test selection::tests::empty_selection_does_not_inherit_total ... ok
test selection::tests::toggling_changes_selected_size ... ok
test size::tests::path_size_ignores_container_metadata ... ok
test safety::tests::container_plan_skips_containermanager_metadata ... ok
test safety::tests::container_plan_trashes_real_files_and_unlinks_escaping_links ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```
