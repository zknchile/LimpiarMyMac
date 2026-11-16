# PLAN-0017 — Paleta de día lluvioso

- **Fecha:** 2026-11-16
- **Estado:** completado
- **Solicitud:** cambiar el estilo a `#0F172A`, `#1E293B`, `#475569`, `#94A3B8` y `#E2E8F0`. El tono más oscuro va en títulos, el gris medio en texto secundario y divisores, y el fondo claro separa las secciones sin cajas pesadas. Añadir un grado de transparencia a la ventana.
- **Tareas:** [TASKS-0017](TASKS-0017-paleta-dia-lluvioso.md)

Este documento no cambia la app al escribirse. La ventana vigente es Ocean Glass, según [PLAN-0016](PLAN-0016-paleta-ocean-glass.md). La persona dijo que se ve muy mal.

## Objetivo, alcance y criterios de aceptación

Sustituir esa paleta por estos cinco grises y dejar ver un poco lo que hay detrás de la ventana. No es vidrio nativo.

Criterios observables:

- El fondo es `#E2E8F0` al 88 % (`#E2E8F0E0`). La ventana GPUI se abre con `WindowBackgroundAppearance::Transparent`, para que ese alfa se vea. No usa `NSVisualEffectView` ni `NSGlassEffectView`.
- El título es `#0F172A`. El texto es `#1E293B`, a 11.87:1 sobre el fondo opaco. El secundario es `#475569`, a 6.15:1.
- `#94A3B8` solo separa. Sobre el fondo es 2.08:1, así que no es texto.
- El botón principal es `#0F172A` con texto `#E2E8F0`, a 14.48:1. No se inventa un sexto color de acento. El botón secundario no lleva un relleno distinto: lo marca el borde `#475569`.
- La barra de abajo usa el mismo velo que el fondo. No hay una franja de otro color. Salir sigue abajo a la derecha de Analizar.
- El diseño aplicado niega que esta transparencia cumpla [REQ-0003](../requirements/baseline.md).

Fuera de alcance: Liquid Glass, mover Salir, cambiar `safety.rs`, la versión, la release y las pruebas. No se empuja salvo que se pida.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `src-tauri/src/view.rs`, `src-tauri/src/main.rs`, [diseño aplicado](../design/liquid-glass.md) y `WindowBackgroundAppearance` de GPUI 1.28.1. En macOS, `Transparent` pone la ventana no opaca. `Blurred` inserta un `NSVisualEffectView`. El contraste de las cinco parejas se calculó antes de fijar el reparto.

Hoy el fondo es `#ECFEFF`, el sello es `#22D3EE` y la barra es `#A5F3FC`.

No hay ADR: no cambia el motor. No hay RD: la persona no retiró Liquid Glass; pidió estos colores y un grado de transparencia.

Supuesto: «un grado» es un velo del 88 %, no un desenfoque. No hay pregunta abierta.

## Diseño, responsabilidades, contratos y alternativas

- Las constantes pasan a tinta, pizarra, ceniza, pelo y nube. El velo es `rgba`, no `rgb`.
- Tinta `#0F172A` en el título y en el botón principal. Pizarra `#1E293B` en el texto y en la categoría abierta, con texto nube. Ceniza `#475569` en lo secundario y en el borde del botón que no es principal. Pelo `#94A3B8` en las líneas. Nube `#E2E8F0` en el fondo, al 88 %.
- `main.rs` pide el fondo transparente de GPUI. La barra de título del sistema se queda: no se oculta para pintar otra.
- Alternativa rechazada: `WindowBackgroundAppearance::Blurred`. Eso es `NSVisualEffectView`, no el grado de alfa pedido, y tampoco es `NSGlassEffectView`.
- Alternativa rechazada: un acento brillante fuera de los cinco hex. El pedido no dio ese color, y el cian anterior es lo que se vio mal.
- Alternativa rechazada: usar `#94A3B8` como texto. No se lee.

## Pasos y entregables

1. Guardar este plan y su listado — [TASK-0017-01](TASKS-0017-paleta-dia-lluvioso.md#task-0017-01--dejar-el-plan).
2. Aplicar la paleta y el velo — [TASK-0017-02](TASKS-0017-paleta-dia-lluvioso.md#task-0017-02--aplicar-la-paleta). Depende del paso 1.

## Riesgos, seguridad y bloqueos

- Riesgo: leer el velo como Liquid Glass. Mitigación: el diseño nombra `Transparent` y niega `NSGlassEffectView`.
- Riesgo: el alfa no se ve porque la vista sigue opaca. Mitigación: el fondo de la vista usa el mismo velo, no un `rgb` sólido.
- No se toca `safety.rs`. No hay secretos. No hay bloqueo.

## Validación y evidencia esperada

No hay aserción nueva. No se ficha un caso. `cargo test --lib` sigue cubriendo rutas, tamaño y sitio de la grilla. La ventana se abre titulada Cristal y se mira el velo; eso se anota en la tarea, sin ID. `git diff --check` no reporta espacios conflictivos.

## Entrega, documentación, decisiones y recuperación

Diseño aplicado, changelog, `AGENTS.md` y el párrafo de planes del README. Sin ADR, RD ni release. Recuperación: revertir el cambio. No se sube en este plan.

## Seguimiento y resultados

Completado el 2026-11-16. La vista usa solo los cinco hex. El fondo del buffer queda en alfa 0.88. `cargo test --lib` pasó 14 pruebas y 0 fallos. No cambió aserciones, así que no hay ficha nueva. La ventana se listó como `title=Cristal`. El título es tinta, el botón principal es tinta con texto nube y Salir sigue a su derecha. No se movió ningún archivo. No se empujó.
