# TASKS-0018 — Profundidad de los botones

- **Plan:** [PLAN-0018](PLAN-0018-profundidad-de-botones.md)
- **Fecha:** 2026-11-16
- **Coordinación e integración:** agente principal
- **Ejecución y propiedad:** un agente, en secuencia. No hay edición paralela de `view.rs`.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
| --- | --- | --- | --- | --- |
| TASK-0018-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0018-02 | Pintar la sombra | TASK-0018-01 | Agente principal | completada |

## TASK-0018-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de editar la vista. No tocar `src-tauri/src/`.
- **Entradas y dependencias:** ninguna. Los botones vigentes son planos, según [PLAN-0017](PLAN-0017-paleta-dia-lluvioso.md).
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan fija la sombra de tinta y nube, y no afirma Liquid Glass ni un sexto hex.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** no paralelizable con el cambio de la vista. Entrega a TASK-0018-02.
- **Resultado y evidencia:** archivos creados el 2026-11-16, antes de editar `button()`.

## TASK-0018-02 — Pintar la sombra

- **Referencia al plan:** paso 2 y el reparto de sombras.
- **Objetivo y alcance:** dar a `button()` la sombra de reposo, la de puntero y la de pulsación. Anotarla en el diseño aplicado, el changelog, `AGENTS.md` y el párrafo de planes. No cambiar `safety.rs`, el sitio de Salir, la paleta, la versión ni las pruebas.
- **Entradas y dependencias:** TASK-0018-01.
- **Entregable:** el botón principal habilitado se separa del velo. Un lector no cree que esto cumple [REQ-0003](../requirements/baseline.md).
- **Aceptación y validación:** `button()` llama a `shadow` de GPUI. El deshabilitado no lleva sombra. `git diff --check` no reporta espacios conflictivos. La ventana abre titulada Cristal y Salir sigue a la derecha de Analizar. No se pulsa «Mover a la Papelera».
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** cierre. No se empuja en esta tarea.
- **Resultado y evidencia:** el 2026-11-16, `button()` llama a `shadow` de GPUI con tinta y nube. El deshabilitado no lleva sombra. `git diff --check` no reportó espacios conflictivos. `cargo test --lib` pasó 14 pruebas y 0 fallos. La ventana abierta se listó como `title=Cristal`. Salir sigue abajo a la derecha de Analizar. Bajo el botón principal un pixel salió en 177,182,191, frente a 228,233,239 en la barra. El diseño niega `NSGlassEffectView`. No se empujó.
