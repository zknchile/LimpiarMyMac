# TASKS-0012 — Migrar la vista a GPUI

- **Plan:** [PLAN-0012](PLAN-0012-migrar-vista-a-gpui.md)
- **Fecha:** 2026-11-16
- **Coordinación e integración:** agente principal
- **Ejecución y propiedad:** un agente, en secuencia. No hay edición paralela de `lib.rs` y de la vista.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
| --- | --- | --- | --- | --- |
| TASK-0012-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0012-02 | Proponer el ADR | TASK-0012-01 | Agente principal | completada |
| TASK-0012-03 | Trasladar las pruebas puras | TASK-0012-02 | Agente principal | completada |
| TASK-0012-04 | Abrir la ventana | TASK-0012-03 | Agente principal | completada |
| TASK-0012-05 | Conectar el flujo | TASK-0012-04 | Agente principal | completada |
| TASK-0012-06 | Quitar la ventana WebKit | TASK-0012-05 | Agente principal | completada |
| TASK-0012-07 | Cerrar documentación | TASK-0012-06 | Agente principal | completada |

## TASK-0012-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado, y enlazarlos desde el índice. No tocar `src/` ni `src-tauri/`. No crear el ADR: el archivo todavía no existe y un enlace roto no cumple el criterio.
- **Entradas y dependencias:** ninguna. La ventana vigente está en [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md).
- **Entregable:** ambos archivos, con enlaces recíprocos. El índice dice que el plan está abierto.
- **Aceptación y validación:** el plan no autoriza borrar la lista blanca, ni afirmar que Liquid Glass se cumple, ni marcar ADR-0001 como sustituido.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** no paralelizable con código. Entrega a la misma sesión.
- **Resultado y evidencia:** archivos creados el 2026-11-16, antes de cualquier cambio en `src/` o `src-tauri/`.

## TASK-0012-02 — Proponer el ADR

- **Referencia al plan:** paso 2 y la sección de decisiones.
- **Objetivo y alcance:** crear el ADR siguiente en `Proposed`. Contexto: esta prueba pide GPUI y acepta perder lo visual. Alternativas: las del plan, con el motivo ya escrito. No pasar [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md) a `Superseded`. No cambiar la línea base de requisitos.
- **Entradas y dependencias:** TASK-0012-01.
- **Entregable:** el ADR enlazado desde [el registro](../decisions/README.md) como propuesto, no vigente en el código.
- **Aceptación y validación:** el ADR nombra `gpui-unofficial` `=1.28.1`, el commit `e8bc16acee5ae1937600d571d441c4b5d7b0d288` y que el vidrio nativo queda fuera de la prueba. No enlaza un fork de shader.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** un agente. Entrega a TASK-0012-03.
- **Resultado y evidencia:** [ADR-0003](../decisions/ADR-0003-vista-gpui.md) quedó `Proposed` el 2026-11-16, enlazado desde el registro. Nombra `gpui-unofficial` `=1.28.1`, el commit `e8bc16acee5ae1937600d571d441c4b5d7b0d288` y que el vidrio nativo queda fuera. [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md) sigue `Accepted`.

## TASK-0012-03 — Trasladar las pruebas puras

