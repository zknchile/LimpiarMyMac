# PLAN-0021 — Icono de Cristal

- **Fecha:** 2026-11-16
- **Estado:** completado
- **Solicitud:** agregar un icono a la aplicación.
- **Tareas:** [TASKS-0021](TASKS-0021-icono-de-cristal.md)

Este documento no cambia la app al escribirse. La ventana vigente es GPUI, con la paleta de día lluvioso, según [PLAN-0017](PLAN-0017-paleta-dia-lluvioso.md). El `.app` copia `src-tauri/icons/icon.icns`. Ese archivo es la gota azul de Tauri, no una marca de Cristal. `cargo run` no lo muestra: un binario suelto no lee `CFBundleIconFile`.

## Objetivo, alcance y criterios de aceptación

El Dock y el `.app` muestran un cristal plano de Cristal. No es la gota de Tauri y no es vidrio.

Criterios observables:

- `src-tauri/icons/icon.png` es un cuadrado de 1024. El fondo es tinta `#0F172A`. El cristal tiene tres facetas: la alta `#E2E8F0`, la izquierda `#D2DAE5` y la derecha `#94A3B8`. No hay brillo, sombra ni un sexto color.
- `src-tauri/icons/icon.icns` sale de esa figura. `scripts/package_macos.sh` sigue copiando esa ruta. No cambia el identificador ni la versión.
- Al abrir la ventana con `cargo run`, el Dock muestra esa figura. `main.rs` llama a `setApplicationIconImage` en el hilo principal, después de que GPUI tiene `NSApplication` y antes de abrir la ventana. Si la imagen no decodifica, la ventana abre igual.
- Desaparecen `32x32.png`, `128x128.png`, `128x128@2x.png` e `icon.ico`. Nada del binario GPUI los usa, y son la marca de Tauri. El `icon.iconset` suelto, que git ignora, también se quita.
- La vista no gana un logo. Salir sigue abajo a la derecha de Analizar. No se toca `safety.rs`.
- No es `NSGlassEffectView` y no cumple [REQ-0003](../requirements/baseline.md). No hay RD ni ADR. No se sube la versión ni se publica. El disco 0.2.1 sigue con el icono anterior.

Fuera de alcance: un icono dentro de la ventana, una release, firmar con un certificado de Apple y un binario Intel.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `src-tauri/icons/icon.png`, `scripts/package_macos.sh`, `src-tauri/src/main.rs`, [diseño aplicado](../design/liquid-glass.md) y `NSApplication.setApplicationIconImage` de `objc2-app-kit` 0.3.2, que ya arrastra GPUI. La gota azul es el icono por defecto de Tauri. El empaquetado ya usa `icon.icns`. El código de la ventana no asigna icono.

Supuesto: «un icono a la aplicación» es el del Dock y del Finder, no un dibujo dentro de la vista. Los hex son los de la paleta vigente, incluida la franja `#D2DAE5` de [PLAN-0019](PLAN-0019-franja-del-menu.md). No hay pregunta abierta.

## Diseño, responsabilidades, contratos y alternativas

- La figura la dibuja `scripts/make_icon.swift`. El shell `scripts/make_icon.sh` escribe `icon.png` y `icon.icns`. Los hex viven en ese script, no en una imagen adivinada.
- El binario incrusta `icon.png` y se lo pasa a `NSImage`. El `.app` usa el `.icns`. Son la misma figura.
- El cuadrado es de borde a borde. macOS recorta la esquina al mostrarlo. No se dibuja la cápsula ni un reflejo.
- Alternativa rechazada: dejar la gota de Tauri. Es la marca de otro producto.
- Alternativa rechazada: un icono con brillo de vidrio. Parecería Liquid Glass y no cumple REQ-0003.
- Alternativa rechazada: solo el `.icns`. `cargo run` seguiría sin icono.
- Alternativa rechazada: un logo en la franja del menú. No se pidió cambiar la vista.
- Alternativa rechazada: publicar `0.2.2`. No se pidió una release.

## Pasos y entregables

1. Guardar este plan y su listado — [TASK-0021-01](TASKS-0021-icono-de-cristal.md#task-0021-01--dejar-el-plan).
2. Dibujar el icono y mostrarlo en el Dock y en el `.icns` — [TASK-0021-02](TASKS-0021-icono-de-cristal.md#task-0021-02--poner-el-icono). Depende del paso 1.

## Riesgos, seguridad y bloqueos

- Riesgo: leer el cristal como vidrio de Apple. Mitigación: el diseño nombra los hex planos y niega `NSGlassEffectView`.
- Riesgo: el Dock de `cargo run` sigue con el icono genérico si `NSApplication` aún no existe. Mitigación: la llamada va dentro de `application().run`.
- Riesgo: abrir la ventana con el puntero sobre Analizar. Mitigación: mover el puntero y no pulsar «Mover a la Papelera».
- No se toca `safety.rs`. No hay secretos. No hay bloqueo.

## Validación y evidencia esperada

No hay aserción nueva. No se ficha un caso. `cargo test --lib` sigue cubriendo rutas, tamaño y sitio de la grilla. Un pixel de la esquina del PNG es `#0F172A` y uno de la faceta alta es `#E2E8F0`. La ventana abre titulada Cristal y el icono del proceso no es la gota de Tauri. Eso se anota en la tarea, sin ID. No se pulsa «Mover a la Papelera». `git diff --check` no reporta espacios conflictivos.

## Entrega, documentación, decisiones y recuperación

Diseño aplicado, changelog, `AGENTS.md` y el párrafo de planes del README. Sin ADR, RD ni release. Recuperación: revertir el cambio y volver a poner los archivos de Tauri. No se sube en este plan.

## Seguimiento y resultados

Completado el 2026-11-16. Un pixel de la esquina del PNG salió en 15,23,42 (`#0F172A`). Uno de la faceta alta, en 226,232,240 (`#E2E8F0`). La izquierda, 210,218,229 (`#D2DAE5`). La derecha, 148,163,184 (`#94A3B8`). `cargo test --lib` pasó 14 pruebas y 0 fallos. No cambió aserciones, así que no hay ficha nueva. `git diff --check` no reportó espacios conflictivos. La ventana se listó como `title=Cristal`. El Dock mostró el cristal sobre tinta, no la gota azul de Tauri. Salir no se movió: no se tocó `view.rs`. No se pulsó «Mover a la Papelera» y no se movió ningún archivo. No se empujó. El disco publicado 0.2.1 sigue con el icono anterior.
