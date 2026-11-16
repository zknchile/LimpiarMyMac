# PLAN-0012 — Migrar la vista a GPUI

- **Fecha:** 2026-11-16
- **Estado:** completado
- **Solicitud:** es una prueba. Migrar el renderizado visual a GPUI. Perder elementos visuales no es un problema.
- **Tareas:** [TASKS-0012](TASKS-0012-migrar-vista-a-gpui.md)

Al escribirse, este plan no cambiaba la app y la ventana vigente era Tauri. El cierre está en el seguimiento: `cargo run` abre la ventana GPUI. No aplica `NSGlassEffectView` y no cumple [REQ-0003](../requirements/baseline.md).

## Objetivo, alcance y criterios de aceptación

Sustituir la ventana WebKit y el HTML por una ventana GPUI en el mismo proceso Rust. El análisis, la lista blanca y el movimiento a la Papelera no cambian de reglas.

GPUI no pinta el HTML existente. No hay una migración del motor dejando `src/main.js`. La vista se reescribe. El puente `invoke` desaparece porque la vista y el análisis quedan en el mismo binario.

Criterios observables de la prueba, cuando se ejecute:

- `cargo run` dentro de `src-tauri` abre una ventana en este Mac. El título es Cristal.
- Analizar llama a `scan::run` fuera del hilo de pintado. La ventana sigue respondiendo. El texto de progreso cambia al menos una vez, o aparece el informe final. Sigue diciendo que todavía no se borra nada.
- Marcar o desmarcar cambia el tamaño mostrado. Si no queda nada, muestra 0 B.
- Revisar y limpiar pide confirmación. Cancelar no llama a `clean::execute`. Confirmar solo envía identificadores del último análisis.
- Salir cierra el proceso desde el inicio y desde los resultados. Analizar no se puede pulsar otra vez mientras hay un análisis o una limpieza.
- Ver sigue usando `open -R` y rechaza rutas fuera de la carpeta del usuario. Abrir Papelera sigue abriendo la Papelera.
- `cargo test --lib` pasa, incluidas las pruebas de selección y de sitio de la grilla trasladadas a Rust.
- En el árbol no quedan `tauri`, `window-vibrancy`, `invoke` ni Vite.
- `safety.rs` no gana rutas nuevas. Un arreglo mecánico de edición de Rust, si el compilador lo exige, no cambia lo que `is_deletable` acepta.

Lo que esta prueba no tiene que conservar, porque así se pidió:

- Liquid Glass, vibrancy, cápsulas, orbe, anillo animado, hoja translúcida, `light-dark` y la tipografía del sistema.
- La esquina exacta de Salir. El botón tiene que existir, verse y cerrar. No es un fallo de esta prueba que no esté en el píxel de [REQ-0004](../requirements/baseline.md).
- Un `.app` firmado, un `.dmg` o una release.

[REQ-0003](../requirements/baseline.md) no se reescribe. Esta prueba no afirma que lo cumple. La línea base no se toca salvo que, después, la persona acepte la ventana nueva.