- **Referencia al plan:** paso 3. Casos [TEST-selection-001](../testing/TEST-CASES.md#test-selection-001) a [TEST-place-003](../testing/TEST-CASES.md#test-place-003).
- **Objetivo y alcance:** llevar `formatBytes`, `selectionOf` y `placeToKeep` a Rust, con los mismos resultados que el JavaScript. Actualizar la ficha: ruta nueva y comando `cargo test`. No borrar `src/selection.js`, `src/place.js` ni `npm test` en esta tarea. No inventar casos de ventana.
- **Entradas y dependencias:** TASK-0012-02.
- **Entregable:** pruebas Rust que cubren 0 B, marcar y desmarcar, el salto de texto de 80 MB, y las tres decisiones de sitio de la grilla.
- **Aceptación y validación:** `cargo test --lib` pasa. La salida se registra en [TEST-RUNS.md](../testing/TEST-RUNS.md) o en esta tarea. No se reutiliza la evidencia vieja de `npm test`. `npm test` sigue pasando, porque el JavaScript aún está.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** un agente. No se edita `safety.rs` para esta tarea. Entrega a TASK-0012-04.
- **Resultado y evidencia:** completada el 2026-11-16. `cargo test --lib` en `src-tauri`: 14 passed, 0 failed. Incluye `selection::tests` y `place::tests`. `npm test` en la raíz: 6 pass, 0 fail. Registro: [RUN-2026-11-16-01](../testing/TEST-RUNS.md#run-2026-11-16-01). Las fichas apuntan a Rust. El JavaScript no se borró.

## TASK-0012-04 — Abrir la ventana

- **Referencia al plan:** paso 4 y la dependencia fijada.
- **Objetivo y alcance:** añadir GPUI con la versión exacta del plan, `font-kit` en el crate de plataforma, y `rust-version` al menos 1.85. Una ventana con el título Cristal, un texto de inicio, Analizar y Salir. Salir cierra el proceso. No conectar el barrido todavía. No quitar Tauri en esta tarea si eso deja el árbol sin una forma de volver atrás en el mismo commit. Si la ventana no abre, revertir este commit.
- **Entradas y dependencias:** TASK-0012-03. Xcode, porque Metal lo pide.
- **Entregable:** `cargo run` abre la ventana, o un registro del error y el árbol otra vez compilable con Tauri.
- **Aceptación y validación:** la persona, o el agente en esta máquina, ve la ventana y Salir la cierra. Eso se anota en esta tarea, no como un ID nuevo del catálogo. Si falla, no se sigue al paso 5.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** un agente. No se empuja un estado en el que ninguna ventana abre. Entrega a TASK-0012-05 solo si la ventana abrió.
- **Resultado y evidencia:** completada el 2026-11-16. `cargo run` abrió una ventana. CoreGraphics la listó como `title=Cristal`, 1080×773, en `488,287`. Una captura muestra el texto de inicio, «Analizar esta Mac» y «Salir» abajo a la derecha. El build usó `DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer` porque el selector apuntaba a Command Line Tools. Accesibilidad no deja pulsar desde esta sesión. Un clic interno de Salir en el inicio escribió `check-press quit` y el proceso desapareció. Ese arnés no quedó en el árbol. No es un caso del catálogo.

## TASK-0012-05 — Conectar el flujo

- **Referencia al plan:** paso 5 y los criterios funcionales.
- **Objetivo y alcance:** inicio, progreso, resultados, confirmación, limpieza y cierre, en español, en texto plano. `scan::run` y `clean::execute` fuera del hilo de pintado. Confirmación obligatoria. Cancelar no limpia. Ver y Papelera se quedan como están en reglas. Una sola tarea a la vez.
- **Entradas y dependencias:** TASK-0012-04 con ventana abierta.
- **Entregable:** se puede analizar, cambiar la selección, cancelar una limpieza y, si se confirma, mover solo identificadores del último informe.
- **Aceptación y validación:** los criterios funcionales del plan, menos la ausencia de Tauri, que es la tarea siguiente. Durante el barrido la ventana sigue pudiendo cerrarse. `cargo test --lib` sigue pasando.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** un agente. No se reescribe `safety.rs` para hacer pasar la vista. Entrega a TASK-0012-06.
- **Resultado y evidencia:** completada el 2026-11-16. El mismo clic interno pulsó Analizar. La ventana mostró «Todavía no se borra nada.» y Salir, durante el barrido, cerró el proceso. Otro arranque llegó a resultados: la hoja decía «Mover a la Papelera», 96 elementos y 6.68 GB. Cancelar escribió `cancel-hit`, la captura siguiente seguía en resultados (`6.68 GB · 96 seleccionados`, «Revisar y limpiar») y no apareció «Moviendo a la Papelera». Salir desde ahí escribió `check-press quit` y el proceso desapareció. No se confirmó un movimiento real. No se inventó un caso de clic. `cargo test --lib` quedó en [RUN-2026-11-16-02](../testing/TEST-RUNS.md#run-2026-11-16-02).

## TASK-0012-06 — Quitar la ventana WebKit

- **Referencia al plan:** paso 6.
- **Objetivo y alcance:** quitar Tauri, `window-vibrancy`, `tauri.conf.json`, el `build.rs` de Tauri, Vite, `index.html` y el JavaScript de la vista. Quitar `npm test` y Node de los requisitos. Dejar el crate como `rlib` y binario. Actualizar las fichas para que la reproducción ya no nombre `npm test`.
- **Entradas y dependencias:** TASK-0012-05.
- **Entregable:** el árbol no contiene `tauri` ni `invoke`. `cargo test --lib` es la suite.
- **Aceptación y validación:** búsqueda sin coincidencias de `tauri`, `window-vibrancy` e `invoke` en el código de la app. `cargo test --lib` pasa. `cargo run` sigue abriendo la ventana.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** un agente. No se hace este corte si el paso 5 no cumplió. Entrega a TASK-0012-07.
- **Resultado y evidencia:** completada el 2026-11-16. Se quitaron Tauri, Vite, `index.html`, el JavaScript de la vista, `tauri.conf.json` y `capabilities`. El crate ya era `rlib` más binario; `build.rs` de Tauri ya no estaba. Node no queda en los requisitos. Las fichas ya no nombran `npm test` como reproducción. `cargo test --lib`: 14 passed, 0 failed, en [RUN-2026-11-16-02](../testing/TEST-RUNS.md#run-2026-11-16-02).

## TASK-0012-07 — Cerrar documentación

- **Referencia al plan:** paso 7.
- **Objetivo y alcance:** actualizar README, diagrama, `AGENTS.md`, diseño aplicado, changelog y pruebas para describir la ventana GPUI. El diseño debe decir que esta prueba no usa `NSGlassEffectView` y no cumple [REQ-0003](../requirements/baseline.md). El ADR pasa a `Accepted` y ADR-0001 queda `Superseded` solo en ese caso. No subir la versión ni crear release.
- **Entradas y dependencias:** TASK-0012-06.
- **Entregable:** un lector del README no cree que la vista sigue siendo HTML ni que el vidrio sigue aplicado.
- **Aceptación y validación:** enlaces locales resuelven. `git diff --check` no reporta espacios conflictivos. La evidencia de `cargo test --lib` y de la ventana abierta está en las tareas o en [TEST-RUNS.md](../testing/TEST-RUNS.md).
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** cierre. No se empuja antes de esta tarea si el remoto todavía documentaría una app que el código ya no es, o al revés.
- **Resultado y evidencia:** completada el 2026-11-16. README, diagrama, `AGENTS.md`, diseño, changelog, pruebas y el registro de decisiones describen la ventana GPUI. El diseño dice que no usa `NSGlassEffectView` y no cumple [REQ-0003](../requirements/baseline.md). [ADR-0003](../decisions/ADR-0003-vista-gpui.md) está `Accepted`. [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md) está `Superseded by ADR-0003`. La línea base de requisitos no se reescribió. No se subió la versión. `git diff --check` no reportó espacios conflictivos. Tras el corte, `cargo run` volvió a abrir la ventana (`title=Cristal`, 1080×773, en `488,287`) y `kAEQuitApplication` la cerró. No quedó ningún `target/debug/cristal`.
