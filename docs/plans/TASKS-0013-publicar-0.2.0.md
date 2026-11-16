# TASKS-0013 — Publicar Cristal 0.2.0

- **Plan:** [PLAN-0013](PLAN-0013-publicar-0.2.0.md)
- **Fecha:** 2026-11-16
- **Coordinación e integración:** agente principal
- **Ejecución y propiedad:** un agente, en secuencia. No hay edición paralela del empaquetado y de la versión.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
| --- | --- | --- | --- | --- |
| TASK-0013-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0013-02 | Alinear versión y documentación | TASK-0013-01 | Agente principal | completada |
| TASK-0013-03 | Empaquetar y publicar | TASK-0013-02 | Agente principal | completada |

## TASK-0013-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de subir la versión. No tocar `src-tauri/src/` ni `safety.rs`.
- **Entradas y dependencias:** ninguna. [PLAN-0012](PLAN-0012-migrar-vista-a-gpui.md) ya cerró la migración y no publicó.
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan fija `0.2.0`, no reescribe REQ-0003 como cumplido y no autoriza una firma de Apple.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** no paralelizable con el cambio de versión. Entrega a TASK-0013-02.
- **Resultado y evidencia:** archivos creados el 2026-11-16, antes de editar la versión.

## TASK-0013-02 — Alinear versión y documentación

- **Referencia al plan:** paso 2.
- **Objetivo y alcance:** poner `0.2.0` en el crate. Actualizar changelog, README, `AGENTS.md` y el estado de REQ-0003. Enlazar este plan desde el seguimiento de [PLAN-0012](PLAN-0012-migrar-vista-a-gpui.md), sin hacer que aquel plan parezca el de la release.
- **Entradas y dependencias:** TASK-0013-01.
- **Entregable:** un lector no cree que la 0.1.1 sigue siendo la ventana publicada, ni que esta ventana cumple REQ-0003.
- **Aceptación y validación:** `Cargo.toml` dice `0.2.0`. El README enlaza la release y dice que no está firmada ni notarizada. `git diff --check` no reporta espacios conflictivos.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** un agente. Entrega a TASK-0013-03.
- **Resultado y evidencia:** el 2026-11-16, `src-tauri/Cargo.toml` dice `0.2.0`. El README enlaza `v0.2.0` y dice que el disco no está firmado ni notarizado. REQ-0003 sigue pedido y ya no afirma que la ventana publicada use el material nativo. No hay RD.

## TASK-0013-03 — Empaquetar y publicar

- **Referencia al plan:** paso 3.
- **Objetivo y alcance:** pasar `cargo test --lib`, compilar en release, armar el `.dmg` arm64, abrir el `.app`, cerrarlo, empujar `main` y crear la release `v0.2.0` con el disco y las sumas. No dejar el proceso en ejecución. No firmar con un certificado de Apple.
- **Entradas y dependencias:** TASK-0013-02.
- **Entregable:** el tag y los dos archivos en GitHub. `HEAD` coincide con `origin/main`.
- **Aceptación y validación:** CoreGraphics lista la ventana del `.app` como `title=Cristal`. Tras `kAEQuitApplication` no queda un proceso `cristal` de esa app. La release apunta al commit empujado. Las notas incluyen el SHA-256 del `.dmg`.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** cierre. Entrega a la persona usuaria.
- **Resultado y evidencia:** el 2026-11-16, `cargo test --lib` pasó 14 pruebas y 0 fallos. No cambió aserciones, así que no hay ficha nueva. `cargo build --release` produjo un binario arm64. El `.app` del `.dmg` abrió una ventana que CoreGraphics listó como `title=Cristal`, 1080×773, en `488,287`. `kAEQuitApplication` cerró ese proceso y no quedó `cristal`. `b7b7381` quedó en `origin/main`. La release https://github.com/zknchile/LimpiarMyMac/releases/tag/v0.2.0 apunta a ese commit y adjunta el `.dmg` y las sumas. SHA-256: `2670bae5496c0aef2507f60523b8bf20019e8c277010c24531528e68834bc03e`. Este cierre es el commit siguiente.
