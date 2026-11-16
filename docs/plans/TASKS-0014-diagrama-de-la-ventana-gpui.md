# TASKS-0014 — Diagrama de la ventana GPUI

- **Plan:** [PLAN-0014](PLAN-0014-diagrama-de-la-ventana-gpui.md)
- **Fecha:** 2026-11-16
- **Coordinación e integración:** agente principal
- **Ejecución y propiedad:** un agente, en secuencia. No hay edición paralela del README.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
| --- | --- | --- | --- | --- |
| TASK-0014-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0014-02 | Redibujar la sección | TASK-0014-01 | Agente principal | completada |

## TASK-0014-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de editar el README. No tocar `src-tauri/`.
- **Entradas y dependencias:** ninguna. El diagrama actual está en «Cómo está armada».
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan no autoriza cambiar la limpieza ni dibujar vidrio nativo.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** no paralelizable con el README. Entrega a TASK-0014-02.
- **Resultado y evidencia:** archivos creados el 2026-11-16, antes de editar `README.md`.

## TASK-0014-02 — Redibujar la sección

- **Referencia al plan:** paso 2 y los criterios del diagrama.
- **Objetivo y alcance:** sustituir el diagrama y la tabla de «Cómo está armada». Actualizar changelog, `AGENTS.md` y el párrafo de planes. No cambiar código ni la versión.
- **Entradas y dependencias:** TASK-0014-01.
- **Entregable:** un lector no crea que la ventana y el análisis son procesos distintos, ni que siguen los comandos de Tauri.
- **Aceptación y validación:** el diagrama nombra un solo binario, el hilo de pintado, `safety` en ambos caminos y Metal sin vidrio. `git diff --check` no reporta espacios conflictivos.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** cierre. No se empuja en esta tarea.
- **Resultado y evidencia:** el 2026-11-16, el diagrama y la tabla de «Cómo está armada» nombran un solo binario. `safety` está en el análisis y en la limpieza. El texto dice que Metal no es `NSGlassEffectView`. Changelog y `AGENTS.md` enlazan este plan. No se tocó `src-tauri/src/`. No se empujó.
