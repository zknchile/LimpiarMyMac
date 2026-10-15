# TASKS-0002 — Registro del desarrollo de Cristal

- **Plan:** [PLAN-0002](PLAN-0002-registro-desarrollo-cristal.md)
- **Fecha:** 2026-10-15
- **Coordinación e integración:** agente que construyó la app, en una sola sesión
- **Estrategia de ejecución y propiedad de archivos compartidos:** un agente, sin paralelismo. Registro reconstruido: estas fichas no se asignaron antes de codificar.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
|---|---|---|---|---|
| TASK-0002-01 | Preparar Rust y el stack | Ninguna | Agente de la sesión | completada |
| TASK-0002-02 | Analizar con lista blanca | TASK-0002-01 | Agente de la sesión | completada |
| TASK-0002-03 | Limpiar hacia la Papelera | TASK-0002-02 | Agente de la sesión | completada |
| TASK-0002-04 | Interfaz Liquid Glass | TASK-0002-01 | Agente de la sesión | completada |
| TASK-0002-05 | Salir, README y GitHub | TASK-0002-04, TASK-0002-06 parcial | Agente de la sesión | completada |
| TASK-0002-06 | Verificación del desarrollo | TASK-0002-02, TASK-0002-03, TASK-0002-04 | Agente de la sesión | completada |

TASK-0002-05 depende de la aceptación visual, que es parte de TASK-0002-06. No es un ciclo: la persona vio la ventana, pidió los tres cambios y esos cambios entraron en el mismo commit que el resto. En el registro se deja así, sin inventar dos commits.

## TASK-0002-01 — Preparar Rust y el stack

- **Referencia al plan:** paso 1 y [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md).
- **Objetivo y alcance:** instalar el toolchain Rust y crear el proyecto Tauri en este directorio. No elegir SolidJS.
- **Entradas y dependencias:** ninguna. macOS 27.0.1, Node existente, Rust ausente al inicio.
- **Entregable:** `cargo` 1.99 usable y proyecto Tauri 2 con `window-vibrancy` 0.8.1.
- **Aceptación y validación:** `tauri dev` compiló y ejecutó `target/debug/cristal`.
- **Paralelismo, límites de edición y destinatario de la entrega:** primera tarea. El resto del crate depende de ella.
- **Resultado y evidencia:** Rust estable `1.99.0` instalado con rustup. La ventana llegó a abrirse; la persona usuaria lo confirmó.

## TASK-0002-02 — Analizar con lista blanca

- **Referencia al plan:** paso 2, [REQ-0001](../requirements/baseline.md) y [REQ-0002](../requirements/baseline.md).
- **Objetivo y alcance:** inventario de apps, cachés, navegadores, restos, registros, desarrollo, instaladores y Papelera. Archivos: `scan.rs`, `apps.rs`, `safety.rs`, `size.rs`, `model.rs`.
- **Entradas y dependencias:** TASK-0002-01.
- **Entregable:** comando `scan` que emite progreso y devuelve categorías.
- **Aceptación y validación:** pruebas de `safety.rs` y `apps.rs` en verde. Un análisis completo del disco de la persona no quedó registrado.
- **Paralelismo, límites de edición y destinatario de la entrega:** podía avanzar junto a la interfaz solo después del modelo. En la sesión se escribió antes de cerrar la UI.
- **Resultado y evidencia:** módulos presentes. Cuatro pruebas pasaron el 2026-10-15. Limitación: no hay log de un análisis completo guardado en el repo.

## TASK-0002-03 — Limpiar hacia la Papelera

- **Referencia al plan:** paso 3 y [ADR-0002](../decisions/ADR-0002-lista-blanca-y-papelera.md).
- **Objetivo y alcance:** `clean.rs` mueve solo rutas del informe que vuelven a pasar `safety.rs`. Vaciar la Papelera queda aparte, vía Finder.
- **Entradas y dependencias:** TASK-0002-02, porque necesita el informe.
- **Entregable:** comando `clean`.
- **Aceptación y validación:** revisión del código y compilación. No hay una prueba que mueva un archivo temporal y lo restaure; eso sigue pendiente.
- **Paralelismo, límites de edición y destinatario de la entrega:** después del análisis. No comparte archivos con la UI salvo `lib.rs`.
- **Resultado y evidencia:** `clean.rs` y el comando registrado en `lib.rs`. Sin prueba de restauración.

## TASK-0002-04 — Interfaz Liquid Glass

- **Referencia al plan:** paso 4, [REQ-0003](../requirements/baseline.md) y [diseño](../design/liquid-glass.md).
- **Objetivo y alcance:** flujo analizar, revisar y confirmar, en español. Archivos: `src/main.js`, `src/styles.css`, `index.html`, efecto en `lib.rs`.
- **Entradas y dependencias:** TASK-0002-01. El contrato de comandos puede estabilizarse con TASK-0002-02.
- **Entregable:** ventana usable, con selección segura precargada y hoja de confirmación.
- **Aceptación y validación:** aceptación verbal de la persona usuaria. Las desviaciones de la pauta Liquid Glass están escritas, no ocultas.
- **Paralelismo, límites de edición y destinatario de la entrega:** la UI no edita `safety.rs`.
- **Resultado y evidencia:** la persona dijo que la app se desplegó y quedó muy buena, y pidió solo Salir, README y GitHub.

## TASK-0002-05 — Salir, README y GitHub

- **Referencia al plan:** paso 5 y [RD-0001](../decisions/RD-0001-salir-readme-github.md).
- **Objetivo y alcance:** botón Salir, README y repositorio público. No cambiar qué archivos se pueden mover.
- **Entradas y dependencias:** aceptación visual de TASK-0002-04.
- **Entregable:** commit `c9e6f67` en https://github.com/zknchile/LimpiarMyMac.
- **Aceptación y validación:** `cargo check` tras `quit`; `gh repo view` devolvió la URL.
- **Paralelismo, límites de edición y destinatario de la entrega:** toca `lib.rs`, `main.js`, `styles.css` y `README.md`. Un solo agente.
- **Resultado y evidencia:** commit `c9e6f67` en `main`, remoto `origin`. Repositorio público.

## TASK-0002-06 — Verificación del desarrollo

- **Referencia al plan:** validación y criterios.
- **Objetivo y alcance:** comprobar lo que se puede comprobar sin afirmar una limpieza real.
- **Entradas y dependencias:** TASK-0002-02, TASK-0002-03 y TASK-0002-04.
- **Entregable:** resultado de pruebas y límite explícito de lo no probado.
- **Aceptación y validación:** `cargo test` en verde y ventana abierta. La restauración desde la Papelera no es criterio cumplido.
- **Paralelismo, límites de edición y destinatario de la entrega:** cierre. No reabre el alcance.
- **Resultado y evidencia:** 4 pruebas pasaron. La persona usuaria vio la ventana. No hay evidencia de una limpieza restaurada. El registro se cierra con esa limitación.
