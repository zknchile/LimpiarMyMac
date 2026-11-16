# PLAN-0014 — Diagrama de la ventana GPUI

- **Fecha:** 2026-11-16
- **Estado:** completado
- **Solicitud:** actualizar `README.md` con la nueva arquitectura.
- **Tareas:** [TASKS-0014](TASKS-0014-diagrama-de-la-ventana-gpui.md)

Este documento no cambia la app. El diagrama vigente todavía separa «ventana», «mismo proceso» y «Rust», como si fueran capas distintas.

## Objetivo, alcance y criterios de aceptación

La sección «Cómo está armada» describe el binario que abre `cargo run`: un proceso GPUI, sin HTML ni comandos de Tauri.

Criterios observables:

- El diagrama cabe en un solo proceso. No hay una caja «Rust» aparte de la ventana.
- Aparecen `main.rs`, la ventana, el informe, el análisis fuera del hilo de pintado y la limpieza.
- `safety` está en el camino del análisis y en el de la limpieza.
- macOS aporta la carpeta del usuario, la Papelera, Finder y Metal. No se dibuja `NSGlassEffectView`.
- No se dibuja HTTP, WebKit, `invoke` ni un servicio aparte.
- La tabla nombra los mismos archivos que el diagrama.

Fuera de alcance: cambiar módulos, la versión, la release, las pruebas o `safety.rs`. No se empuja salvo que se pida.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `README.md`, `src-tauri/src/main.rs`, `lib.rs`, `view.rs`, `scan.rs`, `clean.rs` y [ADR-0003](../decisions/ADR-0003-vista-gpui.md).

`main.rs` abre la ventana con `gpui_platform`. `view.rs` llama a `scan::run` y a `clean::execute` con `background_spawn`. El informe queda en `Session`. Ver y la Papelera usan `open`. No hay puente `invoke`.

No hay ADR nuevo: el diagrama describe [ADR-0003](../decisions/ADR-0003-vista-gpui.md). No hay RD: no cambia un requisito.

Supuesto: rehacer el diagrama Mermaid basta. No hay pregunta abierta.

## Diseño, responsabilidades, contratos y alternativas

- Un recuadro, el binario. `main.rs` abre. `view.rs` pinta y decide. `selection.rs` cuenta. `place.rs` conserva el sitio. `lib.rs` guarda el informe y habla con Finder.
- `scan` y `clean` no corren en el hilo de pintado. Los dos pasan por `safety`. `model` es la forma del informe.
- macOS: carpeta del usuario, Papelera, Finder y Metal. El vidrio nativo no está.
- Alternativa rechazada: dejar las tres cajas actuales. Siguen pareciendo la app de Tauri partida en interfaz y backend.
- Alternativa rechazada: un diagrama de secuencia del clic. La solicitud pide la arquitectura, no un flujo.

## Pasos y entregables

1. Guardar este plan y su listado — [TASK-0014-01](TASKS-0014-diagrama-de-la-ventana-gpui.md#task-0014-01--dejar-el-plan).
2. Redibujar la sección — [TASK-0014-02](TASKS-0014-diagrama-de-la-ventana-gpui.md#task-0014-02--redibujar-la-sección). Depende del paso 1.

## Riesgos, seguridad y bloqueos

- Riesgo: atribuir Liquid Glass a Metal. Mitigación: el texto dice que la ventana no usa `NSGlassEffectView`.
- No se toca `safety.rs`. No hay secretos. No hay bloqueo.

## Validación y evidencia esperada

Los enlaces locales del README y del plan resuelven. `git diff --check` no reporta espacios conflictivos. No hay prueba nueva: el diagrama no ejecuta código.

## Entrega, documentación, decisiones y recuperación

Changelog, `AGENTS.md` y el párrafo de planes del README. Sin ADR, RD ni release. Recuperación: revertir el cambio. No se sube en este plan.

## Seguimiento y resultados

Completado el 2026-11-16. «Cómo está armada» dibuja un solo binario. El análisis y la limpieza salen del hilo de pintado y pasan por `safety`. Metal está en macOS, sin `NSGlassEffectView`. No se cambió código ni se empujó.
