# TASKS-0021 — Icono de Cristal

- **Plan:** [PLAN-0021](PLAN-0021-icono-de-cristal.md)
- **Fecha:** 2026-11-16
- **Coordinación e integración:** agente principal
- **Ejecución y propiedad:** un agente, en secuencia. No hay edición paralela del icono y de `main.rs`.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
| --- | --- | --- | --- | --- |
| TASK-0021-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0021-02 | Poner el icono | TASK-0021-01 | Agente principal | completada |

## TASK-0021-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de tocar el icono o `main.rs`. No tocar `src-tauri/src/` ni `safety.rs`.
- **Entradas y dependencias:** ninguna. El icono vigente es la gota de Tauri y `cargo run` no lo muestra.
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan fija los hex, niega la gota de Tauri y no afirma Liquid Glass ni una release.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** no paralelizable con el cambio del icono. Entrega a TASK-0021-02.
- **Resultado y evidencia:** archivos creados el 2026-11-16, antes de dibujar el icono.

## TASK-0021-02 — Poner el icono

- **Referencia al plan:** paso 2 y la figura.
- **Objetivo y alcance:** dibujar el cristal, escribir `icon.png` e `icon.icns`, quitar los PNG y el `.ico` de Tauri, y asignar el icono al Dock al abrir. Anotarlo en el diseño aplicado, el changelog, `AGENTS.md` y el párrafo de planes. No cambiar `safety.rs`, la vista, el sitio de Salir, la versión ni las pruebas.
- **Entradas y dependencias:** TASK-0021-01.
- **Entregable:** el Dock y el `.icns` muestran el cristal plano. Un lector no cree que esto cumple [REQ-0003](../requirements/baseline.md) ni que sigue el icono de Tauri.
- **Aceptación y validación:** un pixel de la esquina es `#0F172A` y uno de la faceta alta es `#E2E8F0`. `main.rs` llama a `setApplicationIconImage`. `git diff --check` no reporta espacios conflictivos. La ventana abre titulada Cristal. El icono del proceso no es la gota azul. Salir sigue a la derecha de Analizar. No se pulsa «Mover a la Papelera».
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** cierre. No se empuja en esta tarea.
- **Resultado y evidencia:** el 2026-11-16, la esquina del PNG midió 15,23,42 y la faceta alta 226,232,240. `main.rs` llama a `setApplicationIconImage` antes de abrir la ventana. `cargo test --lib` pasó 14 pruebas y 0 fallos. No hay ficha nueva. `git diff --check` no reportó espacios conflictivos. La ventana se listó como `title=Cristal`. El Dock mostró el cristal sobre tinta, no la gota de Tauri. No se tocó `view.rs`, así que Salir sigue a la derecha de Analizar. No se pulsó «Mover a la Papelera» y no se movió ningún archivo. No se empujó.
