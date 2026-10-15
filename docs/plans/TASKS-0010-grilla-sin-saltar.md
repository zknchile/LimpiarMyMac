# TASKS-0010 — Dejar la grilla donde se marcó

- **Plan:** [PLAN-0010](PLAN-0010-grilla-sin-saltar.md)
- **Fecha:** 2026-10-15
- **Coordinación e integración:** agente principal
- **Ejecución y propiedad:** un agente, en secuencia. No hay edición paralela.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
| --- | --- | --- | --- | --- |
| TASK-0010-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0010-02 | Conservar el desplazamiento | TASK-0010-01 | Agente principal | completada |
| TASK-0010-03 | Verificar y registrar | TASK-0010-02 | Agente principal | completada |

## TASK-0010-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de editar la interfaz. No tocar `src/`.
- **Entradas y dependencias:** ninguna. El fallo está en `render()` de `src/main.js`.
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan nombra la causa y no autoriza cambiar la limpieza.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** no paralelizable con el código. Entrega a la misma sesión.
- **Resultado y evidencia:** archivos creados el 2026-10-15, antes del cambio en `src/`.

## TASK-0010-02 — Conservar el desplazamiento

- **Referencia al plan:** paso 2 y los criterios de la misma categoría, de otra categoría y de otra pantalla.
- **Objetivo y alcance:** decidir en `src/place.js` qué desplazamiento se conserva y aplicarlo en `src/main.js`. Fichar la prueba. No tocar Rust ni `safety.rs`.
- **Entradas y dependencias:** TASK-0010-01.
- **Entregable:** la lista de la misma categoría no nace en 0 tras marcar.
- **Aceptación y validación:** la prueba de `src/place.test.js` cubre las tres variantes. La ventana usa esa decisión.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** un agente. Entrega a TASK-0010-03.
- **Resultado y evidencia:** `src/place.js` decide el desplazamiento. `src/main.js` lo aplica después de redibujar y devuelve el foco a la casilla sin desplazar, salvo que la hoja esté abierta. Fichas TEST-place-001 a TEST-place-003.

## TASK-0010-03 — Verificar y registrar

- **Referencia al plan:** validación, changelog y seguimiento.
- **Objetivo y alcance:** ejecutar `npm test`, actualizar catálogo, registro, changelog, README y `AGENTS.md`. No afirmar que el clic en la ventana quedó probado.
- **Entradas y dependencias:** TASK-0010-02.
- **Entregable:** evidencia de la prueba y plan cerrado.
- **Aceptación y validación:** el comando pasa. El plan queda `completado` solo si esa salida existe. `git diff --check` no reporta espacios conflictivos.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** cierre. Entrega a la persona usuaria.
- **Resultado y evidencia:** el 2026-10-15, `npm test` pasó 6 pruebas y 0 fallos. La salida está en [RUN-2026-10-15-02](../testing/TEST-RUNS.md#run-2026-10-15-02). Changelog, README y `AGENTS.md` quedaron actualizados. El clic en la ventana no se comprobó.
