# PLAN-0013 — Publicar Cristal 0.2.0

- **Fecha:** 2026-11-16
- **Estado:** completado
- **Solicitud:** actualizar la documentación, subir el código a GitHub y publicar el nuevo release.
- **Tareas:** [TASKS-0013](TASKS-0013-publicar-0.2.0.md)

Este documento no cambia la app al escribirse. La ventana vigente ya es GPUI, según [PLAN-0012](PLAN-0012-migrar-vista-a-gpui.md). Ese plan no subió la versión ni creó una release. Esta publicación es posterior.

## Objetivo, alcance y criterios de aceptación

Dejar en GitHub el código de la ventana GPUI y una release 0.2.0 para Apple Silicon, con la documentación alineada a esa ventana.

Criterios observables:

- `src-tauri/Cargo.toml` dice `0.2.0`. No es un parche de la 0.1.1: cambia la ventana y deja de aplicar Liquid Glass.
- El README deja de decir que no hay release. Enlaza `v0.2.0` y dice que el `.dmg` no está firmado con un certificado de Apple ni notarizado, y que no hay binario para Mac Intel.
- El changelog registra la publicación. [PLAN-0012](PLAN-0012-migrar-vista-a-gpui.md) no se reescribe como si él hubiera publicado la versión.
- [REQ-0003](../requirements/baseline.md) sigue pidiendo Liquid Glass. Su estado deja de afirmar que la ventana publicada usa el material nativo. No hay RD: la persona no cambió el pedido; cambió el estado de lo implementado, ya decidido en [ADR-0003](../decisions/ADR-0003-vista-gpui.md).
- `cargo test --lib` pasa antes de empaquetar.
- El `.dmg` contiene `Cristal.app` para arm64. Abrirlo muestra una ventana titulada Cristal. Cerrarla no deja el proceso.
- `main` queda en `origin/main`. La release `v0.2.0` apunta a ese commit y adjunta el `.dmg` y las sumas SHA-256.

Fuera de alcance: firmar con un certificado de Apple, notarizar, un binario Intel, cambiar `safety.rs` o reescribir REQ-0003 como cumplido.

## Fuentes, estado actual, supuestos y preguntas

Revisados: [PLAN-0012](PLAN-0012-migrar-vista-a-gpui.md), `src-tauri/Cargo.toml` en `0.1.1`, las releases `v0.1.0` y `v0.1.1` en GitHub, y `tauri.conf.json` del commit `5a95fa5`. El identificador de aquellas apps era `com.limpiarmymac.cristal`. El artefacto era `Cristal_<versión>_aarch64.dmg` más `Cristal_<versión>_SHA256SUMS.txt`. Las notas decían que no estaba firmada ni notarizada.

El binario de `target/release/cristal` fechado el 2026-10-15 es el de Tauri. No sirve para esta release.

Supuesto: la persona quiere el mismo tipo de artefacto que las dos releases anteriores, no solo el tag. No hay pregunta abierta.

## Diseño, responsabilidades, contratos y alternativas

- La versión vive en `src-tauri/Cargo.toml`. El `Info.plist` del `.app` copia ese número. El identificador sigue siendo `com.limpiarmymac.cristal`, para que sustituya la app anterior si se instala en el mismo sitio.
- El empaquetado arma `Cristal.app` con el binario de `cargo build --release`, el icono `src-tauri/icons/icon.icns` y un enlace a `/Applications`. No usa Tauri.
- Se firma en local con `codesign -s -`. Eso no es un certificado de Apple. Las notas lo dicen.
- Alternativa rechazada: publicar `0.1.2`. Un parche no describe el cambio de ventana.
- Alternativa rechazada: un tag sin `.dmg`. Las dos releases anteriores adjuntaron el disco.
- Alternativa rechazada: un RD que dé por aceptado el incumplimiento de REQ-0003. La persona no retiró ese pedido.

## Pasos y entregables

1. Guardar este plan y su listado — [TASK-0013-01](TASKS-0013-publicar-0.2.0.md#task-0013-01--dejar-el-plan).
2. Alinear versión y documentación — [TASK-0013-02](TASKS-0013-publicar-0.2.0.md#task-0013-02--alinear-versión-y-documentación).
3. Empaquetar, comprobar la ventana y publicar — [TASK-0013-03](TASKS-0013-publicar-0.2.0.md#task-0013-03--empaquetar-y-publicar). Depende del paso 2.

## Riesgos, seguridad y bloqueos

- Riesgo: subir un binario de Tauri por error. Mitigación: recompilar después del corte y abrir el `.app` antes de crear la release.
- Riesgo: dejar Cristal en ejecución. Mitigación: cerrar el PID con `kAEQuitApplication` y comprobar que no queda `cristal`.
- No se amplía la lista blanca. No hay secretos. No hay bloqueo.

## Validación y evidencia esperada

`cargo test --lib` en el host, con `DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer`. No es un caso nuevo del catálogo: no cambia aserciones. La ventana del `.app` se anota en la tarea, sin ID. `git diff --check` no reporta espacios conflictivos. Falla si el `.dmg` no abre una ventana titulada Cristal, o si la release no apunta al commit empujado.

## Entrega, documentación, decisiones y recuperación

Changelog, README, `AGENTS.md` y el estado de REQ-0003. Sin ADR nuevo y sin RD. Recuperación: borrar la release y el tag, y revertir el commit de la versión. El `.dmg` no se versiona.

## Seguimiento y resultados

Completado el 2026-11-16. `cargo test --lib` pasó 14 pruebas y 0 fallos. El `.app` del disco abrió una ventana titulada Cristal. `kAEQuitApplication` cerró ese proceso y no quedó `cristal`. La release es https://github.com/zknchile/LimpiarMyMac/releases/tag/v0.2.0, sobre `b7b7381`. El SHA-256 del `.dmg` es `2670bae5496c0aef2507f60523b8bf20019e8c277010c24531528e68834bc03e`. No está firmada con un certificado de Apple ni notarizada. No hay binario Intel. REQ-0003 no se reescribió como cumplido.
