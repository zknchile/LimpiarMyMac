# TASKS-0015 — Colores más suaves

- **Plan:** [PLAN-0015](PLAN-0015-colores-mas-suaves.md)
- **Fecha:** 2026-11-16
- **Coordinación e integración:** agente principal
- **Ejecución y propiedad:** un agente, en secuencia. No hay edición paralela de `view.rs`.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
| --- | --- | --- | --- | --- |
| TASK-0015-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0015-02 | Aplicar la paleta | TASK-0015-01 | Agente principal | completada |

## TASK-0015-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de editar la vista. No tocar `src-tauri/`.
- **Entradas y dependencias:** ninguna. Los colores actuales están en `src-tauri/src/view.rs`.
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan nombra la paleta, no mueve Salir y no afirma Liquid Glass.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** no paralelizable con el cambio de color. Entrega a TASK-0015-02.
- **Resultado y evidencia:** archivos creados el 2026-11-16, antes de editar `view.rs`.

## TASK-0015-02 — Aplicar la paleta

- **Referencia al plan:** paso 2 y la paleta del diseño.
- **Objetivo y alcance:** sustituir los `rgb` sueltos por las constantes del plan. Anotar la paleta en el diseño aplicado, el changelog, `AGENTS.md` y el párrafo de planes. No cambiar `safety.rs`, el sitio de Salir, la versión ni las pruebas.
- **Entradas y dependencias:** TASK-0015-01.
- **Entregable:** la ventana usa el papel, la tinta cálida y el azul apagado. Un lector no cree que esto cumple [REQ-0003](../requirements/baseline.md).
- **Aceptación y validación:** no queda `1d4ed8` ni `1c1c1c` en la vista. `git diff --check` no reporta espacios conflictivos. La ventana abre titulada Cristal.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** cierre. No se empuja en esta tarea.
- **Resultado y evidencia:** el 2026-11-16, la vista usa las constantes del plan. No queda `1d4ed8` ni `1c1c1c`. `git diff --check` no reportó espacios conflictivos. `cargo test --lib` pasó 14 pruebas y 0 fallos. La ventana abierta se listó como `title=Cristal`. Salir sigue abajo a la derecha de Analizar. El diseño aplicado niega `NSGlassEffectView`. No se empujó.
