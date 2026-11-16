# ADR-0003 — Vista en GPUI, sin vidrio nativo

- **Status:** Accepted
- **Date:** 2026-11-16

## Context

La persona pidió una prueba: migrar el renderizado visual a GPUI y aceptó perder elementos visuales. [PLAN-0012](../plans/PLAN-0012-migrar-vista-a-gpui.md) fija el alcance. No pidió conservar Liquid Glass ni reescribir [REQ-0003](../requirements/baseline.md).

[ADR-0001](ADR-0001-app-escritorio-tauri.md) dejó la ventana en Tauri 2 con `window-vibrancy` `0.8.1`, porque `NSGlassEffectView` estaba disponible y la persona aceptó ese resultado. GPUI no incrusta ese HTML ni aplica ese material. Una migración no puede ser un cambio de motor sobre `src/main.js`.

El crate oficial `gpui` en crates.io no es la publicación usable. La republicación que compila el 2026-11-16 es `gpui-unofficial` `1.28.1`, biblioteca `gpui`, del commit de Zed `e8bc16acee5ae1937600d571d441c4b5d7b0d288`. No la mantiene Zed. El crate de plataforma se importa como `gpui_platform` y necesita la feature `font-kit`, que no es la de por defecto, para dibujar glifos.

## Decision

- La ventana se abre con `gpui_platform::application()` en el mismo proceso Rust. La versión queda fijada en `=1.28.1`, no en un rango.
- El análisis, la lista blanca y la Papelera no cambian de reglas. `clean` sigue aceptando solo identificadores del último informe.
- Esta ventana no aplica `NSGlassEffectView` y no cumple [REQ-0003](../requirements/baseline.md). Ese requisito no se reescribe.
- [ADR-0001](ADR-0001-app-escritorio-tauri.md) queda `Superseded by ADR-0003`. Sigue valiendo lo de aquel ADR que no era la ventana: Rust en el host, sin HTTP y sin Docker.

## Alternatives Considered

- Dejar Tauri. No cumple el pedido de esta prueba.
- El crate `gpui` oficial. La publicación usable no es esa.
- Depender del repositorio de Zed por git. Arrastra el workspace del editor. La republicación ya nombra el commit de origen.
- `gpui-kit`, GPUI-CE o un fork con shader de vidrio. Más piezas, y el vidrio pintado no es `NSGlassEffectView`. Esta prueba no lo necesita.
- Meter GPUI dentro del WebView de Tauri. Son dos ventanas. No es una migración.
- Marcar ya ADR-0001 como sustituido, antes de que la ventana abra. El código vigente todavía no lo justifica.

## Consequences

- El 2026-11-16 `cargo run` abrió la ventana, un análisis llegó a resultados y Cancelar no llamó a la limpieza. Salir cerró el proceso. Por eso este ADR pasa a `Accepted` y ADR-0001 queda sustituido.
- La republicación puede quedarse sin mantenedor. No se flota a otra versión dentro de esta prueba.
- Rust 1.77 no compila la dependencia. Hace falta 1.85 o posterior. El 2026-11-16 compiló con `rustc 1.99.0` y Xcode 27.2, después de descargar Metal Toolchain 27B47. El selector del sistema seguía en Command Line Tools; el build usó `DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer`.
- Sigue sin haber servicio HTTP ni Docker. Eso no se revierte con el cambio de ventana.

## Related Requirements

[REQ-0001](../requirements/baseline.md). No modifica [REQ-0003](../requirements/baseline.md).

## Related ADRs

[ADR-0001](ADR-0001-app-escritorio-tauri.md), [ADR-0002](ADR-0002-lista-blanca-y-papelera.md).
