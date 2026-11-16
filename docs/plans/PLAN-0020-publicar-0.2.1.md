# PLAN-0020 — Publicar Cristal 0.2.1

- **Fecha:** 2026-11-16
- **Estado:** completado
- **Solicitud:** actualizar la documentación, subir los cambios a GitHub y publicar un nuevo release.
- **Tareas:** [TASKS-0020](TASKS-0020-publicar-0.2.1.md)

Este documento no cambia la app al escribirse. La ventana vigente ya tiene la paleta de día lluvioso, la sombra de los botones y la franja del menú, según [PLAN-0017](PLAN-0017-paleta-dia-lluvioso.md), [PLAN-0018](PLAN-0018-profundidad-de-botones.md) y [PLAN-0019](PLAN-0019-franja-del-menu.md). Esos planes no subieron la versión. [PLAN-0013](PLAN-0013-publicar-0.2.0.md) ya publicó la 0.2.0.

## Objetivo, alcance y criterios de aceptación

Dejar en GitHub el aspecto actual y una release 0.2.1 para Apple Silicon. La documentación deja de presentar la 0.2.0 como la última.

Criterios observables:

- `src-tauri/Cargo.toml` dice `0.2.1`. No es `0.3.0`: el análisis y la limpieza no cambian. No se reescribe la 0.2.0: esa release ya está en GitHub.
- El README enlaza `v0.2.1` y dice que el `.dmg` no está firmado con un certificado de Apple ni notarizado, y que no hay binario para Mac Intel.
- El changelog registra la publicación sin borrar las notas de la 0.2.0 ni de la paleta.
- [REQ-0003](../requirements/baseline.md) sigue pedido y sin cumplir. La 0.2.1 no se presenta como Liquid Glass. No hay RD.
- `cargo test --lib` pasa antes de empaquetar.
- El `.dmg` contiene `Cristal.app` para arm64. Abrirlo muestra una ventana titulada Cristal. Cerrarla no deja el proceso.
- `main` queda en `origin/main`. La release `v0.2.1` apunta al commit de la versión y adjunta el `.dmg` y las sumas SHA-256.

Fuera de alcance: firmar con un certificado de Apple, notarizar, un binario Intel, cambiar `safety.rs` o reescribir REQ-0003 como cumplido.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `src-tauri/Cargo.toml` en `0.2.0`, la release `v0.2.0` y `scripts/package_macos.sh`. El script lee la versión del crate. El artefacto anterior fue `Cristal_0.2.0_aarch64.dmg` más las sumas. El identificador sigue siendo `com.limpiarmymac.cristal`.

Los cambios de color, sombra y franja están en el árbol y no están en `origin/main`. `origin/main` es `639db3a`.

Supuesto: la persona quiere el mismo tipo de artefacto que la 0.2.0, no solo el tag. No hay pregunta abierta.

## Diseño, responsabilidades, contratos y alternativas

- La versión vive en `src-tauri/Cargo.toml`. El `Info.plist` del `.app` copia ese número. El identificador no cambia, para que sustituya la app anterior si se instala en el mismo sitio.
- El empaquetado es el de [PLAN-0013](PLAN-0013-publicar-0.2.0.md). Se firma en local con `codesign -s -`. Eso no es un certificado de Apple. Las notas lo dicen.
- El commit de la versión incluye el aspecto que todavía no está en GitHub. El commit que registra el SHA de la release es el siguiente, como en la 0.2.0. El tag apunta al primero.
- Alternativa rechazada: publicar `0.3.0`. No cambia el contrato de limpieza ni el motor de la ventana.
- Alternativa rechazada: sustituir los archivos de `v0.2.0`. Esa release ya está publicada.
- Alternativa rechazada: un tag sin `.dmg`. La 0.2.0 adjuntó el disco.
- Alternativa rechazada: un RD que dé por aceptado el incumplimiento de REQ-0003. La persona no retiró ese pedido.

## Pasos y entregables

1. Guardar este plan y su listado — [TASK-0020-01](TASKS-0020-publicar-0.2.1.md#task-0020-01--dejar-el-plan).
2. Alinear versión y documentación — [TASK-0020-02](TASKS-0020-publicar-0.2.1.md#task-0020-02--alinear-versión-y-documentación). Depende del paso 1.
3. Empaquetar, comprobar la ventana y publicar — [TASK-0020-03](TASKS-0020-publicar-0.2.1.md#task-0020-03--empaquetar-y-publicar). Depende del paso 2.

## Riesgos, seguridad y bloqueos

- Riesgo: subir un binario de depuración. Mitigación: `cargo build --release` después del corte de versión, y abrir el `.app` del disco antes de crear la release.
- Riesgo: dejar Cristal en ejecución, o abrir la ventana con el puntero sobre Analizar. Mitigación: cerrar el PID con `kAEQuitApplication`, mover el puntero y comprobar que no queda `cristal`.
- No se amplía la lista blanca. No hay secretos. No hay bloqueo.

## Validación y evidencia esperada

`cargo test --lib` en el host, con `DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer`. No es un caso nuevo del catálogo: no cambia aserciones. La ventana del `.app` se anota en la tarea, sin ID. `git diff --check` no reporta espacios conflictivos. Falla si el `.dmg` no abre una ventana titulada Cristal, o si la release no apunta al commit empujado.

## Entrega, documentación, decisiones y recuperación

Changelog, README, `AGENTS.md` y el estado de REQ-0003. Sin ADR nuevo y sin RD. Recuperación: borrar la release y el tag, y revertir el commit de la versión. El `.dmg` no se versiona.

## Seguimiento y resultados

Completado el 2026-11-16. `cargo test --lib` pasó 14 pruebas y 0 fallos. El `.app` del disco, versión 0.2.1 e identificador `com.limpiarmymac.cristal`, abrió una ventana que CoreGraphics listó como `title=Cristal`. `kAEQuitApplication` cerró ese proceso y no quedó `cristal`. La release es https://github.com/zknchile/LimpiarMyMac/releases/tag/v0.2.1, sobre `cff7c7a`. El SHA-256 del `.dmg` es `50394dfd3cc6a6f25c3290c3082b2dd9545a8f4d4f644ec6c3b86e791755d495`. No está firmada con un certificado de Apple ni notarizada. No hay binario Intel. REQ-0003 no se reescribió como cumplido.
