# PLAN-0018 — Profundidad de los botones

- **Fecha:** 2026-11-16
- **Estado:** completado
- **Solicitud:** dar profundidad a los botones.
- **Tareas:** [TASKS-0018](TASKS-0018-profundidad-de-botones.md)

Este documento no cambia la app al escribirse. La ventana vigente es la paleta de día lluvioso, según [PLAN-0017](PLAN-0017-paleta-dia-lluvioso.md). Los botones son planos: un borde de 1 px y ningún `box_shadow`.

## Objetivo, alcance y criterios de aceptación

Que un botón se lea como una pieza que descansa sobre el velo, no como una mancha del mismo plano. La profundidad sale de la sombra de GPUI. No es vidrio.

Criterios observables:

- El botón principal habilitado lleva una sombra exterior de tinta `#0F172A` al 32 %, desplazada 3 px hacia abajo y desenfocada 8 px, más una luz interior de nube `#E2E8F0` al 22 % en el borde superior. El relleno sigue siendo tinta y el texto, nube.
- El botón que no es principal, si está habilitado, lleva una sombra más corta: tinta al 18 %, 1 px abajo, desenfoque 4 px. El relleno sigue siendo el velo y el borde, ceniza `#475569`.
- El botón deshabilitado no lleva sombra. Sigue al 55 % de opacidad.
- Al pasar el puntero, la sombra baja un poco más. Al pulsar, la exterior desaparece y queda una sombra interior de tinta al 20 %. El botón no cambia de sitio.
- No se inventa un sexto hex. El alfa sale de tinta y nube, que ya están en la paleta. No es `NSGlassEffectView` ni `NSVisualEffectView`.
- Salir sigue abajo a la derecha de Analizar. La lista de categorías no es un botón de acción y no lleva esta sombra.

Fuera de alcance: Liquid Glass, mover Salir, cambiar `safety.rs`, la versión, la release y las pruebas. No se empuja salvo que se pida.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `button()` en `src-tauri/src/view.rs`, [diseño aplicado](../design/liquid-glass.md) y `box_shadow_style_methods` de GPUI 1.28.1. Esa macro expone `shadow`, `shadow_sm` y `shadow_md`. `BoxShadow::new` acepta color, desplazamiento, desenfoque, expansión e interior.

Hoy el botón principal es `#0F172A` con borde del mismo color. El secundario es el velo `#E2E8F0` al 88 % con borde `#475569`. Ninguno llama a `shadow`.

No hay ADR: no cambia el motor. No hay RD: no se retiró ni se reinterpretó un requisito. [REQ-0003](../requirements/baseline.md) sigue pedido y sin cumplir.

Supuesto: «profundidad» es una sombra corta, no un desenfoque de ventana ni un material nuevo. No hay pregunta abierta.

## Diseño, responsabilidades, contratos y alternativas

- `button()` arma la sombra. El resto de la vista no la repite. La barra y el sitio de Salir no cambian.
- La luz viene de arriba, como un día nublado. El desplazamiento es solo vertical. El botón principal es el que más se separa del velo; el otro descansa más cerca.
- `hover` y `active` de GPUI cambian solo la sombra. No cambian el relleno, así que el contraste del texto se queda.
- Alternativa rechazada: `shadow_sm` y `shadow_md` tal cual. Usan negro al 10 %. Sobre este velo casi no se leen, y la profundidad quedaría en un negro que la paleta no nombra.
- Alternativa rechazada: un bisel de otro color, o un acento brillante. Sería un sexto hex, y el pedido anterior no lo dio.
- Alternativa rechazada: `WindowBackgroundAppearance::Blurred` o `NSGlassEffectView`. Eso no es profundidad de un botón, y afirmaría un vidrio que esta ventana no tiene.

## Pasos y entregables

1. Guardar este plan y su listado — [TASK-0018-01](TASKS-0018-profundidad-de-botones.md#task-0018-01--dejar-el-plan).
2. Pintar la sombra y anotarla — [TASK-0018-02](TASKS-0018-profundidad-de-botones.md#task-0018-02--pintar-la-sombra). Depende del paso 1.

## Riesgos, seguridad y bloqueos

- Riesgo: leer la sombra como Liquid Glass. Mitigación: el diseño dice que es `BoxShadow` de GPUI y niega `NSGlassEffectView`.
- Riesgo: la sombra se corta en el borde de la ventana. Mitigación: el desplazamiento en reposo es de 1 o 3 px, y la barra ya tiene 16 px de relleno.
- No se toca `safety.rs`. No hay secretos. No hay bloqueo.

## Validación y evidencia esperada

No hay aserción nueva. No se ficha un caso. `cargo test --lib` sigue cubriendo rutas, tamaño y sitio de la grilla. La ventana se abre titulada Cristal, con el puntero fuera de Analizar, y se mira que el botón principal no es plano. Eso se anota en la tarea, sin ID. No se pulsa «Mover a la Papelera». `git diff --check` no reporta espacios conflictivos.

## Entrega, documentación, decisiones y recuperación

Diseño aplicado, changelog, `AGENTS.md` y el párrafo de planes del README. Sin ADR, RD ni release. Recuperación: revertir el cambio. No se sube en este plan.

## Seguimiento y resultados

Completado el 2026-11-16. `button()` pinta la sombra de tinta. El deshabilitado no la lleva. `cargo test --lib` pasó 14 pruebas y 0 fallos. No cambió aserciones, así que no hay ficha nueva. `git diff --check` no reportó espacios conflictivos. La ventana se listó como `title=Cristal`. El inicio muestra Analizar a la izquierda de Salir. Bajo el botón principal, un pixel salió en 177,182,191; el de la barra, lejos de los botones, en 228,233,239. No se pulsó «Mover a la Papelera» y no se movió ningún archivo. No se empujó.
