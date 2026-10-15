# RD-0001 — Salir, README y GitHub

- **Status:** Accepted
- **Date:** 2026-10-15

## Requirement

Después de probar la ventana, la persona usuaria pidió tres cambios: un botón Salir, un README del proyecto y subir el código a GitHub.

## Previous Requirement

[REQ-0001](../requirements/baseline.md), [REQ-0002](../requirements/baseline.md) y [REQ-0003](../requirements/baseline.md) pedían la app de análisis y limpieza. No pedían cierre explícito, resumen escrito ni publicación. La ventana ya tenía los controles nativos de macOS.

## New Requirement

- [REQ-0004](../requirements/baseline.md): botón Salir visible, que cierra la app.
- [REQ-0005](../requirements/baseline.md): README con qué limpia, cómo se usa y cómo se ejecuta.
- [REQ-0006](../requirements/baseline.md): repositorio GitHub de la cuenta `zknchile`.

El comportamiento de limpieza no cambia.

## Reason

La persona lo pidió después de ver la app en marcha y decir que quedaba muy buena, salvo esos detalles.

## Impact

- `src/main.js`, `src/styles.css` y el comando Rust `quit`.
- `README.md`.
- Commit `c9e6f67` en https://github.com/zknchile/LimpiarMyMac, rama `main`, repositorio público.
- No cambia las reglas de `safety.rs`.

## Related ADRs

[ADR-0001](ADR-0001-app-escritorio-tauri.md). El botón cierra el proceso de la app de escritorio; no añade una API.

## Related Tests

No hay una prueba automática del botón. La comprobación de compilación fue `cargo check` del crate después de añadir `quit`. La publicación se comprobó con `gh repo view`. Pendiente un identificador `TEST-...` de interfaz; no se inventa.
