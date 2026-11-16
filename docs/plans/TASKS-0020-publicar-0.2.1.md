# TASKS-0020 — Publicar Cristal 0.2.1

- **Plan:** [PLAN-0020](PLAN-0020-publicar-0.2.1.md)
- **Fecha:** 2026-11-16
- **Coordinación e integración:** agente principal
- **Ejecución y propiedad:** un agente, en secuencia. No hay edición paralela del empaquetado y de la versión.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
| --- | --- | --- | --- | --- |
| TASK-0020-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0020-02 | Alinear versión y documentación | TASK-0020-01 | Agente principal | completada |
| TASK-0020-03 | Empaquetar y publicar | TASK-0020-02 | Agente principal | pendiente |

## TASK-0020-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de subir la versión. No tocar `src-tauri/src/` ni `safety.rs`.
- **Entradas y dependencias:** ninguna. La release vigente es la 0.2.0, según [PLAN-0013](PLAN-0013-publicar-0.2.0.md).
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan fija `0.2.1`, no reescribe REQ-0003 como cumplido y no autoriza una firma de Apple.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** no paralelizable con el cambio de versión. Entrega a TASK-0020-02.
- **Resultado y evidencia:** archivos creados el 2026-11-16, antes de editar la versión.

## TASK-0020-02 — Alinear versión y documentación

- **Referencia al plan:** paso 2.
- **Objetivo y alcance:** poner `0.2.1` en el crate. Actualizar changelog, README, `AGENTS.md` y el estado de REQ-0003. No hacer que [PLAN-0013](PLAN-0013-publicar-0.2.0.md) parezca el de esta release.
- **Entradas y dependencias:** TASK-0020-01.
- **Entregable:** un lector no cree que la 0.2.0 sigue siendo la última ventana publicada, ni que esta ventana cumple REQ-0003.
- **Aceptación y validación:** `Cargo.toml` dice `0.2.1`. El README enlaza `v0.2.1` y dice que no está firmada ni notarizada. `git diff --check` no reporta espacios conflictivos.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** un agente. Entrega a TASK-0020-03.
- **Resultado y evidencia:** el 2026-11-16, `src-tauri/Cargo.toml` dice `0.2.1`. El README enlaza `v0.2.1` y dice que el disco no está firmado ni notarizado. REQ-0003 sigue pedido. No hay RD.

## TASK-0020-03 — Empaquetar y publicar

- **Referencia al plan:** paso 3.
- **Objetivo y alcance:** pasar `cargo test --lib`, compilar en release, armar el `.dmg` arm64, abrir el `.app`, cerrarlo, empujar `main` y crear la release `v0.2.1` con el disco y las sumas. No dejar el proceso en ejecución. No firmar con un certificado de Apple.
- **Entradas y dependencias:** TASK-0020-02.
- **Entregable:** el tag y los dos archivos en GitHub. `HEAD` coincide con `origin/main` en el commit de la versión; el registro del SHA va en el commit siguiente.
- **Aceptación y validación:** CoreGraphics lista la ventana del `.app` como `title=Cristal`. Tras `kAEQuitApplication` no queda un proceso `cristal` de esa app. La release apunta al commit empujado. Las notas incluyen el SHA-256 del `.dmg`.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** cierre. Entrega a la persona usuaria.
- **Resultado y evidencia:** pendiente.
