# PLAN-0007 — Mostrar completa la nota del registro interno

- **Fecha:** 2026-10-15
- **Estado:** completado
- **Solicitud:** la ventana cortó el aviso en «No se puede mover a la».
- **Tareas:** [TASKS-0007](TASKS-0007-nota-del-registro-visible.md)

## Objetivo, alcance y criterios de aceptación

Ese aviso no es un fallo. El texto tiene que leerse entero y decir que el resto ya está en la Papelera. Un contenedor que solo conserva el registro interno y un `.DS_Store` no vuelve a ofrecerse.

Criterios observables:

- La nota cabe en más de una línea. No usa el recorte de una sola línea del progreso.
- El texto dice que lo demás está en la Papelera.
- El tamaño no cuenta `.DS_Store` ni el registro de containermanagerd. Si no queda otra cosa, el contenedor no aparece.
- `cargo test --lib` cubre el tamaño. No se toca el contenedor real.

Hecho comprobado: `.detail` tiene `white-space: nowrap` y puntos suspensivos. El contenedor de UTM ya solo tiene el plist de 30 KB y un `.DS_Store` de 6 KB, por eso el análisis lo vuelve a listar.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `src/styles.css`, `src/main.js`, `clean.rs` y el contenedor en disco. No hay pregunta abierta. No es un ADR ni un RD.

## Diseño, responsabilidades, contratos y alternativas

- La nota de cierre usa otra clase, con salto de línea.
- `path_size` no suma `.DS_Store`. No es espacio recuperable y Finder lo vuelve a crear.
- Alternativa rechazada: alargar la ventana. El recorte seguiría en pantallas estrechas.

## Pasos y entregables

1. Guardar este plan — [TASK-0007-01](TASKS-0007-nota-del-registro-visible.md#task-0007-01--dejar-el-plan).
2. Mostrar la nota y no volver a ofrecer el cascarón — [TASK-0007-02](TASKS-0007-nota-del-registro-visible.md#task-0007-02--nota-completa).
3. Probar y registrar — [TASK-0007-03](TASKS-0007-nota-del-registro-visible.md#task-0007-03--verificar-y-registrar).

## Riesgos, seguridad y bloqueos

No se amplía lo que se puede borrar. No hay bloqueo.

## Validación y evidencia esperada

`cargo test --lib`. La persona analiza de nuevo: `com.utmapp.UTM` no debería salir si solo queda el registro.

## Entrega, documentación, decisiones y recuperación

Changelog y pruebas. Recuperación: revertir el cambio.

## Seguimiento y resultados

Completado el 2026-10-15. La nota salta de línea y el tamaño ignora `.DS_Store`. El agente no movió el contenedor real.
