# PLAN-0005 — No fallar por el registro interno del contenedor

- **Fecha:** 2026-10-15
- **Estado:** completado
- **Solicitud:** al limpiar `~/Library/Containers/com.utmapp.UTM`, Cristal dijo que 1 elemento no se pudo mover. El detalle apunta a `.com.apple.containermanagerd.metadata.plist`.
- **Tareas:** [TASKS-0005](TASKS-0005-omitir-metadata-de-contenedor.md)

## Objetivo, alcance y criterios de aceptación

Ese archivo es un registro de macOS, no la máquina virtual. No se puede borrar ni con `rm` desde la terminal. La limpieza de los archivos reales no debe marcarse como fallo por dejarlo. No se amplía la lista blanca, no se pide Acceso total al disco para este caso y no se usa `rm` sobre datos de la persona.

Criterios observables:

- El plan de limpieza no incluye `.com.apple.containermanagerd.metadata.plist` ni entrega a Finder una carpeta que lo contenga.
- El tamaño ofrecido no cuenta ese archivo. Un contenedor que solo conserva ese registro no vuelve a aparecer.
- Si macOS bloquea otro archivo real, el mensaje sigue en español. Este plan no cambia ese caso.
- `cargo test --lib` cubre el plan con una carpeta temporal. El agente no mueve el contenedor real.

Hecho comprobado el 2026-10-15: después del intento, `Data/Documents` ya no está en el contenedor. Quedan el plist de 30 KB, dos `.DS_Store` y carpetas vacías. `rm` sobre el plist responde `Operation not permitted`. El error de la ventana citaba ese plist y el código `-5000` de Finder.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `src-tauri/src/clean.rs`, `safety.rs`, `size.rs`, `scan.rs` y el contenedor en disco. [PLAN-0004](PLAN-0004-contenedor-sin-seguir-enlaces.md) ya evita seguir enlaces; no cubría este archivo, que no es un enlace. No hay pregunta abierta.

No corresponde un ADR ni un RD. La decisión sigue siendo Papelera y lista blanca. Cambia el trato de un archivo que macOS no deja mover.

## Diseño, responsabilidades, contratos y alternativas

- `safety.rs` reconoce el registro por el nombre `.com.apple.containermanagerd.*.plist` y no lo pone en el plan. Una carpeta que lo contiene se parte, para que Finder no rechace el resto.
- `size.rs` no suma ese archivo. Así no se promete espacio que no se puede recuperar.
- `clean.rs` lo salta si aparece en un paso. Si la limpieza del resto sale bien, la ventana lo dice como nota, no como error.
- Alternativa rechazada: pedir Acceso total al disco. El `rm` de la terminal ya falló; el mensaje actual manda a un ajuste que no resuelve este archivo.
- Alternativa rechazada: borrarlo con privilegios de administrador. Contradice la Papelera y no está autorizado.

## Pasos y entregables

1. Guardar este plan — [TASK-0005-01](TASKS-0005-omitir-metadata-de-contenedor.md#task-0005-01--dejar-el-plan).
2. Omitir el registro y no contar su tamaño — [TASK-0005-02](TASKS-0005-omitir-metadata-de-contenedor.md#task-0005-02--no-tratar-el-registro-como-fallo).
3. Probar y registrar — [TASK-0005-03](TASKS-0005-omitir-metadata-de-contenedor.md#task-0005-03--verificar-y-registrar).

## Riesgos, seguridad y bloqueos

El riesgo es saltarse un archivo de la persona porque el nombre se parece. La mitigación es el prefijo exacto de containermanagerd. No hay bloqueo. Este plan no toca el contenedor real.

## Validación y evidencia esperada

`cargo test --lib` en el host. Falla si el plan incluye el plist o si el tamaño lo cuenta. La persona comprueba la ventana: analizar de nuevo y, si el resto sigue listado, limpiarlo. La carpeta puede quedar con el registro interno.

## Entrega, documentación, decisiones y recuperación

Actualizar seguridad, la consecuencia de [ADR-0002](../decisions/ADR-0002-lista-blanca-y-papelera.md), las pruebas y el changelog. Recuperación: revertir el cambio. Lo que ya está en la Papelera se restaura desde ahí.

## Seguimiento y resultados

Completado el 2026-10-15. `cargo test --lib` pasó 7 pruebas, incluida la que omite el plist de containermanagerd. El agente no movió `~/Library/Containers/com.utmapp.UTM`. La persona tiene que analizar de nuevo: el registro interno ya no debe aparecer como error.
