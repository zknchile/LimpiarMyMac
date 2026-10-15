# TASKS-0005 — No fallar por el registro interno del contenedor

- **Plan:** [PLAN-0005](PLAN-0005-omitir-metadata-de-contenedor.md)
- **Fecha:** 2026-10-15
- **Coordinación e integración:** agente principal de esta sesión
- **Estrategia de ejecución y propiedad de archivos compartidos:** un solo agente, en secuencia. Este listado y el plan son del mismo agente.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
|---|---|---|---|---|
| TASK-0005-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0005-02 | No tratar el registro como fallo | TASK-0005-01 | Agente principal | completada |
| TASK-0005-03 | Verificar y registrar | TASK-0005-02 | Agente principal | completada |

## TASK-0005-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de cambiar la limpieza. No tocar `src-tauri/src` en esta tarea.
- **Entradas y dependencias:** ninguna.
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan nombra el plist bloqueado y no autoriza borrar el contenedor real desde el agente.
- **Paralelismo, límites de edición y destinatario de la entrega:** no paralelizable con el código. Entrega a la misma sesión.
- **Resultado y evidencia:** archivos creados el 2026-10-15.

## TASK-0005-02 — No tratar el registro como fallo

- **Referencia al plan:** paso 2 y los tres criterios.
- **Objetivo y alcance:** omitir `.com.apple.containermanagerd.*.plist` en el plan, en el tamaño y en la ejecución. Avisar en la ventana si macOS lo conserva. No ampliar la lista blanca.
- **Entradas y dependencias:** TASK-0005-01. La ruta confirmada es `~/Library/Containers/com.utmapp.UTM`.
- **Entregable:** una limpieza con ese archivo presente no lo entrega a Finder ni lo cuenta como error.
- **Aceptación y validación:** prueba de `safety.rs` y de `size.rs` con directorio temporal.
- **Paralelismo, límites de edición y destinatario de la entrega:** un agente. Entrega a TASK-0005-03.
- **Resultado y evidencia:** el plan omite `.com.apple.containermanagerd.*.plist`, el tamaño no lo cuenta y la ventana lo avisa como nota si macOS lo conserva.

## TASK-0005-03 — Verificar y registrar

- **Referencia al plan:** validación, seguridad y changelog.
- **Objetivo y alcance:** ejecutar `cargo test --lib`, actualizar seguridad, ADR-0002, pruebas y changelog. No limpiar el contenedor real.
- **Entradas y dependencias:** TASK-0005-02.
- **Entregable:** pruebas en verde y documentos al día.
- **Aceptación y validación:** el comando pasa. La persona comprueba la ventana después de analizar de nuevo.
- **Paralelismo, límites de edición y destinatario de la entrega:** cierre. Entrega a la persona usuaria.
- **Resultado y evidencia:** el 2026-10-15, `cargo test --lib` pasó 7 pruebas y 0 fallos. Seguridad, ADR-0002, pruebas y changelog quedaron actualizados. El contenedor real no se tocó.
