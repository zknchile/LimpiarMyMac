# TASKS-0001 — Documentar el desarrollo de Cristal

- **Plan:** [PLAN-0001](PLAN-0001-documentar-desarrollo.md)
- **Fecha:** 2026-10-15
- **Coordinación e integración:** agente principal de esta sesión
- **Estrategia de ejecución y propiedad de archivos compartidos:** un solo agente, en secuencia. Este listado y el plan son del mismo agente. No hay edición paralela.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
|---|---|---|---|---|
| TASK-0001-01 | Leer el marco y el producto | Ninguna | Agente principal | completada |
| TASK-0001-02 | Escribir la línea base | TASK-0001-01 | Agente principal | completada |
| TASK-0001-03 | Reconstruir el registro de tareas | TASK-0001-01 | Agente principal | completada |
| TASK-0001-04 | Integrar y verificar | TASK-0001-02, TASK-0001-03 | Agente principal | completada |

## TASK-0001-01 — Leer el marco y el producto

- **Referencia al plan:** fuentes y paso 1.
- **Objetivo y alcance:** leer planificación, política, formato de decisiones, un ADR, un RD y Liquid Glass en Arquitectura_agent. Contrastar con el código de Cristal. No editar el marco.
- **Entradas y dependencias:** ninguna. Cuenta `zknchile` con acceso al marco.
- **Entregable:** lista de fuentes aplicables y desviaciones reales (Tauri, host, sin IA, sin HTTP).
- **Aceptación y validación:** las desviaciones aparecen en ADR-0001 y en `AGENTS.md`, no como si el marco se hubiera cumplido al pie de la letra.
- **Paralelismo, límites de edición y destinatario de la entrega:** no paralelizable con la escritura. Entrega interna a TASK-0001-02.
- **Resultado y evidencia:** lectura hecha el 2026-10-15 vía `gh api` sobre `zknchile/Arquitectura_agent`. Desviaciones registradas en [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md).

## TASK-0001-02 — Escribir la línea base

- **Referencia al plan:** paso 2 y criterios de decisiones con evidencia.
- **Objetivo y alcance:** crear requisitos, decisiones, diseño, seguridad, pruebas, changelog y `AGENTS.md`. No cambiar Rust ni la interfaz.
- **Entradas y dependencias:** TASK-0001-01. Pedidos de la sesión y commit `c9e6f67`.
- **Entregable:** documentos enlazados desde `AGENTS.md`.
- **Aceptación y validación:** cada ADR tiene contexto, alternativas y motivo. El RD distingue requisito anterior y nuevo. No hay carpetas vacías de simetría.
- **Paralelismo, límites de edición y destinatario de la entrega:** puede convivir con TASK-0001-03 si no tocan los mismos archivos. En esta sesión fue secuencial. Entrega a coordinación.
- **Resultado y evidencia:** archivos en `docs/requirements`, `docs/decisions`, `docs/design`, `docs/security`, `docs/testing`, más `AGENTS.md` y `CHANGELOG.md`.

## TASK-0001-03 — Reconstruir el registro de tareas

- **Referencia al plan:** paso 3. El desarrollo inicial no tuvo plan previo.
- **Objetivo y alcance:** escribir [PLAN-0002](PLAN-0002-registro-desarrollo-cristal.md) y [TASKS-0002](TASKS-0002-registro-desarrollo-cristal.md) como registro reconstruido, con evidencia y limitaciones. No marcar como planificado antes de codificar algo que ya estaba hecho.
- **Entradas y dependencias:** TASK-0001-01. Evidencia: código, pruebas de la sesión y aceptación verbal de la persona usuaria.
- **Entregable:** plan y listado con el mismo número, enlaces recíprocos e IDs `TASK-0002-nn`.
- **Aceptación y validación:** el encabezado dice que es un registro posterior. Cada tarea cerrada cita evidencia. Lo no comprobado queda pendiente, no como éxito.
- **Paralelismo, límites de edición y destinatario de la entrega:** archivos distintos de TASK-0001-02. Entrega a coordinación.
- **Resultado y evidencia:** ambos archivos creados y enlazados. El análisis real del disco y la restauración desde la Papelera quedan como no comprobados.

## TASK-0001-04 — Integrar y verificar

- **Referencia al plan:** paso 4 y validación.
- **Objetivo y alcance:** enlazar el README, comprobar enlaces locales e IDs, y cerrar este listado y PLAN-0001. No ampliar el producto.
- **Entradas y dependencias:** TASK-0001-02 y TASK-0001-03 completadas.
- **Entregable:** documentación coherente y diff sin errores de espacios.
- **Aceptación y validación:** enlaces relativos de `docs/` y `AGENTS.md` resuelven; IDs únicos; `git diff --check` limpio.
- **Paralelismo, límites de edición y destinatario de la entrega:** después de las otras tres. Coordinación es dueña de este listado. Entrega a la persona usuaria.
- **Resultado y evidencia:** el 2026-10-15 se recorrieron los enlaces locales de `*.md` fuera de `node_modules`. Los archivos existen y las anclas coinciden con el slug de GitHub, incluidas `línea` y `verificación`. Los IDs `REQ-0001` a `REQ-0007`, `ADR-0001`, `ADR-0002`, `RD-0001`, `PLAN-0001`, `PLAN-0002` y `TASK-0001-01` a `TASK-0002-06` no se repiten. `git diff --check` no reportó espacios conflictivos.
