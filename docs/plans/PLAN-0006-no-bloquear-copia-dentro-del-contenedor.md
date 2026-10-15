# PLAN-0006 — No bloquear una copia que vive dentro de otra app

- **Fecha:** 2026-10-15
- **Estado:** completado
- **Solicitud:** al limpiar, Cristal respondió «No se tocan datos del sistema de Apple» y no dijo qué ruta.
- **Tareas:** [TASKS-0006](TASKS-0006-no-bloquear-copia-dentro-del-contenedor.md)

## Objetivo, alcance y criterios de aceptación

Un archivo o carpeta llamado `com.apple…` que está dentro del contenedor de otra app no es el Llavero, Mail ni las preferencias del sistema. No debe tumbar la limpieza de ese contenedor. Los datos de Apple de verdad siguen fuera de la lista.

Criterios observables:

- `~/Library/Containers/com.utmapp.QEMUHelper/.../com.apple.security.plist` no se trata como dato de Apple. Si es un enlace, solo se quita el enlace.
- `~/Library/Preferences/com.apple.*.plist` y `~/Library/Containers/com.apple.mail` siguen prohibidos.
- Si algo se rechaza, el mensaje incluye la ruta.
- `cargo test --lib` cubre ambos casos. El agente no mueve el disco de la persona.

Hecho comprobado: el plan de `~/Library/Containers/com.utmapp.UTM` ya no incluye el registro interno y no produce ese mensaje. El mensaje sale cuando un hijo real se llama `com.apple.…` y la regla mira cualquier tramo del camino, no la app dueña.

## Fuentes, estado actual, supuestos y preguntas

Revisados: `safety.rs`, `clean.rs`, `src/main.js` y los contenedores de UTM en disco. No hay pregunta abierta. No corresponde un ADR nuevo ni un RD: la lista blanca no se amplía.

## Diseño, responsabilidades, contratos y alternativas

- La identidad es el nombre de la app bajo Containers, Group Containers, Application Support, Preferences o las otras raíces permitidas. `ByHost` usa el archivo de adentro.
- Un `com.apple` anidado en una app de terceros se puede mover con esa app.
- Alternativa rechazada: quitar el aviso por completo. Dejaría limpiar `com.apple.mail`.

## Pasos y entregables

1. Guardar este plan — [TASK-0006-01](TASKS-0006-no-bloquear-copia-dentro-del-contenedor.md#task-0006-01--dejar-el-plan).
2. Juzgar la app dueña, no cada archivo interno — [TASK-0006-02](TASKS-0006-no-bloquear-copia-dentro-del-contenedor.md#task-0006-02--juzgar-la-app-duena).
3. Probar y registrar — [TASK-0006-03](TASKS-0006-no-bloquear-copia-dentro-del-contenedor.md#task-0006-03--verificar-y-registrar).

## Riesgos, seguridad y bloqueos

El riesgo es autorizar un contenedor de Apple con prefijo de equipo, por ejemplo `TEAM.com.apple.…`. Este plan no afloja ese caso: solo deja de mirar los hijos de una app que ya estaba permitida. No hay bloqueo.

## Validación y evidencia esperada

`cargo test --lib` en el host. La persona analiza de nuevo y limpia el elemento que falló. Si vuelve a fallar, el aviso nombra la ruta.

## Entrega, documentación, decisiones y recuperación

Actualizar seguridad, la consecuencia de [ADR-0002](../decisions/ADR-0002-lista-blanca-y-papelera.md), pruebas y changelog. Recuperación: revertir el cambio.

## Seguimiento y resultados

Completado el 2026-10-15. `cargo test --lib` pasó 8 pruebas. El plan real de `~/Library/Containers/com.utmapp.UTM` no produce ese mensaje. El agente no movió el contenedor.
