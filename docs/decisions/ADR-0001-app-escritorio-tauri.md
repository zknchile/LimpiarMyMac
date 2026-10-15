# ADR-0001 — App de escritorio con Tauri

- **Status:** Accepted
- **Date:** 2026-10-15

## Context

La persona usuaria pidió probar Rust en su Mac con una app visual, estilo Liquid Glass, para analizar y luego limpiar ([REQ-0001](../requirements/baseline.md), [REQ-0003](../requirements/baseline.md)). El equipo era un Mac con Apple M1 Pro y macOS 27.0.1. Rust no estaba instalado al empezar.

El marco [Arquitectura_agent](https://github.com/zknchile/Arquitectura_agent) define frontend SolidJS con Atomic Design, backend hexagonal y comunicación solo por HTTP/JSON. También pide ejecutar instalación, desarrollo y pruebas dentro de Docker, y Python solo si hay IA. Cristal no tiene IA.

Durante la construcción se consideraron interfaces Rust que no abren un material nativo de macOS con la misma facilidad. La ventana implementada usa Tauri 2 y `window-vibrancy` `0.8.1`, que expone `apply_liquid_glass` en macOS 26+. La persona usuaria probó la ventana y dijo que quedó muy buena. Esa aceptación es la evidencia de que la forma entregada sirve; no es una instrucción previa de usar Tauri.

## Decision

Cristal es una app de escritorio local:

- Rust analiza, calcula tamaños y mueve archivos. No hay servicio HTTP ni puerto de red.
- Tauri 2 muestra la ventana y expone comandos (`scan`, `clean`, `reveal`, `open_trash`, `quit`).
- El frontend es HTML, CSS y JavaScript servido por Vite. No se adopta SolidJS ni Atomic Design en este producto.
- Liquid Glass se aplica con `NSGlassEffectView` mediante `window-vibrancy`, estilo Regular, radio 26 e interactivo. Si el efecto no está disponible, se usa vibrancy.
- El desarrollo y `cargo test` corren en el host. Docker no se usa porque la ventana necesita el servidor gráfico y las API de AppKit del Mac.

Python no aplica: no hay funcionalidad de IA.

## Alternatives Considered

- SolidJS, arquitectura hexagonal y HTTP/JSON, como indica el marco. Se descarta para este producto: añadiría un servidor y un cliente separados para una herramienta que solo actúa sobre el disco local del usuario. No había un requisito de red.
- Interfaz inmediata en Rust (`egui` o `iced`). Se consideró al elegir stack. No se adoptó porque el material Liquid Glass de macOS 26+ está disponible para una ventana nativa a través de `window-vibrancy`, y una UI web permite la cápsula, la hoja de confirmación y la tipografía del sistema con menos trabajo.
- SwiftUI. Cumple Liquid Glass de forma nativa, pero no cumple el pedido de desarrollar en Rust.
- Contenedor Docker para compilar y probar. El marco lo exige para aplicaciones de servicio. Aquí impediría abrir la ventana sobre el escritorio del usuario. Se deja fuera y se documenta, en lugar de afirmar que el marco se cumplió.

## Consequences

- Un agente futuro no debe partir el análisis en un servicio HTTP solo por simetría con el marco.
- Los comandos Tauri son el contrato interno. No hay OpenAPI.
- Cualquier cambio de stack visual o de lenguaje requiere un ADR nuevo; este queda `Superseded` si eso ocurre.
- La instalación de Rust y Node queda en la máquina de quien desarrolla. No se versionan `node_modules` ni `src-tauri/target`.

## Related Requirements

[REQ-0001](../requirements/baseline.md), [REQ-0003](../requirements/baseline.md). No modifica el marco de Arquitectura_agent.

## Related ADRs

Ninguno anterior en este repositorio. [ADR-0002](ADR-0002-lista-blanca-y-papelera.md) fija cómo se borran archivos dentro de esta app.
