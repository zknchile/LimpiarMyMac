# Liquid Glass aplicado en Cristal

La referencia normativa de adopción es la del marco: [Liquid Glass](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/design/liquid-glass.md). Esa guía es para una SPA. Aquí se anota lo que la ventana de Cristal hace de verdad, y dónde no sigue la pauta.

## Hecho comprobado

La ventana que abre `cargo run` es GPUI. No usa `NSGlassEffectView`, ni `window-vibrancy`, ni CSS. No cumple [REQ-0003](../requirements/baseline.md). Ese requisito no se reescribe: la persona no aceptó esta ventana como Liquid Glass. El detalle está en [ADR-0003](../decisions/ADR-0003-vista-gpui.md).

La vista, en `src-tauri/src/view.rs`, es texto plano: inicio, análisis, resultados, confirmación, limpieza y cierre. Salir está abajo a la derecha de Analizar. No hay cápsula, orbe, anillo animado ni hoja translúcida. No se midió contraste ni `prefers-reduced-transparency`.

El 2026-10-15 la persona vio otra ventana, la de Tauri, y dijo que quedó muy buena. Esa ventana aplicaba `apply_liquid_glass` y ya no es la que pinta el binario. [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md) quedó sustituido.

## Desviaciones aceptadas

| Pauta del marco | Qué hace Cristal | Motivo |
|---|---|---|
| Vidrio nativo en navegación y controles. | La ventana GPUI no aplica `NSGlassEffectView`. | [PLAN-0012](../plans/PLAN-0012-migrar-vista-a-gpui.md) aceptó perder lo visual. No afirma [REQ-0003](../requirements/baseline.md). |
| Texto sobre superficie convencional. | El texto va sobre un fondo plano. | No hay material de vidrio que medir. No se midió WCAG. |
| Superficie opaca si se reduce la transparencia. | No está implementado. | No hay material nativo al que aplicar esa pauta. |

Estas desviaciones no convierten la ventana actual en Liquid Glass. Un cambio para acercarse a la pauta del marco necesita su propio plan.

## Qué no afirma este documento

CSS no reproduce el material óptico de Apple. El efecto nativo sería el de `NSGlassEffectView`. Esta ventana no lo usa.
