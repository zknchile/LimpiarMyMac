# PLAN-0010 — Dejar la grilla donde se marcó

- **Fecha:** 2026-10-15
- **Estado:** completado
- **Solicitud:** al marcar un elemento de la grilla, la lista sube al inicio. Debe quedarse en el sitio del clic.
- **Tareas:** [TASKS-0010](TASKS-0010-grilla-sin-saltar.md)

## Objetivo, alcance y criterios de aceptación

Marcar o desmarcar no mueve la lista ni la barra lateral. No cambia qué se puede limpiar ni el tamaño que se muestra.

Criterios observables:

- En la misma categoría, la lista conserva el desplazamiento de antes del clic. La fila marcada sigue a la vista.
- La barra lateral tampoco vuelve arriba al actualizar los tamaños.
- Abrir otra categoría empieza su lista arriba. La barra lateral se queda donde estaba, para que el botón pulsado no desaparezca.
- Salir de resultados, analizar o limpiar no reutiliza ese desplazamiento.
- La hoja de confirmación no le quita el foco a sus propios controles para devolverlo a la casilla.

Fuera de alcance: cambiar el diseño de la grilla, añadir selección al hacer clic en toda la fila, y probar el clic dentro de la ventana. Eso sigue sin ficha.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `src/main.js`, `src/styles.css`, [REQ-0002](../requirements/baseline.md) y [pruebas](../testing/TESTING.md). La lista con desplazamiento es `.list`. La barra es `.sidebar`. Ambas se crean de nuevo en cada `render()`, porque `view.innerHTML` sustituye el HTML. Una lista nueva nace con desplazamiento 0. Por eso el clic, que llama a `render()`, sube la grilla.

No hay ADR que fije el desplazamiento. No corresponde un ADR ni un RD: no cambia la limpieza ni un requisito escrito. Falla la presentación al marcar.

Supuesto: la persona marca la casilla de una fila ya desplazada, no un control de otra pantalla. No hay pregunta abierta.

## Diseño, responsabilidades, contratos y alternativas

- `src/place.js` decide qué desplazamiento se conserva. La ventana solo lo aplica. Así se prueba sin Tauri.
- Se guarda el desplazamiento antes de sustituir el HTML y se devuelve después, en la misma categoría. El foco vuelve a esa casilla con `preventScroll`, salvo que la hoja de confirmación esté abierta.
- Cambiar de categoría no copia el desplazamiento de la lista anterior.
- Alternativa rechazada: no redibujar la lista al marcar. Dejaría viejos los tamaños de la barra, del encabezado y del pie, que [PLAN-0003](PLAN-0003-actualizar-tamano-seleccion.md) ya exige actualizar.
- Alternativa rechazada: anclar siempre la fila marcada al borde superior. Eso también mueve la grilla.

## Pasos y entregables

1. Guardar este plan y su listado — [TASK-0010-01](TASKS-0010-grilla-sin-saltar.md#task-0010-01--dejar-el-plan).
2. Conservar el desplazamiento y fichar la prueba — [TASK-0010-02](TASKS-0010-grilla-sin-saltar.md#task-0010-02--conservar-el-desplazamiento).
3. Ejecutar la prueba, registrar la salida y cerrar — [TASK-0010-03](TASKS-0010-grilla-sin-saltar.md#task-0010-03--verificar-y-registrar). Depende del paso 2.

## Riesgos, seguridad y bloqueos

- Riesgo: devolver el foco a la casilla con la hoja abierta. Mitigación: si hay confirmación, no se enfoca la casilla.
- No se amplía la lista blanca. No hay secretos. No hay bloqueo.

## Validación y evidencia esperada

`npm test` en el host. La ficha nueva entra en [TEST-CASES.md](../testing/TEST-CASES.md) y la salida en [TEST-RUNS.md](../testing/TEST-RUNS.md). No cubre el clic en la ventana. Falla si, en la misma categoría, el desplazamiento guardado se pierde, o si al cambiar de categoría la lista nueva hereda el anterior.

## Entrega, documentación, decisiones y recuperación

Changelog, catálogo, registro de ejecución, README y `AGENTS.md`. Sin ADR, RD ni migración. Recuperación: revertir el commit. La ventana de desarrollo hay que volver a abrirla para verlo.

## Seguimiento y resultados

Completado el 2026-10-15. Marcar en la misma categoría conserva el desplazamiento; cambiar de categoría no se lo pasa a la lista nueva. [RUN-2026-10-15-02](../testing/TEST-RUNS.md#run-2026-10-15-02) aprobó las tres pruebas de `place` y las tres de selección. Limitación: no se midió el clic dentro de la ventana. La persona tiene que volver a abrir la app para verlo.
