# TASKS-0008 — Dejar Salir siempre visible junto a Analizar

- **Plan:** [PLAN-0008](PLAN-0008-salir-junto-a-analizar.md)
- **Fecha:** 2026-10-15
- **Coordinación e integración:** agente principal de esta sesión
- **Estrategia de ejecución y propiedad de archivos compartidos:** un solo agente, en secuencia. Este listado y el plan son del mismo agente.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
|---|---|---|---|---|
| TASK-0008-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0008-02 | Mover los botones | TASK-0008-01 | Agente principal | completada |
| TASK-0008-03 | Registrar | TASK-0008-02 | Agente principal | completada |

## TASK-0008-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de cambiar la ventana. No tocar `src/` ni `index.html` en esta tarea.
- **Entradas y dependencias:** ninguna.
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan distingue la barra inferior de un botón flotante que tape «Revisar y limpiar».
- **Paralelismo, límites de edición y destinatario de la entrega:** no paralelizable con el código. Entrega a la misma sesión.
- **Resultado y evidencia:** archivos creados el 2026-10-15.

## TASK-0008-02 — Mover los botones

- **Referencia al plan:** paso 2 y los cinco criterios.
- **Objetivo y alcance:** barra inferior con Analizar a la izquierda y Salir a la derecha, visible en todas las pantallas y en la hoja de confirmación. Quitar las copias de arriba y del centro. No cambiar la limpieza.
- **Entradas y dependencias:** TASK-0008-01.
- **Entregable:** `index.html`, `src/main.js` y `src/styles.css`.
- **Aceptación y validación:** un solo par de botones, en el pie, y la hoja no los cubre.
- **Paralelismo, límites de edición y destinatario de la entrega:** un agente. Entrega a TASK-0008-03.
- **Resultado y evidencia:** el pie tiene Analizar a la izquierda y Salir a la derecha. Las copias de la barra de título, del inicio y del cierre se quitaron.

## TASK-0008-03 — Registrar

- **Referencia al plan:** entrega y RD.
- **Objetivo y alcance:** actualizar la línea base, registrar el RD, README y changelog. No publicar.
- **Entradas y dependencias:** TASK-0008-02.
- **Entregable:** documentos al día.
- **Aceptación y validación:** REQ-0004 dice que Salir está abajo a la derecha de Analizar.
- **Paralelismo, límites de edición y destinatario de la entrega:** cierre. Entrega a la persona usuaria.
- **Resultado y evidencia:** REQ-0004, RD-0002, README y changelog quedaron actualizados el 2026-10-15. No se publicó.
