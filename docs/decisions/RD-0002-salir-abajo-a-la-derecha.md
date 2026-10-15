# RD-0002 — Salir abajo a la derecha

- **Status:** Accepted
- **Date:** 2026-10-15

## Requirement

Después de usar la ventana, la persona pidió mover Salir a la esquina inferior derecha, a la derecha de Analizar, y dejarlo siempre visible para poder cerrar la app.

## Previous Requirement

[REQ-0004](../requirements/baseline.md) pedía un botón Salir visible. En `c9e6f67` quedó arriba a la derecha, en la barra de título. Analizar no estaba junto a él: en el inicio estaba al centro, y «Analizar de nuevo» volvía a aparecer arriba o en el cierre.

## New Requirement

Salir permanece visible en todas las pantallas y en la hoja de confirmación. Está a la derecha de Analizar, en el pie de la ventana. Analizar dice «Analizar esta Mac» hasta el primer resultado y «Analizar de nuevo» después. Durante el análisis o la limpieza no se puede lanzar otro; Salir sigue disponible.

## Reason

La persona lo pidió para tener siempre la opción de salir, al lado del botón de analizar.

## Impact

- `index.html`, `src/main.js` y `src/styles.css`.
- [PLAN-0008](../plans/PLAN-0008-salir-junto-a-analizar.md).
- No cambia las reglas de `safety.rs` ni el comando `quit`.

## Related ADRs

[ADR-0001](ADR-0001-app-escritorio-tauri.md). El botón sigue cerrando el proceso de la app de escritorio.

## Related Tests

No hay una prueba automática de la posición. La comprobación es visual en la ventana de desarrollo.
