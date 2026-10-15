# TASKS-0006 — No bloquear una copia que vive dentro de otra app

- **Plan:** [PLAN-0006](PLAN-0006-no-bloquear-copia-dentro-del-contenedor.md)
- **Fecha:** 2026-10-15
- **Coordinación e integración:** agente principal de esta sesión
- **Estrategia de ejecución y propiedad de archivos compartidos:** un solo agente, en secuencia. Este listado y el plan son del mismo agente.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
|---|---|---|---|---|
| TASK-0006-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0006-02 | Juzgar la app dueña | TASK-0006-01 | Agente principal | completada |
| TASK-0006-03 | Verificar y registrar | TASK-0006-02 | Agente principal | completada |

## TASK-0006-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de cambiar la regla. No tocar `src-tauri/src` en esta tarea.
- **Entradas y dependencias:** ninguna.
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan distingue una copia dentro de UTM de `com.apple.mail` y de las preferencias del sistema.
- **Paralelismo, límites de edición y destinatario de la entrega:** no paralelizable con el código. Entrega a la misma sesión.
- **Resultado y evidencia:** archivos creados el 2026-10-15.

## TASK-0006-02 — Juzgar la app dueña

- **Referencia al plan:** paso 2 y los tres criterios.
- **Objetivo y alcance:** `is_apple_private` usa la identidad de la app. El aviso de rechazo incluye la ruta. La ventana también la muestra. No ampliar la lista blanca.
- **Entradas y dependencias:** TASK-0006-01.
- **Entregable:** un contenedor de terceros con un hijo `com.apple…` se puede planear. Mail y las preferencias de Apple no.
- **Aceptación y validación:** pruebas de `safety.rs`.
- **Paralelismo, límites de edición y destinatario de la entrega:** un agente. Entrega a TASK-0006-03.
- **Resultado y evidencia:** `is_apple_private` mira el nombre de la app. El aviso y la ventana incluyen la ruta.

## TASK-0006-03 — Verificar y registrar

- **Referencia al plan:** validación, seguridad y changelog.
- **Objetivo y alcance:** `cargo test --lib`, actualizar seguridad, ADR-0002, pruebas y changelog. No limpiar el disco real.
- **Entradas y dependencias:** TASK-0006-02.
- **Entregable:** pruebas en verde y documentos al día.
- **Aceptación y validación:** el comando pasa.
- **Paralelismo, límites de edición y destinatario de la entrega:** cierre. Entrega a la persona usuaria.
- **Resultado y evidencia:** el 2026-10-15, `cargo test --lib` pasó 8 pruebas y 0 fallos. Seguridad, ADR-0002, pruebas y changelog quedaron actualizados. El disco real no se limpió.
