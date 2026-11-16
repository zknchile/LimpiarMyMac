# TASKS-0019 — Franja del menú un poco más oscura

- **Plan:** [PLAN-0019](PLAN-0019-franja-del-menu.md)
- **Fecha:** 2026-11-16
- **Coordinación e integración:** agente principal
- **Ejecución y propiedad:** un agente, en secuencia. No hay edición paralela de `view.rs` y `main.rs`.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
| --- | --- | --- | --- | --- |
| TASK-0019-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0019-02 | Oscurecer la franja | TASK-0019-01 | Agente principal | completada |

## TASK-0019-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de editar la vista. No tocar `src-tauri/src/`.
- **Entradas y dependencias:** ninguna. La franja vigente es el blanco del sistema.
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan fija `#D2DAE5` al 88 % y no afirma Liquid Glass ni un sexto acento.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** no paralelizable con el cambio de color. Entrega a TASK-0019-02.
- **Resultado y evidencia:** archivos creados el 2026-11-16, antes de editar la vista.

## TASK-0019-02 — Oscurecer la franja

- **Referencia al plan:** paso 2 y la mezcla.
- **Objetivo y alcance:** pedir la barra transparente y pintar la franja de 32 pt con la mezcla, más el título «Cristal» en tinta. Anotarla en el diseño aplicado, el changelog, `AGENTS.md` y el párrafo de planes. No cambiar `safety.rs`, el sitio de Salir, los botones, la versión ni las pruebas.
- **Entradas y dependencias:** TASK-0019-01.
- **Entregable:** la franja deja de ser blanca. Un lector no cree que esto cumple [REQ-0003](../requirements/baseline.md).
- **Aceptación y validación:** `main.rs` pide `appears_transparent`. La vista pinta `#D2DAE5` al 88 %. `git diff --check` no reporta espacios conflictivos. La ventana abre titulada Cristal. Salir sigue a la derecha de Analizar. No se pulsa «Mover a la Papelera».
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** cierre. No se empuja en esta tarea.
- **Resultado y evidencia:** el 2026-11-16, `main.rs` pide `appears_transparent` y la vista pinta `#D2DAE5` al 88 %. Un pixel de la franja salió en 213,219,230, frente a 228,232,239 en el velo. `git diff --check` no reportó espacios conflictivos. `cargo test --lib` pasó 14 pruebas y 0 fallos. La ventana abierta se listó como `title=Cristal`. Salir sigue abajo a la derecha de Analizar. El diseño niega `NSGlassEffectView`. No se empujó.
