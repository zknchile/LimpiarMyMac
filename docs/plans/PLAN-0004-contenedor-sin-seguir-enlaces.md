# PLAN-0004 — Mover un contenedor sin seguir sus enlaces

- **Fecha:** 2026-10-15
- **Estado:** completado
- **Solicitud:** al limpiar `~/Library/Containers/com.utmapp.UTM`, macOS responde con un error de permisos.
- **Tareas:** [TASKS-0004](TASKS-0004-contenedor-sin-seguir-enlaces.md)

## Objetivo, alcance y criterios de aceptación

Esa carpeta se puede mandar a la Papelera sin tocar el Escritorio, las Descargas, el Llavero ni las demás carpetas a las que apunta. No se amplía la lista blanca y no se borra nada de forma permanente.

Criterios observables:

- Un contenedor con un enlace hacia fuera no se entrega entero a Finder.
- El plan de limpieza mueve los archivos reales y solo quita el enlace, nunca su destino.
- Si macOS sigue bloqueando un archivo real, el mensaje está en español y dice qué camino quedó protegido.
- `cargo test --lib` cubre el plan con una carpeta temporal. No se mueve el contenedor real de UTM desde el agente.

Hecho comprobado: el contenedor es del usuario y pesa unos 26 GB por una máquina virtual en `Data/Documents`. También tiene enlaces a `~/Desktop`, `~/Downloads`, `~/Library/Keychains` y otras carpetas personales. Finder ya puede tirar un archivo suelto de Descargas, así que el fallo no es el control de Finder en general. [ADR-0002](../decisions/ADR-0002-lista-blanca-y-papelera.md) ya prohíbe seguir enlaces que salgan de la lista blanca; el código solo lo comprobaba en la ruta de arriba.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `src-tauri/src/clean.rs`, `safety.rs`, `scan.rs`, el contenedor en disco y la crate `trash` 5.2.9. No hay pregunta abierta: la persona indicó la ruta y quiere limpiarla.

Rust sigue sin IA. No hay HTTP.

## Diseño, responsabilidades, contratos y alternativas

- `safety.rs` arma el plan: tirar a la Papelera, quitar un enlace o borrar una carpeta ya vacía.
- `clean.rs` ejecuta ese plan. Si Finder rechaza un archivo real por permisos, se reintenta con `NSFileManager`, que también usa la Papelera.
- Alternativa rechazada: pedir a Finder que borre el contenedor completo. Puede fallar por los enlaces o, en algunas versiones, resolverlos y mover carpetas personales.
- Alternativa rechazada: `rm` o privilegios de administrador. Contradice la Papelera y no salta la protección de macOS de forma aceptable.

## Pasos y entregables

1. Guardar este plan — [TASK-0004-01](TASKS-0004-contenedor-sin-seguir-enlaces.md#task-0004-01--dejar-el-plan).
2. Planificar y ejecutar la limpieza segura — [TASK-0004-02](TASKS-0004-contenedor-sin-seguir-enlaces.md#task-0004-02--no-seguir-enlaces-del-contenedor).
3. Probar y registrar — [TASK-0004-03](TASKS-0004-contenedor-sin-seguir-enlaces.md#task-0004-03--verificar-y-registrar).

## Riesgos, seguridad y bloqueos

El riesgo grave es seguir un enlace y mover el Escritorio o el Llavero. La mitigación es no recorrer enlaces y volver a comprobar que cada ruta sigue dentro de la carpeta elegida justo antes de tocarla. No hay bloqueo. Este plan no ejecuta la limpieza sobre el disco de la persona.

## Validación y evidencia esperada

`cargo test --lib` en el host. Falla si el plan incluye el destino de un enlace o si entrega el contenedor entero a la Papelera. La ventana queda para la persona: analizar de nuevo y limpiar esa ruta.

## Entrega, documentación, decisiones y recuperación

Actualizar seguridad, la consecuencia de [ADR-0002](../decisions/ADR-0002-lista-blanca-y-papelera.md) y el changelog. No es un requisito nuevo. Recuperación: revertir el commit; lo que ya esté en la Papelera se restaura desde ahí.

## Seguimiento y resultados

Completado el 2026-10-15. `cargo test --lib` pasó 5 pruebas, incluida la del contenedor con un enlace hacia el Escritorio. El agente no movió `~/Library/Containers/com.utmapp.UTM`. La persona tiene que analizar de nuevo y confirmar esa limpieza en la ventana.
