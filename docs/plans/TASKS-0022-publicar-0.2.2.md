# TASKS-0022 — Publicar Cristal 0.2.2

- **Plan:** [PLAN-0022](PLAN-0022-publicar-0.2.2.md)
- **Fecha:** 2026-11-16
- **Coordinación e integración:** agente principal
- **Ejecución y propiedad:** un agente, en secuencia. No hay edición paralela del empaquetado y de la versión.

## Listado

| ID | Tarea | Dependencias | Responsable | Estado |
| --- | --- | --- | --- | --- |
| TASK-0022-01 | Dejar el plan | Ninguna | Agente principal | completada |
| TASK-0022-02 | Alinear versión y documentación | TASK-0022-01 | Agente principal | completada |
| TASK-0022-03 | Empaquetar y publicar | TASK-0022-02 | Agente principal | completada |

## TASK-0022-01 — Dejar el plan

- **Referencia al plan:** paso 1.
- **Objetivo y alcance:** guardar el plan y este listado antes de subir la versión. No tocar `src-tauri/src/` ni `safety.rs`.
- **Entradas y dependencias:** ninguna. La release vigente es la 0.2.1, según [PLAN-0020](PLAN-0020-publicar-0.2.1.md). El icono ya está en el árbol, según [PLAN-0021](PLAN-0021-icono-de-cristal.md).
- **Entregable:** ambos archivos, con enlaces recíprocos.
- **Aceptación y validación:** el plan fija `0.2.2`, no reescribe REQ-0003 como cumplido y no autoriza una firma de Apple.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** no paralelizable con el cambio de versión. Entrega a TASK-0022-02.
- **Resultado y evidencia:** archivos creados el 2026-11-16, antes de editar la versión.

## TASK-0022-02 — Alinear versión y documentación

- **Referencia al plan:** paso 2.
- **Objetivo y alcance:** poner `0.2.2` en el crate. Actualizar changelog, README, `AGENTS.md` y el estado de REQ-0003. No hacer que [PLAN-0020](PLAN-0020-publicar-0.2.1.md) parezca el de esta release.
- **Entradas y dependencias:** TASK-0022-01.
- **Entregable:** un lector no cree que la 0.2.1 sigue siendo la última ventana publicada, ni que esta ventana cumple REQ-0003.
- **Aceptación y validación:** `Cargo.toml` dice `0.2.2`. El README enlaza `v0.2.2` y dice que no está firmada ni notarizada. `git diff --check` no reporta espacios conflictivos.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** un agente. Entrega a TASK-0022-03.
- **Resultado y evidencia:** el 2026-11-16, `src-tauri/Cargo.toml` dice `0.2.2`. El README enlaza `v0.2.2` y dice que el disco no está firmado ni notarizado. REQ-0003 sigue pedido. No hay RD.

## TASK-0022-03 — Empaquetar y publicar

- **Referencia al plan:** paso 3.
- **Objetivo y alcance:** pasar `cargo test --lib`, compilar en release, armar el `.dmg` arm64, abrir el `.app`, cerrarlo, empujar `main` y crear la release `v0.2.2` con el disco y las sumas. No dejar el proceso en ejecución. No firmar con un certificado de Apple.
- **Entradas y dependencias:** TASK-0022-02.
- **Entregable:** el tag y los dos archivos en GitHub. `HEAD` coincide con `origin/main` en el commit de la versión; el registro del SHA va en el commit siguiente.
- **Aceptación y validación:** CoreGraphics lista la ventana del `.app` como `title=Cristal`. El `AppIcon.icns` del disco es el cristal plano, no la gota de Tauri. Tras `kAEQuitApplication` no queda un proceso `cristal` de esa app. La release apunta al commit empujado. Las notas incluyen el SHA-256 del `.dmg`.
- **Responsable y estado:** según listado.
- **Paralelismo, límites y entrega:** cierre. Entrega a la persona usuaria.
- **Resultado y evidencia:** el 2026-11-16, `cargo test --lib` pasó 14 pruebas y 0 fallos. No cambió aserciones, así que no hay ficha nueva. `cargo build --release` produjo un binario arm64. El `.app` del `.dmg` dice versión 0.2.2 e identificador `com.limpiarmymac.cristal`. El `AppIcon.icns` tiene la misma suma que `src-tauri/icons/icon.icns` y un pixel de la esquina en 15,23,42. Abrió una ventana que CoreGraphics listó como `title=Cristal`, 1080×741, en `488,287`. `kAEQuitApplication` cerró ese proceso y no quedó `cristal`. `c8709ea` quedó en `origin/main`. La release https://github.com/zknchile/LimpiarMyMac/releases/tag/v0.2.2 apunta a ese commit y adjunta el `.dmg` y las sumas. SHA-256: `70da17965169d3b434c39ea00c673bd4bf77b64c64baf6d01a3c827a6dd4cd97`. Este cierre es el commit siguiente.
