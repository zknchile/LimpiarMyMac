# Requisitos confirmados

Línea base de lo que la persona usuaria pidió en la sesión del 2026-10-15. No se añaden funcionalidades que no estén en esos pedidos ni en el código comprobado.

| ID | Requisito | Estado |
|---|---|---|
| REQ-0001 | App visual en Rust para macOS que limpie datos de navegadores y restos de aplicaciones ya borradas. | Confirmado e implementado. |
| REQ-0002 | Primero analizar; con ese resultado decidir qué se puede borrar; después ofrecer ejecutar. | Confirmado e implementado. |
| REQ-0003 | Interfaz Liquid Glass y fácil de usar. | Confirmado. La ventana usa el material nativo y una interfaz en español. La adaptación no reproduce todas las pautas del marco; ver [diseño](../design/liquid-glass.md). |
| REQ-0004 | Botón Salir siempre visible, en la esquina inferior derecha y a la derecha de Analizar. | Confirmado. El sitio cambió el 2026-10-15; ver [RD-0002](../decisions/RD-0002-salir-abajo-a-la-derecha.md). |
| REQ-0005 | README que resuma el proyecto. | Confirmado e implementado. |
| REQ-0006 | Subir el proyecto a GitHub. | Confirmado. Repositorio público https://github.com/zknchile/LimpiarMyMac. |
| REQ-0007 | Documentar tareas, cambios y decisiones con las definiciones de [Arquitectura_agent](https://github.com/zknchile/Arquitectura_agent). | Confirmado. Este documento y los enlazados son el resultado. El catálogo de pruebas aplica la guía del marco del 2026-10-15; no cambia el producto. |

## Fuera de alcance confirmado

No se pidió historial de navegación, contraseñas, marcadores, Documentos, Fotos ni vaciado automático. No se pidió una API HTTP, SolidJS, Atomic Design ni despliegue en contenedores. Esas piezas pertenecen al marco de referencia y no a este producto, salvo donde un ADR local diga lo contrario.
