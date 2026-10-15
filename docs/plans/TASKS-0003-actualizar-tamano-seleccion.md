# TASKS-0003 — Actualizar el tamaño al cambiar la selección

- **Plan:** [PLAN-0003](PLAN-0003-actualizar-tamano-seleccion.md)
- **Fecha:** 2026-10-15
- **Coordinación e integración:** agente principal de esta sesión
- **Estrategia de ejecución y propiedad de archivos compartidos:** un solo agente, en secuencia. Este listado y el plan son del mismo agente. No hay edición paralela.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
|---|---|---|---|---|
| TASK-0003-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0003-02 | Mostrar el tamaño seleccionado | TASK-0003-01 | Agente principal | completada |
| TASK-0003-03 | Verificar y registrar | TASK-0003-02 | Agente principal | completada |

## TASK-0003-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de editar la interfaz. No tocar `src/`.
- **Entradas y dependencias:** ninguna. El fallo está descrito en el plan.
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan cubre los criterios de aceptación y no abre un ADR ni un RD.
- **Paralelismo, límites de edición y destinatario de la entrega:** no paralelizable con la corrección. Entrega a la misma sesión.
- **Resultado y evidencia:** archivos creados el 2026-10-15. La corrección de código empieza después.

## TASK-0003-02 — Mostrar el tamaño seleccionado

- **Referencia al plan:** paso 2 y los tres criterios de tamaño.
- **Objetivo y alcance:** que resumen, barra, encabezado de categoría y pie muestren el tamaño seleccionado, y que el texto tenga decimales suficientes. Archivos: `src/selection.js`, `src/selection.test.js`, `src/main.js`, `package.json`. No tocar Rust ni `safety.rs`.
- **Entradas y dependencias:** TASK-0003-01.
- **Entregable:** la selección vacía muestra 0 B y un cambio de 80 MB sobre un total en GB cambia el texto.
- **Aceptación y validación:** `node --test src/selection.test.js` en verde. La interfaz usa esas funciones, no un cálculo paralelo que vuelva a tratar 0 como vacío.
- **Paralelismo, límites de edición y destinatario de la entrega:** un agente. Entrega a TASK-0003-03.
- **Resultado y evidencia:** `src/selection.js` calcula el texto y el total. `src/main.js` lo usa en la barra, el resumen, el encabezado de la categoría y el pie. Ya no hay un `bytes || total` en la fila del resumen.

## TASK-0003-03 — Verificar y registrar

- **Referencia al plan:** validación, changelog y seguimiento.
- **Objetivo y alcance:** ejecutar la prueba, actualizar changelog, pruebas y el cierre de este plan. No ampliar la limpieza.
- **Entradas y dependencias:** TASK-0003-02.
- **Entregable:** evidencia de la prueba y documentos al día.
- **Aceptación y validación:** el comando de prueba pasa y el plan queda `completado` solo si esa evidencia existe. La comprobación visual en la ventana queda para la persona usuaria.
- **Paralelismo, límites de edición y destinatario de la entrega:** cierre. Entrega a la persona usuaria.
- **Resultado y evidencia:** el 2026-10-15, `node --test src/selection.test.js` pasó 3 pruebas y 0 fallos. Changelog y pruebas quedaron actualizados. La ventana no se comprobó en esta tarea.
