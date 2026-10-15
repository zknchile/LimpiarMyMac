# PLAN-0003 — Actualizar el tamaño al cambiar la selección

- **Fecha:** 2026-10-15
- **Estado:** completado
- **Solicitud:** durante la prueba, la persona vio selecciones por defecto, pero al quitar o agregar elementos el tamaño de archivos a borrar no cambiaba.
- **Tareas:** [TASKS-0003](TASKS-0003-actualizar-tamano-seleccion.md)

## Objetivo, alcance y criterios de aceptación

El número de lo que se va a limpiar debe cambiar en cuanto cambia la selección. No se cambia qué archivos se pueden mover ni las reglas de `safety.rs`.

Criterios observables:

- Desmarcar un elemento baja el tamaño seleccionado de esa categoría, el del resumen y el del pie. Si no queda nada marcado, se muestra 0, no el total encontrado.
- Marcar un elemento que no venía seleccionado sube esos mismos números.
- Un cambio de decenas de MB sobre un total en GB se nota en el texto. No basta con que el estado interno cambie y el rótulo se quede igual por el redondeo.
- La limpieza sigue usando los identificadores seleccionados. Este plan no amplía la lista blanca.

Hecho comprobado en `src/main.js`: el pie y el resumen ya leen `selectedBytes()`, pero la barra lateral muestra el total de la categoría y, si la selección de una categoría suma 0, la fila usa el total porque `0` es falso en `bytes || total`. `formatBytes` quita los decimales a partir de 10 unidades, así que un total grande no se mueve al quitar un archivo mediano.

Supuesto: la persona miraba esos rótulos mientras marcaba casillas. No hay otra pregunta abierta.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `src/main.js`, `src/styles.css`, `src-tauri/src/model.rs`, [REQ-0002](../requirements/baseline.md), [pruebas](../testing/TESTING.md) y el plan vigente de documentación. No hay ADR que fije el redondeo. No corresponde un ADR ni un RD: el requisito de decidir después del análisis no cambia; falla la presentación de esa decisión.

Rust sigue sin responsabilidad de IA. No hay contrato HTTP.

## Diseño, responsabilidades, contratos y alternativas

- `src/selection.js` calcula el texto del tamaño y el total seleccionado. La interfaz solo lo muestra. Así se puede probar sin Tauri.
- La barra y las filas del resumen muestran lo seleccionado y, como contexto, el total encontrado: `4.2 GB de 6.1 GB`.
- El encabezado de la categoría muestra el tamaño seleccionado de esa lista, junto a las casillas.
- Alternativa rechazada: solo corregir el `||`. Dejaría la barra lateral quieta, que es el número visible mientras se marcan casillas.
- Alternativa rechazada: mostrar siempre bytes exactos. El texto sería largo y no ayuda a decidir.

## Pasos y entregables

1. Guardar este plan y su listado — [TASK-0003-01](TASKS-0003-actualizar-tamano-seleccion.md#task-0003-01--dejar-el-plan).
2. Corregir el cálculo y los rótulos — [TASK-0003-02](TASKS-0003-actualizar-tamano-seleccion.md#task-0003-02--mostrar-el-tamaño-seleccionado).
3. Probar el cálculo y anotar el cambio — [TASK-0003-03](TASKS-0003-actualizar-tamano-seleccion.md#task-0003-03--verificar-y-registrar).

## Riesgos, seguridad y bloqueos

No se mueven archivos en esta corrección. El riesgo es mostrar 0 cuando la persona esperaba el total encontrado. Se mitiga dejando el total al lado, con la preposición «de». No hay bloqueo.

## Validación y evidencia esperada

`node --test src/selection.test.js` en el host. No aplica Docker ni `cargo test`: el fallo está en la presentación, no en la lista blanca. Fallo: un total en GB no cambia al restar 80 MB, o una selección vacía no da 0 B.

## Entrega, documentación, decisiones y recuperación

Actualizar [CHANGELOG.md](../../CHANGELOG.md) y [pruebas](../testing/TESTING.md). No hay ADR, RD ni migración. Recuperación: revertir el commit de esta corrección. La app de desarrollo hay que volver a abrirla para ver el cambio.

## Seguimiento y resultados

Completado el 2026-10-15. `node --test src/selection.test.js` pasó 3 pruebas. La barra, el resumen y el encabezado de cada categoría muestran el tamaño seleccionado; una selección vacía queda en 0 B. Limitación: la persona usuaria aún no confirmó el cambio en la ventana. Un ajuste menor de 10 MB sobre un total en GB puede seguir redondeándose al mismo texto.
