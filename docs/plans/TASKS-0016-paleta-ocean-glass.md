# TASKS-0016 — Paleta Ocean Glass

- **Plan:** [PLAN-0016](PLAN-0016-paleta-ocean-glass.md)
- **Fecha:** 2026-11-16
- **Coordinación e integración:** agente principal
- **Ejecución y propiedad:** un agente, en secuencia. No hay edición paralela de `view.rs`.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
| --- | --- | --- | --- | --- |
| TASK-0016-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0016-02 | Aplicar la paleta | TASK-0016-01 | Agente principal | completada |

## TASK-0016-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de editar la vista. No tocar `src-tauri/`.
- **Entradas y dependencias:** ninguna. La paleta vigente es la de [PLAN-0015](PLAN-0015-colores-mas-suaves.md).
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan fija los cinco hex, no mueve Salir y no afirma Liquid Glass.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** no paralelizable con el cambio de color. Entrega a TASK-0016-02.
- **Resultado y evidencia:** archivos creados el 2026-11-16, antes de editar `view.rs`.

## TASK-0016-02 — Aplicar la paleta

- **Referencia al plan:** paso 2 y el reparto de colores.
- **Objetivo y alcance:** sustituir las constantes y los `rgb` de la vista. Ampliar el margen de las pantallas de texto y el relleno de los botones. Anotar la paleta en el diseño aplicado, el changelog, `AGENTS.md` y el párrafo de planes. No cambiar `safety.rs`, el sitio de Salir, la versión ni las pruebas.
- **Entradas y dependencias:** TASK-0016-01.
- **Entregable:** la ventana usa solo esos cinco colores. Un lector no cree que el nombre Ocean Glass cumple [REQ-0003](../requirements/baseline.md).
- **Aceptación y validación:** no queda `#f4f1eb` ni `#45687a`. `git diff --check` no reporta espacios conflictivos. La ventana abre titulada Cristal. Salir sigue a la derecha de Analizar.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** cierre. No se empuja en esta tarea.
- **Resultado y evidencia:** el 2026-11-16, la vista usa bruma, profundo, medio, vivo y hielo. No queda `#f4f1eb` ni `#45687a`. `git diff --check` no reportó espacios conflictivos. `cargo test --lib` pasó 14 pruebas y 0 fallos. La ventana abierta se listó como `title=Cristal`. Salir sigue abajo a la derecha de Analizar. El diseño aplicado niega `NSGlassEffectView`. No se empujó.
