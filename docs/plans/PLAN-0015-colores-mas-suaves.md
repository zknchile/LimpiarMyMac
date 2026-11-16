# PLAN-0015 — Colores más suaves

- **Fecha:** 2026-11-16
- **Estado:** completado
- **Solicitud:** mejorar la interfaz con un estilo de colores más suaves.
- **Tareas:** [TASKS-0015](TASKS-0015-colores-mas-suaves.md)

Este documento no cambia la app al escribirse. La ventana vigente es GPUI y plana, según [PLAN-0012](PLAN-0012-migrar-vista-a-gpui.md).

## Objetivo, alcance y criterios de aceptación

Suavizar la paleta de `src-tauri/src/view.rs`. No es vidrio, no mueve Salir y no cambia qué se puede limpiar.

Criterios observables:

- El botón principal deja de ser `1d4ed8`. Pasa a un azul apagado, `45687a`, con texto crema `f6f3ee`.
- El fondo deja el blanco cálido `f6f5f2` por un papel `f4f1eb`. La tinta deja el casi negro `1c1c1c` por `3d3933`.
- Las líneas pasan de `e4e2dc` a `e3dcd2`. La categoría abierta pasa de `e7e5de` a `e8e1d6`.
- El botón secundario deja de ser blanco puro. Usa `faf7f2`. El desactivado usa `ddd6cc` y el borde del botón `d5cdc2`.
- Salir sigue abajo a la derecha de Analizar.
- El texto sobre el papel y sobre el botón principal sigue leyéndose. La tinta sobre el papel queda por encima de 10:1. El crema sobre `45687a` queda por encima de 5:1.
- El diseño aplicado nombra estos colores y dice que no son `NSGlassEffectView` ni cumplen [REQ-0003](../requirements/baseline.md).

Fuera de alcance: Liquid Glass, mover Salir, cambiar `safety.rs`, la versión, la release y las pruebas. No se empuja salvo que se pida.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `src-tauri/src/view.rs` y [diseño aplicado](../design/liquid-glass.md).

Los colores están sueltos en el render, el muelle, la barra de resultados, la navegación, el resumen, la fila y `button`. El fondo es `f6f5f2`, la tinta `1c1c1c`, la línea `e4e2dc`, la categoría abierta `e7e5de`, el principal `1d4ed8` con texto blanco, el secundario blanco y el desactivado `d6d3cb`. No hay color de riesgo. No hay hover.

No hay ADR: no cambia el motor. No hay RD: la persona no retiró Liquid Glass; pidió colores más suaves sobre la ventana plana.

Supuesto: suavizar significa menos saturación y menos contraste duro, no transparencia. No hay pregunta abierta.

## Diseño, responsabilidades, contratos y alternativas

- La paleta vive en constantes al inicio de `view.rs`. El resto de la vista las usa. Así un color no queda a medias.
- Papel `f4f1eb`, tinta `3d3933`, línea `e3dcd2`, lavado `e8e1d6`, acción `45687a`, texto de la acción `f6f3ee`, crema `faf7f2`, apagado `ddd6cc`, borde `d5cdc2`.
- `45687a` es más oscuro que un azul pastel para que el texto crema siga por encima de 5:1. Lo suave está en la saturación, no en un botón claro ilegible.
- Alternativa rechazada: un azul pastel con texto blanco. El contraste cae de 4.5:1.
- Alternativa rechazada: atribuir el cambio a Liquid Glass. La ventana sigue opaca y sin `NSGlassEffectView`.

## Pasos y entregables

1. Guardar este plan y su listado — [TASK-0015-01](TASKS-0015-colores-mas-suaves.md#task-0015-01--dejar-el-plan).
2. Aplicar la paleta y anotarla — [TASK-0015-02](TASKS-0015-colores-mas-suaves.md#task-0015-02--aplicar-la-paleta). Depende del paso 1.

## Riesgos, seguridad y bloqueos

- Riesgo: un azul tan claro que el botón no se lea. Mitigación: el contraste del texto crema sobre `45687a` se comprobó por encima de 5:1 antes de fijar el color.
- Riesgo: presentar esto como REQ-0003 cumplido. Mitigación: el diseño lo niega.
- No se toca `safety.rs`. No hay secretos. No hay bloqueo.

## Validación y evidencia esperada

No hay aserción nueva. No se ficha un caso. `cargo test --lib` sigue cubriendo rutas, tamaño y sitio de la grilla. La ventana se abre para ver el papel y el botón; eso se anota en la tarea, sin ID. `git diff --check` no reporta espacios conflictivos.

## Entrega, documentación, decisiones y recuperación

Diseño aplicado, changelog, `AGENTS.md` y el párrafo de planes del README. Sin ADR, RD ni release. Recuperación: revertir el cambio. No se sube en este plan.

## Seguimiento y resultados

Completado el 2026-11-16. La paleta quedó en constantes de `src-tauri/src/view.rs`. No queda `1d4ed8` ni `1c1c1c`. `cargo test --lib` pasó 14 pruebas y 0 fallos. No cambió aserciones, así que no hay ficha nueva. `cargo run` abrió una ventana que CoreGraphics listó como `title=Cristal`. El papel es cálido, el botón principal es el azul apagado y Salir sigue a su derecha. No se empujó.
