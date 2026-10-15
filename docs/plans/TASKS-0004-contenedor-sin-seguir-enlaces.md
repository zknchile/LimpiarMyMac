# TASKS-0004 — Mover un contenedor sin seguir sus enlaces

- **Plan:** [PLAN-0004](PLAN-0004-contenedor-sin-seguir-enlaces.md)
- **Fecha:** 2026-10-15
- **Coordinación e integración:** agente principal de esta sesión
- **Estrategia de ejecución y propiedad de archivos compartidos:** un solo agente, en secuencia. Este listado y el plan son del mismo agente.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
|---|---|---|---|---|
| TASK-0004-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0004-02 | No seguir enlaces del contenedor | TASK-0004-01 | Agente principal | completada |
| TASK-0004-03 | Verificar y registrar | TASK-0004-02 | Agente principal | completada |

## TASK-0004-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de cambiar la limpieza. No tocar `src-tauri/src` en esta tarea.
- **Entradas y dependencias:** ninguna.
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan nombra el riesgo de seguir enlaces y no autoriza borrar el contenedor real desde el agente.
- **Paralelismo, límites de edición y destinatario de la entrega:** no paralelizable con el código. Entrega a la misma sesión.
- **Resultado y evidencia:** archivos creados el 2026-10-15.

## TASK-0004-02 — No seguir enlaces del contenedor

- **Referencia al plan:** paso 2 y los tres criterios.
- **Objetivo y alcance:** plan de limpieza en `safety.rs` y ejecución en `clean.rs`. Ajustar el texto del análisis de contenedores. No ampliar la lista blanca ni usar `rm` sobre archivos reales.
- **Entradas y dependencias:** TASK-0004-01. La ruta confirmada es `~/Library/Containers/com.utmapp.UTM`.
- **Entregable:** una carpeta con enlaces hacia fuera se parte en pasos seguros.
- **Aceptación y validación:** prueba de `safety.rs` con directorio temporal. Ningún paso apunta al destino del enlace.
- **Paralelismo, límites de edición y destinatario de la entrega:** un agente. Entrega a TASK-0004-03.
- **Resultado y evidencia:** `disposal_plan` en `safety.rs` y la ejecución en `clean.rs`. El texto del análisis de contenedores avisa que esos enlaces no se mueven.

## TASK-0004-03 — Verificar y registrar

- **Referencia al plan:** validación, seguridad y changelog.
- **Objetivo y alcance:** ejecutar `cargo test --lib`, actualizar seguridad, ADR-0002 y changelog. No limpiar el contenedor real.
- **Entradas y dependencias:** TASK-0004-02.
- **Entregable:** pruebas en verde y documentos al día.
- **Aceptación y validación:** el comando pasa. La persona comprueba la ventana después de analizar de nuevo.
- **Paralelismo, límites de edición y destinatario de la entrega:** cierre. Entrega a la persona usuaria.
- **Resultado y evidencia:** el 2026-10-15, `cargo test --lib` pasó 5 pruebas y 0 fallos. Seguridad, ADR-0002 y changelog quedaron actualizados. El contenedor real no se tocó.
