# Pruebas

No hay identificadores `TEST-...`. Las pruebas que existen se nombran por su función. No se inventan resultados de interfaz.

## Automáticas

Se ejecutan en el host, no en Docker. Ver [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md).

```bash
cd src-tauri
cargo test
```

El 2026-10-15, `cargo test --lib` pasó 4 pruebas:

- `safety::tests::allows_cache_child_and_rejects_documents`
- `safety::tests::rejects_parent_escape_and_allows_installer`
- `safety::tests::apple_support_is_private_but_apple_cache_is_not`
- `apps::tests::matches_apps_and_cli_tools`

Cubren la lista blanca y el emparejamiento de nombres. No cubren un análisis real del disco ni el comando que mueve a la Papelera.

`cargo check` pasó después de añadir el comando `quit`.

## Manual

La persona usuaria abrió la app el 2026-10-15 y dijo que la ventana quedó muy buena. No hay captura ni guion de esa prueba en el repositorio.

Pendiente, sin afirmar que se haya hecho: contraste en claro y oscuro, `prefers-reduced-transparency`, y una limpieza real con restauración desde la Papelera.
