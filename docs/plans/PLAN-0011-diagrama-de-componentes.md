# PLAN-0011 — Diagrama de componentes en el README

- **Fecha:** 2026-10-15
- **Estado:** completado
- **Solicitud:** actualizar `README.md` con el diagrama de componentes y subirlo a GitHub.
- **Tareas:** [TASKS-0011](TASKS-0011-diagrama-de-componentes.md)

## Objetivo, alcance y criterios de aceptación

El README muestra cómo está partida la app, con un diagrama que coincide con el código y con [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md). No cambia el comportamiento.

Criterios observables:

- El diagrama está en la sección «Cómo está armada» de `README.md`.
- Separa interfaz, comandos de Tauri, módulos Rust y lo que aporta macOS.
- No dibuja HTTP, SolidJS, Docker ni un servicio aparte.
- Liquid Glass queda en la ventana nativa, no en el CSS.
- `safety` aparece en el camino del análisis y en el de la limpieza.
- Los enlaces locales del plan y del README resuelven.

Fuera de alcance: cambiar módulos, la versión, la release o las pruebas. No se redibuja la app.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `README.md`, `src/main.js`, `src/selection.js`, `src/place.js`, `src-tauri/src/lib.rs`, `scan.rs`, `clean.rs`, `safety.rs`, `apps.rs`, `size.rs`, `model.rs` y [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md).

Hoy el README solo lista tres carpetas. La ventana llama `scan`, `clean`, `reveal`, `open_trash` y `quit` con `invoke`. El progreso llega por `scan-progress` y `clean-progress`. No hay puerto de red.

No hay ADR nuevo: el diagrama describe la decisión ya aceptada. No hay RD: no cambia un requisito.

Supuesto: un diagrama Mermaid en el README basta; GitHub lo dibuja. No hay pregunta abierta.

## Diseño, responsabilidades, contratos y alternativas

- Interfaz: `main.js` pinta y llama. `selection.js` cuenta el tamaño. `place.js` decide si la grilla se queda.
- Tauri: `lib.rs` expone los comandos, guarda el último informe y aplica Liquid Glass con `window-vibrancy`.
- Rust: `scan` propone, `apps` dice qué está instalado, `size` mide, `safety` autoriza, `clean` mueve a la Papelera, `model` es el informe.
- macOS: carpeta del usuario, Papelera y material de la ventana.
- Alternativa rechazada: un diagrama de secuencia del clic. La solicitud pide componentes, no un flujo.
- Alternativa rechazada: copiar el diagrama hexagonal del marco. [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md) ya descartó HTTP y SolidJS para este producto.

## Pasos y entregables

1. Guardar este plan y su listado — [TASK-0011-01](TASKS-0011-diagrama-de-componentes.md#task-0011-01--dejar-el-plan).
2. Añadir el diagrama al README y enlazarlo — [TASK-0011-02](TASKS-0011-diagrama-de-componentes.md#task-0011-02--dibujar-los-componentes).
3. Subir el cambio a GitHub — [TASK-0011-03](TASKS-0011-diagrama-de-componentes.md#task-0011-03--subir-a-github). Depende del paso 2.

## Riesgos, seguridad y bloqueos

- Riesgo: atribuir el vidrio a CSS. Mitigación: el diagrama lo deja en macOS, aplicado por Tauri.
- No se toca `safety.rs` ni la lista blanca. No hay secretos. No hay bloqueo.

## Validación y evidencia esperada

Los enlaces locales del README y del plan resuelven. `git diff --check` no reporta espacios conflictivos. El remoto recibe el commit. No hay prueba nueva: el diagrama no ejecuta código.

## Entrega, documentación, decisiones y recuperación

Changelog, `AGENTS.md` y el párrafo de planes del README. Sin ADR, RD ni release. Recuperación: revertir el commit.

## Seguimiento y resultados

Completado el 2026-10-15. El README dibuja interfaz, Tauri, Rust y macOS. Liquid Glass queda en la ventana nativa. No hubo ADR, RD ni release. La subida a `main` queda en [TASK-0011-03](TASKS-0011-diagrama-de-componentes.md#task-0011-03--subir-a-github).
