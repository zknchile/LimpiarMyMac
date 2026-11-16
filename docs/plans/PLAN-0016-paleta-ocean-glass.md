# PLAN-0016 — Paleta Ocean Glass

- **Fecha:** 2026-11-16
- **Estado:** completado
- **Solicitud:** reformular el estilo con los colores `#083344`, `#0E7490`, `#22D3EE`, `#A5F3FC` y `#ECFEFF`. El texto importante va en el verde azulado profundo, los fondos en los tonos helados y el cian vivo en sellos y resaltes. El contraste se mantiene nítido y los márgenes, generosos.
- **Tareas:** [TASKS-0016](TASKS-0016-paleta-ocean-glass.md)

Este documento no cambia la app al escribirse. La ventana vigente es la paleta cálida de [PLAN-0015](PLAN-0015-colores-mas-suaves.md). Ocean Glass es un nombre de paleta. No es `NSGlassEffectView` ni cumple [REQ-0003](../requirements/baseline.md).

## Objetivo, alcance y criterios de aceptación

Sustituir la paleta plana por esos cinco colores, sin añadir un sexto y sin volver translúcida la ventana.

Criterios observables:

- El fondo es `#ECFEFF`. El texto es `#083344`. Esa pareja queda en 12.88:1.
- El botón principal es `#22D3EE` con texto `#083344`, a 7.41:1. El borde es `#0E7490`, para que el sello no se funda con el fondo: el cian sobre `#ECFEFF` es solo 1.74:1.
- El botón secundario y la barra de abajo usan `#A5F3FC`, con texto `#083344` a 10.74:1. `#A5F3FC` sobre `#ECFEFF` es 1.20:1, así que no sirve para marcar la categoría abierta. Esa marca es `#22D3EE`.
- Las líneas de separación son `#0E7490`. Sobre el fondo quedan en 5.15:1.
- El desactivado sigue en `#A5F3FC`, con texto `#0E7490` y la opacidad que ya tenía.
- Las pantallas de texto pasan de `p_8` a `p_12` y de `gap_2` a `gap_4`. Los botones pasan a `px_4` y `py_2p5`. Salir sigue abajo a la derecha de Analizar.
- El diseño aplicado lista estos cinco colores y niega el vidrio nativo.

Fuera de alcance: Liquid Glass, mover Salir, cambiar `safety.rs`, la versión, la release y las pruebas. No se empuja salvo que se pida.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `src-tauri/src/view.rs`, [diseño aplicado](../design/liquid-glass.md) y el contraste de las cinco parejas, calculado antes de fijar el reparto.

Hoy el fondo es `#f4f1eb`, la tinta `#3d3933` y el botón `#45687a`. No hay un color de riesgo ni hover.

No hay ADR: no cambia el motor. No hay RD: la persona no retiró Liquid Glass; pidió estos colores sobre la ventana plana.

Supuesto: los cinco hex son cerrados. Un gris de desactivado o un rojo de error quedarían fuera. No hay pregunta abierta.

## Diseño, responsabilidades, contratos y alternativas

- Las constantes de `view.rs` pasan a llamarse por el papel que cumplen: bruma, profundo, medio, vivo y hielo.
- Bruma `#ECFEFF` en el lienzo. Hielo `#A5F3FC` en el muelle, el pie de resultados y el botón secundario. Profundo `#083344` en todo el texto legible, también sobre el sello. Vivo `#22D3EE` en el botón principal y en la categoría abierta. Medio `#0E7490` en bordes y líneas.
- Alternativa rechazada: botón principal `#0E7490` con texto helado. Cumple 5.15:1, pero el pedido reserva el cian vivo para el sello y el profundo para el texto.
- Alternativa rechazada: categoría abierta en `#A5F3FC`. No se distingue del fondo.
- Alternativa rechazada: llamar a esto Liquid Glass. El fondo sigue opaco.

## Pasos y entregables

1. Guardar este plan y su listado — [TASK-0016-01](TASKS-0016-paleta-ocean-glass.md#task-0016-01--dejar-el-plan).
2. Aplicar la paleta y anotarla — [TASK-0016-02](TASKS-0016-paleta-ocean-glass.md#task-0016-02--aplicar-la-paleta). Depende del paso 1.

## Riesgos, seguridad y bloqueos

- Riesgo: leer «Ocean Glass» como el vidrio de Apple. Mitigación: el diseño lo niega.
- Riesgo: un sello cian ilegible. Mitigación: el texto profundo sobre `#22D3EE` queda en 7.41:1.
- No se toca `safety.rs`. No hay secretos. No hay bloqueo.

## Validación y evidencia esperada

No hay aserción nueva. No se ficha un caso. `cargo test --lib` sigue cubriendo rutas, tamaño y sitio de la grilla. La ventana se abre para ver el fondo helado y el sello cian; eso se anota en la tarea, sin ID. `git diff --check` no reporta espacios conflictivos.

## Entrega, documentación, decisiones y recuperación

Diseño aplicado, changelog, `AGENTS.md` y el párrafo de planes del README. Sin ADR, RD ni release. Recuperación: revertir el cambio. No se sube en este plan.

## Seguimiento y resultados

Completado el 2026-11-16. La vista usa solo los cinco hex. `cargo test --lib` pasó 14 pruebas y 0 fallos. No cambió aserciones, así que no hay ficha nueva. La ventana se listó como `title=Cristal`. El fondo es bruma, el texto es profundo, el sello es cian y Salir sigue a su derecha sobre la barra de hielo. No se movió ningún archivo. No se empujó.
