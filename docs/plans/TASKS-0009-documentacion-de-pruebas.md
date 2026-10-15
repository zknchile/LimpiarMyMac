# TASKS-0009 — Documentar las pruebas de Cristal

- **Plan:** [PLAN-0009](PLAN-0009-documentacion-de-pruebas.md)
- **Fecha:** 2026-10-15
- **Coordinación e integración:** agente principal
- **Ejecución y propiedad:** un agente, propietario de los archivos de esta revisión

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
| --- | --- | --- | --- | --- |
| TASK-0009-01 | Fichar las pruebas existentes | Ninguna | Agente principal | completada |
| TASK-0009-02 | Integrar y verificar | TASK-0009-01 | Agente principal | completada |

## TASK-0009-01 — Fichar las pruebas existentes

- **Referencia al plan:** paso 1 y criterios del catálogo.
- **Objetivo y alcance:** escribir [TEST-CASES.md](../testing/TEST-CASES.md) con las 11 pruebas automatizadas y enlazar cada ID desde el test. No añadir aserciones ni fichar la interfaz, el disco real o la Papelera.
- **Entradas y dependencias:** política del marco y el código de `safety.rs`, `apps.rs`, `size.rs` y `src/selection.test.js`. Ninguna dependencia de otra tarea.
- **Entregable:** catálogo con ocho campos por caso e ID en el fuente.
- **Aceptación y validación:** cada `#[test]` y cada `test(` de `selection.test.js` tiene una ficha, y cada ficha nombra ese símbolo. Los faltantes conocidos no reciben ID.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** secuencial; entregar el catálogo a TASK-0009-02.
- **Resultado y evidencia:** [TEST-CASES.md](../testing/TEST-CASES.md) ficha 11 casos. Cada ID está comentado en `safety.rs`, `apps.rs`, `size.rs` o `src/selection.test.js`. Los faltantes de interfaz y de limpieza real quedaron en «Sin ficha», sin ID.

## TASK-0009-02 — Integrar y verificar

- **Referencia al plan:** paso 2 y el resto de criterios de aceptación.
- **Objetivo y alcance:** alinear pruebas, agentes, README, changelog, requisito y seguridad; ejecutar las dos suites; guardar [TEST-RUNS.md](../testing/TEST-RUNS.md); comprobar enlaces y cerrar el plan.
- **Entradas y dependencias:** TASK-0009-01 completada.
- **Entregable:** fuentes coherentes, registro de la ejecución de esta entrega y plan cerrado.
- **Aceptación y validación:** las suites pasan en el host; el registro identifica revisión, entorno, comandos, los 11 IDs y la salida. Enlaces locales de los archivos tocados resuelven. `git diff --check` no reporta espacios conflictivos. No se declara cubierta una prueba que no se ejecutó.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** después de TASK-0009-01. El agente principal integra y cierra.
- **Resultado y evidencia:** el 2026-10-15, en el host macOS 27.0.1 arm64, sobre el commit `4688269` y comentarios locales de ID, `cargo test --lib` pasó 8 pruebas y `npm test` pasó 3. Cero fallos. La salida está en [RUN-2026-10-15-01](../testing/TEST-RUNS.md#run-2026-10-15-01). Quedaron alineados TESTING, AGENTS, README, changelog, REQ-0007, RD-0001 y seguridad. Los enlaces locales de esta revisión resuelven. `git diff --check` no reportó espacios conflictivos. No se declaró aprobada ninguna prueba de ventana o de Papelera.
