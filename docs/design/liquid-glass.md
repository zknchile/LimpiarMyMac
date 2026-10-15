# Liquid Glass aplicado en Cristal

La referencia normativa de adopción es la del marco: [Liquid Glass](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/design/liquid-glass.md). Esa guía es para una SPA. Aquí se anota lo que la ventana de Cristal hace de verdad, y dónde no sigue la pauta.

## Hecho comprobado

En macOS 26 o posterior, `src-tauri/src/lib.rs` aplica `apply_liquid_glass` con estilo Regular, radio 26, opaco desactivado e interactivo. Si falla, usa vibrancy `HudWindow`. La ventana Tauri es transparente, con barra de título superpuesta y título oculto.

La interfaz, en `src/styles.css` y `src/main.js`, usa la tipografía del sistema, cápsulas, una barra lateral y una hoja de confirmación. Hay un brillo animado en la marca de inicio. `prefers-reduced-motion: reduce` detiene esa animación y el anillo de progreso.

La persona usuaria vio la ventana el 2026-10-15 y dijo que quedó muy buena. No hubo medición de contraste ni prueba con `prefers-reduced-transparency`.

## Desviaciones aceptadas

| Pauta del marco | Qué hace Cristal | Motivo |
|---|---|---|
| Reservar el vidrio a navegación y controles. | El material nativo cubre toda la ventana. | `window-vibrancy` lo aplica a la vista de la ventana, no a un control suelto. |
| No apilar vidrio sobre vidrio. | Hay paneles y la hoja con fondo translúcido encima del material. | Hacen falta superficies legibles sobre el vidrio de la ventana. |
| Texto sobre superficie convencional. | El texto va sobre el vidrio, con rellenos `light-dark`. | La persona aceptó el resultado visual. No se midió WCAG. |
| Superficie opaca si se reduce la transparencia. | No está implementado. | No se pidió en esta entrega. |

Estas desviaciones no son un conflicto abierto: la entrega visual fue aceptada. Un cambio para acercarse a la pauta del marco necesita su propio plan; no se hace aquí.

## Qué no afirma este documento

CSS no reproduce el material óptico de Apple. El efecto nativo es el de `NSGlassEffectView`. Los gradientes del `body` son un apoyo visual, no el material.
