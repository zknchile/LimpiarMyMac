# Liquid Glass aplicado en Cristal

La referencia normativa de adopción es la del marco: [Liquid Glass](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/design/liquid-glass.md). Esa guía es para una SPA. Aquí se anota lo que la ventana de Cristal hace de verdad, y dónde no sigue la pauta.

## Hecho comprobado

La ventana que abre `cargo run` es GPUI. No usa `NSGlassEffectView`, ni `window-vibrancy`, ni CSS. No cumple [REQ-0003](../requirements/baseline.md). Ese requisito no se reescribe: la persona no aceptó esta ventana como Liquid Glass. El detalle está en [ADR-0003](../decisions/ADR-0003-vista-gpui.md).

La vista, en `src-tauri/src/view.rs`, es texto plano: inicio, análisis, resultados, confirmación, limpieza y cierre. Salir está abajo a la derecha de Analizar. No hay cápsula, orbe, anillo animado ni hoja translúcida. No se consulta `prefers-reduced-transparency`.

El 2026-10-15 la persona vio otra ventana, la de Tauri, y dijo que quedó muy buena. Esa ventana aplicaba `apply_liquid_glass` y ya no es la que pinta el binario. [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md) quedó sustituido.

## Color plano

Desde [PLAN-0017](../plans/PLAN-0017-paleta-dia-lluvioso.md) la paleta es la de día lluvioso. Ocean Glass, de [PLAN-0016](../plans/PLAN-0016-paleta-ocean-glass.md), quedó sustituida. No es `NSGlassEffectView` y no cumple [REQ-0003](../requirements/baseline.md).

| Uso | Color |
| --- | --- |
| Fondo, al 88 % | `#E2E8F0` |
| Título y botón principal | `#0F172A` |
| Texto | `#1E293B` |
| Texto secundario y borde de botón | `#475569` |
| Línea | `#94A3B8` |

`main.rs` pide `WindowBackgroundAppearance::Transparent`. Eso deja ver un poco lo de detrás. No es `NSVisualEffectView` ni el vidrio de Apple. `#94A3B8` sobre el fondo es 2.08:1, así que no se usa como texto. Salir no se movió.

## Icono

Desde [PLAN-0021](../plans/PLAN-0021-icono-de-cristal.md) el Dock y el `.app` usan un cristal plano. El fondo es `#0F172A`. Las facetas son `#E2E8F0`, `#D2DAE5` y `#94A3B8`. No hay brillo ni sombra. No es la gota de Tauri, no es `NSGlassEffectView` y no cumple [REQ-0003](../requirements/baseline.md). `cargo run` lo asigna con `setApplicationIconImage`, porque el binario suelto no lee `CFBundleIconFile`. El `.app` copia `src-tauri/icons/icon.icns`.

Desde [PLAN-0019](../plans/PLAN-0019-franja-del-menu.md) la franja del menú, de 32 pt, es `#D2DAE5` al 88 %. Sale de la nube y del pelo, no de un sexto acento. El título lo pinta la vista porque GPUI oculta el del sistema al pedir la barra transparente. No es vidrio.

## Botones

Desde [PLAN-0018](../plans/PLAN-0018-profundidad-de-botones.md) un botón habilitado lleva sombra de GPUI. No es `NSGlassEffectView`.

El principal descansa 3 px bajo una sombra de tinta al 32 %, con una luz de nube al 22 % en el borde superior. El otro descansa 1 px, con tinta al 18 %. El deshabilitado no lleva sombra. Al pasar el puntero la sombra baja un poco; al pulsar queda solo una sombra interior. No hay un sexto color.

## Desviaciones aceptadas

| Pauta del marco | Qué hace Cristal | Motivo |
|---|---|---|
| Vidrio nativo en navegación y controles. | La ventana GPUI no aplica `NSGlassEffectView`. | [PLAN-0012](../plans/PLAN-0012-migrar-vista-a-gpui.md) aceptó perder lo visual. No afirma [REQ-0003](../requirements/baseline.md). |
| Texto sobre superficie convencional. | El texto va sobre un fondo plano. | No hay material de vidrio que medir. No se midió WCAG. |
| Superficie opaca si se reduce la transparencia. | No está implementado. | No hay material nativo al que aplicar esa pauta. |

Estas desviaciones no convierten la ventana actual en Liquid Glass. Un cambio para acercarse a la pauta del marco necesita su propio plan.

## Qué no afirma este documento

CSS no reproduce el material óptico de Apple. El efecto nativo sería el de `NSGlassEffectView`. Esta ventana no lo usa.
