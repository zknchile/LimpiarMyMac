# TASKS-0011 — Diagrama de componentes en el README

- **Plan:** [PLAN-0011](PLAN-0011-diagrama-de-componentes.md)
- **Fecha:** 2026-10-15
- **Coordinación e integración:** agente principal
- **Ejecución y propiedad:** un agente, en secuencia. No hay edición paralela.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
| --- | --- | --- | --- | --- |
| TASK-0011-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0011-02 | Dibujar los componentes | TASK-0011-01 | Agente principal | completada |
| TASK-0011-03 | Subir a GitHub | TASK-0011-02 | Agente principal | completada |

## TASK-0011-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de editar el README. No tocar `src/` ni `src-tauri/`.
- **Entradas y dependencias:** ninguna. La forma real está en [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md) y en los módulos nombrados en el plan.
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan no autoriza un servicio HTTP ni un ADR nuevo.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** no paralelizable con el README. Entrega a la misma sesión.
- **Resultado y evidencia:** archivos creados el 2026-10-15, antes del cambio en `README.md`.

## TASK-0011-02 — Dibujar los componentes

- **Referencia al plan:** paso 2 y los criterios del diagrama.
- **Objetivo y alcance:** añadir el diagrama Mermaid y una tabla corta de archivos en «Cómo está armada». Actualizar changelog, `AGENTS.md` y el índice de planes del README. No cambiar código ni la versión.
- **Entradas y dependencias:** TASK-0011-01.
- **Entregable:** el README distingue interfaz, Tauri, Rust y macOS.
- **Aceptación y validación:** el diagrama nombra `safety` en análisis y limpieza, y no atribuye Liquid Glass al CSS. Los enlaces locales nuevos resuelven.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** un agente. Entrega a TASK-0011-03.
- **Resultado y evidencia:** el 2026-10-15, «Cómo está armada» del README tiene el diagrama y la tabla de archivos. Changelog y `AGENTS.md` enlazan este plan. No se tocó `src/` ni `src-tauri/`.

## TASK-0011-03 — Subir a GitHub

- **Referencia al plan:** paso 3 y la entrega.
- **Objetivo y alcance:** confirmar que no hay cambios de código y empujar `main`. No crear release.
- **Entradas y dependencias:** TASK-0011-02.
- **Entregable:** el commit está en `origin/main`.
- **Aceptación y validación:** `git status` queda limpio y `HEAD` coincide con `origin/main`. `git diff --check` no reporta espacios conflictivos.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** cierre. Entrega a la persona usuaria.
- **Resultado y evidencia:** el 2026-10-15, `863ec71` quedó en `origin/main`. No se creó release. `HEAD` coincidía con el remoto tras el push. Este cierre es el commit siguiente.