Fuera de alcance: un servicio HTTP, Docker, cambiar categorías de limpieza, seguir enlaces de otra forma, y el fork [gpui-liquid-glass](https://github.com/1234igor/gpui-liquid-glass). Si la ventana no abre, no se añade ese fork para «arreglar el vidrio».

## Fuentes, estado actual, supuestos y preguntas

Revisados aquí: [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md), [diseño aplicado](../design/liquid-glass.md), `src-tauri/src/lib.rs`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `src/main.js`, `src/styles.css`, `src/selection.js`, `src/place.js`, `package.json` y [casos](../testing/TEST-CASES.md).

Hoy Tauri 2 abre una ventana transparente de 1080×740 y aplica `NSGlassEffectView` con `window-vibrancy` 0.8. La vista son 453 líneas de `main.js` y 532 de `styles.css`. Los comandos son `scan`, `clean`, `reveal`, `open_trash` y `quit`. El progreso llega por `scan-progress` y `clean-progress`. Las pruebas de tamaño y de sitio de la grilla corren con `npm test`. El crate es edition 2021 y `rust-version` 1.77. El trabajo pesado ya está en Rust: `scan.rs` tiene 1016 líneas; `safety.rs`, 593.

GPUI, el framework de Zed, dibuja sus píxeles con Metal. No incrusta WebKit ni usa controles de AppKit. El crate oficial en crates.io está atrasado y el proyecto sigue en pre-1.0, atado a Zed. La publicación utilizable el 2026-11-16 es la republicación no oficial:

- `gpui-unofficial` 1.28.1, biblioteca `gpui`, Apache-2.0, edition 2024. Corresponde a Zed `v1.28.1`, commit `e8bc16acee5ae1937600d571d441c4b5d7b0d288`. Publicado el 2026-11-12. No lo mantiene Zed. El propio proyecto dice que necesita un mantenedor.
- `gpui-platform-gpui-unofficial` 1.28.1, que se importa como `gpui_platform`. Su feature `font-kit` no es la de por defecto. Sin ella, el texto se mide y no se dibujan glifos.

Edition 2024 exige Rust 1.85 o posterior. 1.77 no compila esa dependencia.

Supuestos: la prueba se hace en este repositorio, en el Mac que ya abre Cristal, con las herramientas de Xcode porque Metal las necesita. No hay pregunta abierta: la persona aceptó perder lo visual en esta prueba y no pidió conservar Liquid Glass.

No hay ADR nuevo en este archivo. Se creará al implementar, en estado `Proposed`. No se enlaza todavía porque el archivo no existe. [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md) no pasa a `Superseded` hasta que la ventana GPUI abra de verdad.

## Diseño, responsabilidades, contratos y alternativas

El crate sigue en `src-tauri/`. No se mueve a la raíz.

- `main.rs` arranca `gpui_platform::application()` y abre una ventana.
- Un módulo nuevo de vista pinta las pantallas: inicio, análisis, resultados, confirmación, limpieza y cierre. Texto plano, lista y botones. Sin material de vidrio.
- `scan`, `apps`, `size`, `safety`, `clean` y `model` se quedan. `clean` sigue aceptando solo identificadores del último informe. `reveal` y `open_trash` siguen siendo procesos de `open`.
- `lib.rs` deja de registrar comandos Tauri. Conserva el informe último y el indicador de tarea en curso, para que `cargo test --lib` siga compilando el núcleo.
- El análisis y la limpieza no corren en el hilo que pinta. Un callback de progreso vuelve a la vista por el mecanismo de actualización de GPUI. No se bloquea esa vista desde Rayon con un mutex propio.
- La selección y el sitio de la grilla pasan a funciones Rust puras, con los mismos casos que `selection.js` y `place.js`. El sitio se aplica si la lista expone un desplazamiento. Si no lo expone, se anota la limitación; no se borran las pruebas de la función.

Dependencia, con versión exacta, no un rango:

```toml
gpui-unofficial = "=1.28.1"
gpui_platform = { package = "gpui-platform-gpui-unofficial", version = "=1.28.1", features = ["font-kit"] }
```

`font-kit` va explícito porque en 1.28.1 no es feature por defecto del crate de plataforma. No se activa `screen-capture` ni `hot-reload`. No se llama al cliente HTTP que GPUI arrastra. Si quitar los features por defecto de `gpui-unofficial` impide abrir la ventana en macOS, se deja el default publicado y se anota; no se inventan features.

Se sube `rust-version` al menos a 1.85. La edition del crate de Cristal no se cambia salvo que el compilador lo exija. La versión que compiló se anota en el ADR. Se quitan `tauri`, `tauri-build`, `window-vibrancy`, `staticlib` y `cdylib` solo cuando la ventana nueva ya abre. Hasta entonces, un fallo revierte ese commit y `npm run tauri dev` tiene que volver a servir.

Alternativa rechazada: el crate `gpui` oficial. La publicación usable no es esa.

Alternativa rechazada: depender del repositorio de Zed por git. Arrastra el workspace del editor. La republicación 1.28.1 ya nombra el commit de origen.

Alternativa rechazada: `gpui-kit`, GPUI-CE o un fork con shader de vidrio. Más piezas, y el vidrio pintado no es `NSGlassEffectView`. Esta prueba no lo necesita.

Alternativa rechazada: dejar Tauri y meter GPUI dentro del WebView. Son dos ventanas. No es una migración.

Alternativa rechazada: marcar ya [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md) como sustituido. El código todavía no cambió.

## Pasos y entregables

1. Guardar este plan y su listado — [TASK-0012-01](TASKS-0012-migrar-vista-a-gpui.md#task-0012-01--dejar-el-plan). No toca `src/` ni `src-tauri/`.
2. Redactar el ADR en `Proposed`, sin sustituir el vigente — [TASK-0012-02](TASKS-0012-migrar-vista-a-gpui.md#task-0012-02--proponer-el-adr). Depende del paso 1.
3. Trasladar las pruebas de selección y de sitio a Rust, sin borrar el JavaScript — [TASK-0012-03](TASKS-0012-migrar-vista-a-gpui.md#task-0012-03--trasladar-las-pruebas-puras). Depende del paso 2.
4. Abrir una ventana mínima: inicio, Analizar deshabilitado si no hay barrido, y Salir — [TASK-0012-04](TASKS-0012-migrar-vista-a-gpui.md#task-0012-04--abrir-la-ventana). Depende del paso 3. Si no abre, se para aquí.
5. Conectar análisis, lista, confirmación, limpieza, Ver y Papelera — [TASK-0012-05](TASKS-0012-migrar-vista-a-gpui.md#task-0012-05--conectar-el-flujo). Depende del paso 4.
6. Quitar Tauri, Vite y el HTML — [TASK-0012-06](TASKS-0012-migrar-vista-a-gpui.md#task-0012-06--quitar-la-ventana-webkit). Depende del paso 5.
7. Cerrar documentación solo si la ventana abrió — [TASK-0012-07](TASKS-0012-migrar-vista-a-gpui.md#task-0012-07--cerrar-documentacion). Depende del paso 6.

No se empuja a GitHub un estado en el que ni Tauri ni GPUI abren. No se crea release.

## Riesgos, seguridad y bloqueos

- Riesgo: la republicación rompe la API o se queda sin mantenedor. Mitigación: versión exacta `=1.28.1` y el commit de Zed anotado. No se flota a `1.29` dentro de esta prueba.
- Riesgo: el análisis en el hilo de pintado congela la ventana. Mitigación: el paso 5 no se acepta si la ventana deja de responder durante el barrido.
- Riesgo: el primer `cargo build` tarda y pide Xcode. No es un bloqueo de diseño. Si Metal no está disponible en esta máquina, la prueba se detiene y se dice.
- Riesgo: edition 2024 obliga a tocar archivos del núcleo. Solo se aceptan cambios mecánicos. `cargo test --lib` de `safety` tiene que seguir pasando con el mismo resultado.
- No se amplía la lista blanca. No hay secretos. No se añade red. El cliente HTTP transitivo de GPUI no se usa.
- Bloqueo de esta entrega: ninguno. El bloqueo de la implementación, si aparece, es no poder abrir la ventana. Entonces no se continúa.

## Validación y evidencia esperada

Al guardar el plan: enlaces locales entre este archivo y el listado, y los enlaces a documentos que ya existen. `git diff --check` no reporta espacios conflictivos. No corresponde `cargo test`: este paso no cambia Rust.

Al implementar, en la tarea que corresponda: `cargo test --lib` con salida real, y una nota de que `cargo run` abrió la ventana. Esa nota no es un caso nuevo del catálogo. [TESTING.md](../testing/TESTING.md) no pide inventar un caso de clic. La ejecución de las pruebas trasladadas se registra en [TEST-RUNS.md](../testing/TEST-RUNS.md) o en la tarea, sin reutilizar un resultado viejo de `npm test`.

## Entrega, documentación, decisiones y recuperación

Esta entrega solo añade el plan, el listado y el índice. Changelog, `AGENTS.md` y el párrafo de planes del README dicen que el plan está abierto y que la app no cambió.

Cuando se implemente: el diagrama del README, el diseño aplicado, `AGENTS.md` y las pruebas se actualizan en el paso 7, no antes. El ADR pasa a `Accepted` solo si la ventana abre y se puede lanzar un análisis. Si no, queda `Proposed` o pasa a `Rejected`, y el commit de GPUI se revierte. [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md) solo queda `Superseded` en ese cierre, y el ADR nuevo conserva lo que sigue valiendo: Rust en el host, sin HTTP y sin Docker.

Recuperación de esta entrega: borrar estos archivos y revertir el índice. Recuperación de la migración: revertir sus commits. No hay migración de datos.

## Seguimiento y resultados

Completado el 2026-11-16. `cargo run` abrió una ventana titulada Cristal. Un análisis llegó a resultados. Cancelar volvió a esa lista y no llamó a la limpieza. Salir cerró el proceso desde el inicio, durante el barrido y desde los resultados. El clic fue interno: Accesibilidad no deja pulsar desde esta sesión, y el arnés no quedó en el árbol. No se inventó un caso de clic. No se confirmó un movimiento real a la Papelera.

Se quitó la vista WebKit. [ADR-0003](../decisions/ADR-0003-vista-gpui.md) pasó a `Accepted`. [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md) quedó `Superseded by ADR-0003`. [REQ-0003](../requirements/baseline.md) no se reescribió. La ventana no lo cumple. [RUN-2026-11-16-02](../testing/TEST-RUNS.md#run-2026-11-16-02) aprobó `cargo test --lib`: 14 passed, 0 failed. No se subió la versión ni se creó una release.

Después del corte, `git diff --check` no reportó espacios conflictivos. `cargo run` en `src-tauri` volvió a abrir la ventana: CoreGraphics la listó como `title=Cristal`, 1080×773, en `488,287`. `kAEQuitApplication` cerró ese proceso y no quedó ningún `target/debug/cristal`. Este plan no publicó la versión. La release es [PLAN-0013](PLAN-0013-publicar-0.2.0.md).
