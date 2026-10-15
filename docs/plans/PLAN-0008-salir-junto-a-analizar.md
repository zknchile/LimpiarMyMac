# PLAN-0008 — Dejar Salir siempre visible junto a Analizar

- **Fecha:** 2026-10-15
- **Estado:** completado
- **Solicitud:** mover Salir a la esquina inferior derecha, a la derecha de Analizar, y dejarlo siempre visible.
- **Tareas:** [TASKS-0008](TASKS-0008-salir-junto-a-analizar.md)

## Objetivo, alcance y criterios de aceptación

Salir cierra la app desde cualquier pantalla. No se cambia qué se analiza ni qué se mueve a la Papelera.

Criterios observables:

- En inicio, análisis, resultados, limpieza, cierre y la hoja de confirmación, Salir está en la esquina inferior derecha.
- A su izquierda está Analizar. Antes del primer resultado dice «Analizar esta Mac»; después, «Analizar de nuevo».
- Durante el análisis o la limpieza, Analizar no se puede pulsar otra vez. Salir sí.
- No hay un segundo Salir arriba, ni un segundo Analizar en el centro o en la barra de título.
- La hoja de confirmación no tapa esos botones.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `index.html`, `src/main.js`, `src/styles.css`, el diseño aplicado y [REQ-0004](../requirements/baseline.md). Hoy Salir vive en la barra de título, a la derecha. Analizar está en el centro del inicio, otra vez arriba si ya hay un informe, y otra vez en el cierre. No hay pregunta abierta. El cambio de sitio es un requisito anterior identificable: se registra en [RD-0002](../decisions/RD-0002-salir-abajo-a-la-derecha.md). No es un ADR: no cambia la arquitectura.

## Diseño, responsabilidades, contratos y alternativas

- Una barra al pie de la ventana, fuera del contenido, alinea Analizar y Salir a la derecha. Así no se montan sobre «Revisar y limpiar».
- La hoja de confirmación cubre solo el contenido, no esa barra.
- Alternativa rechazada: dejar Salir flotando encima del pie de resultados. Taparía el botón de limpiar.
- Alternativa rechazada: un solo botón Salir sin Analizar al lado. No cumple el pedido.

## Pasos y entregables

1. Guardar este plan — [TASK-0008-01](TASKS-0008-salir-junto-a-analizar.md#task-0008-01--dejar-el-plan).
2. Mover los botones — [TASK-0008-02](TASKS-0008-salir-junto-a-analizar.md#task-0008-02--mover-los-botones).
3. Registrar el requisito — [TASK-0008-03](TASKS-0008-salir-junto-a-analizar.md#task-0008-03--registrar).

## Riesgos, seguridad y bloqueos

Salir durante una limpieza cierra el proceso. Eso ya ocurría con el botón de arriba. No se amplía la lista blanca. No hay bloqueo.

## Validación y evidencia esperada

Revisión visual en la ventana de desarrollo. No hay prueba automática de la interfaz. `npm test` no cambia.

## Entrega, documentación, decisiones y recuperación

Actualizar requisitos, RD-0002, README y changelog. Recuperación: volver a poner Salir en la barra de título.

## Seguimiento y resultados

Completado el 2026-10-15. Salir y Analizar viven en el pie de la ventana. La hoja de confirmación no los cubre. No se publicó.
