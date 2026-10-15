# TASKS-0007 — Mostrar completa la nota del registro interno

- **Plan:** [PLAN-0007](PLAN-0007-nota-del-registro-visible.md)
- **Fecha:** 2026-10-15
- **Coordinación e integración:** agente principal de esta sesión
- **Estrategia de ejecución y propiedad de archivos compartidos:** un solo agente, en secuencia. Este listado y el plan son del mismo agente.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
|---|---|---|---|---|
| TASK-0007-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0007-02 | Nota completa | TASK-0007-01 | Agente principal | completada |
| TASK-0007-03 | Verificar y registrar | TASK-0007-02 | Agente principal | completada |

## TASK-0007-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de cambiar la ventana. No tocar `src/` en esta tarea.
- **Entradas y dependencias:** ninguna.
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan dice que el aviso se corta y que el cascarón no debe volver a listarse.
- **Paralelismo, límites de edición y destinatario de la entrega:** no paralelizable con el código. Entrega a la misma sesión.
- **Resultado y evidencia:** archivos creados el 2026-10-15.

## TASK-0007-02 — Nota completa

- **Referencia al plan:** paso 2.
- **Objetivo y alcance:** nota con salto de línea y texto que confirma la Papelera. El tamaño ignora `.DS_Store`. No ampliar la lista blanca.
- **Entradas y dependencias:** TASK-0007-01.
- **Entregable:** la frase se lee entera y el cascarón queda bajo el mínimo de 4 KB.
- **Aceptación y validación:** prueba de `size.rs`.
- **Paralelismo, límites de edición y destinatario de la entrega:** un agente. Entrega a TASK-0007-03.
- **Resultado y evidencia:** la nota usa `.done-note` y dice que el resto está en la Papelera. `path_size` no suma `.DS_Store`.

## TASK-0007-03 — Verificar y registrar

- **Referencia al plan:** validación y changelog.
- **Objetivo y alcance:** `cargo test --lib` y documentos. No limpiar el disco real.
- **Entradas y dependencias:** TASK-0007-02.
- **Entregable:** pruebas en verde.
- **Aceptación y validación:** el comando pasa.
- **Paralelismo, límites de edición y destinatario de la entrega:** cierre. Entrega a la persona usuaria.
- **Resultado y evidencia:** el 2026-10-15, `cargo test --lib size::tests::path_size_ignores_container_metadata` pasó. El disco real no se limpió.
