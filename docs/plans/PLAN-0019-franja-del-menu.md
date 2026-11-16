# PLAN-0019 — Franja del menú un poco más oscura

- **Fecha:** 2026-11-16
- **Estado:** completado
- **Solicitud:** en el espacio del menú, poner un color un poco más oscuro.
- **Tareas:** [TASKS-0019](TASKS-0019-franja-del-menu.md)

Este documento no cambia la app al escribirse. La ventana vigente es la paleta de día lluvioso, según [PLAN-0017](PLAN-0017-paleta-dia-lluvioso.md). Los botones ya tienen sombra, según [PLAN-0018](PLAN-0018-profundidad-de-botones.md).

## Objetivo, alcance y criterios de aceptación

La franja de arriba, donde están los botones de la ventana y el título, deja de ser blanca. Queda un poco más oscura que el velo. No es vidrio.

Criterios observables:

- Esa franja mide 32 pt, que es lo que ocupa hoy el blanco del sistema. Su color es `#D2DAE5` al 88 % (`#D2DAE5E0`): nube `#E2E8F0` al 80 % y pelo `#94A3B8` al 20 %. Sobre el velo, que en el buffer sale cerca de 228,232,239, baja unos 18 puntos. No llega a ceniza ni a tinta.
- El título «Cristal» sigue en el centro, en tinta `#0F172A`. GPUI oculta el título del sistema cuando la barra es transparente, así que lo pinta la vista. El nombre de la ventana, para el menú y Mission Control, sigue siendo Cristal.
- Los botones rojo, amarillo y verde se quedan. Salir sigue abajo a la derecha de Analizar. El resto de la ventana no cambia de color.
- No es un sexto acento. Es una mezcla de dos hex que ya están. No es `NSGlassEffectView` ni `NSVisualEffectView`.

Fuera de alcance: Liquid Glass, mover Salir, cambiar `safety.rs`, la versión, la release y las pruebas. No se empuja salvo que se pida.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `src-tauri/src/main.rs`, `src-tauri/src/view.rs`, [diseño aplicado](../design/liquid-glass.md) y `TitlebarOptions` de GPUI 1.28.1. En macOS, `appears_transparent` pone `FullSizeContentView`, `setTitlebarAppearsTransparent` y oculta el título del sistema.

Una captura de la ventana abierta midió la franja blanca: de la fila 0 a la 63, a 2×, es 255,255,255. La fila 64 ya es el velo. El botón amarillo cae dentro de ese blanco.

Hoy la barra de título no es transparente. El contenido empieza debajo.

No hay ADR: no cambia el motor. No hay RD: no se retiró un requisito. [REQ-0003](../requirements/baseline.md) sigue pedido y sin cumplir.

Supuesto: «el espacio del menú» es esa franja, no la barra de menú de todo el escritorio. «Un poco» es esa mezcla, no un encabezado de tinta. No hay pregunta abierta.

## Diseño, responsabilidades, contratos y alternativas

- `main.rs` pide la barra transparente. No pide que la app arrastre la ventana: `app_owns_titlebar_drag` sigue en falso, y AppKit conserva el arrastre.
- La vista pinta la franja arriba, de 32 pt, y el título centrado. El cuerpo y la barra de abajo no se mueven.
- Alternativa rechazada: dejar el título del sistema y pintar encima. GPUI no deja la barra transparente sin ocultar ese título.
- Alternativa rechazada: usar `#94A3B8` o `#475569` como relleno. No es «un poco»: el pelo sobre la nube ya es una línea, y la ceniza es texto.
- Alternativa rechazada: oscurecer la barra de menú de macOS. Eso cambia el escritorio, no esta ventana.
- Alternativa rechazada: `WindowBackgroundAppearance::Blurred`. Eso es `NSVisualEffectView`, no un color.

## Pasos y entregables

1. Guardar este plan y su listado — [TASK-0019-01](TASKS-0019-franja-del-menu.md#task-0019-01--dejar-el-plan).
2. Oscurecer la franja y anotarla — [TASK-0019-02](TASKS-0019-franja-del-menu.md#task-0019-02--oscurecer-la-franja). Depende del paso 1.

## Riesgos, seguridad y bloqueos

- Riesgo: leer la franja como Liquid Glass. Mitigación: el diseño nombra la mezcla y niega `NSGlassEffectView`.
- Riesgo: el título se pierde porque GPUI lo oculta. Mitigación: la vista lo pinta en el centro, en tinta.
- Riesgo: 32 pt no cubre la franja en otra versión de macOS. Mitigación: la medida es de esta Mac, el 2026-11-16, y se comprueba en la captura.
- No se toca `safety.rs`. No hay secretos. No hay bloqueo.

## Validación y evidencia esperada

No hay aserción nueva. No se ficha un caso. `cargo test --lib` sigue cubriendo rutas, tamaño y sitio de la grilla. La ventana se abre titulada Cristal, con el puntero fuera de Analizar, y se mira que la franja no es blanca. Eso se anota en la tarea, sin ID. No se pulsa «Mover a la Papelera». `git diff --check` no reporta espacios conflictivos.

## Entrega, documentación, decisiones y recuperación

Diseño aplicado, changelog, `AGENTS.md` y el párrafo de planes del README. Sin ADR, RD ni release. Recuperación: revertir el cambio. No se sube en este plan.

## Seguimiento y resultados

Completado el 2026-11-16. La franja deja el blanco. Un pixel del centro salió en 213,219,230; el del velo, en 228,232,239. `cargo test --lib` pasó 14 pruebas y 0 fallos. No cambió aserciones, así que no hay ficha nueva. `git diff --check` no reportó espacios conflictivos. La ventana se listó como `title=Cristal`. El título sigue en el centro, los botones de la ventana se ven, y Salir sigue a la derecha de Analizar. No se pulsó «Mover a la Papelera» y no se movió ningún archivo. No se empujó.
