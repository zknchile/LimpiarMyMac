# TASKS-0017 — Paleta de día lluvioso

- **Plan:** [PLAN-0017](PLAN-0017-paleta-dia-lluvioso.md)
- **Fecha:** 2026-11-16
- **Coordinación e integración:** agente principal
- **Ejecución y propiedad:** un agente, en secuencia. No hay edición paralela de `view.rs` y `main.rs`.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
| --- | --- | --- | --- | --- |
| TASK-0017-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0017-02 | Aplicar la paleta | TASK-0017-01 | Agente principal | completada |

## TASK-0017-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de editar la vista. No tocar `src-tauri/src/`.
- **Entradas y dependencias:** ninguna. La paleta vigente es la de [PLAN-0016](PLAN-0016-paleta-ocean-glass.md).
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan fija los cinco hex, el velo del 88 % y no afirma Liquid Glass.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** no paralelizable con el cambio de color. Entrega a TASK-0017-02.
- **Resultado y evidencia:** archivos creados el 2026-11-16, antes de editar la vista.

## TASK-0017-02 — Aplicar la paleta

- **Referencia al plan:** paso 2 y el reparto de colores.
- **Objetivo y alcance:** sustituir las constantes, el fondo de la ventana y el sello cian. El título usa la tinta. Lo secundario usa la ceniza. Las líneas usan el pelo. Anotar el velo en el diseño aplicado, el changelog, `AGENTS.md` y el párrafo de planes. No cambiar `safety.rs`, el sitio de Salir, la versión ni las pruebas.
- **Entradas y dependencias:** TASK-0017-01.
- **Entregable:** la ventana usa solo esos cinco colores, con el fondo al 88 %. Un lector no cree que esto cumple [REQ-0003](../requirements/baseline.md).
- **Aceptación y validación:** no queda `#22D3EE` ni `#ECFEFF`. `main.rs` pide `WindowBackgroundAppearance::Transparent`. `git diff --check` no reporta espacios conflictivos. La ventana abre titulada Cristal. Salir sigue a la derecha de Analizar.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** cierre. No se empuja en esta tarea.
- **Resultado y evidencia:** el 2026-11-16, la vista usa tinta, pizarra, ceniza, pelo y nube. No queda `#22D3EE` ni `#ECFEFF`. `main.rs` pide `WindowBackgroundAppearance::Transparent`. Un pixel del fondo salió con alfa 0.88. `git diff --check` no reportó espacios conflictivos. `cargo test --lib` pasó 14 pruebas y 0 fallos. La ventana abierta se listó como `title=Cristal`. Salir sigue abajo a la derecha de Analizar. El diseño aplicado niega `NSGlassEffectView`. No se empujó.
