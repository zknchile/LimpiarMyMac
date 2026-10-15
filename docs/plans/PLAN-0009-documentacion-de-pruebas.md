# PLAN-0009 — Documentar las pruebas de Cristal

- **Fecha:** 2026-10-15
- **Estado:** completado
- **Solicitud:** aplicar en este repositorio la documentación de pruebas que Arquitectura_agent añadió el mismo día.
- **Tareas:** [TASKS-0009](TASKS-0009-documentacion-de-pruebas.md)

## Objetivo, alcance y criterios de aceptación

Dejar un catálogo de las pruebas que el código ya tiene, y un registro de la ejecución usada para entregar este cambio. La fuente normativa sigue siendo la del marco: [documentación obligatoria de pruebas](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/testing/TESTING.md#documentación-obligatoria-de-pruebas), ampliada en su [RD-0002](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/decisions/RD-0002-documentacion-de-pruebas.md).

Criterios observables:

- `docs/testing/TEST-CASES.md` ficha cada prueba automatizada existente, con ID estable `TEST-<componente>-<consecutivo>` y los ocho campos del marco.
- El ID está en el test o la correspondencia del catálogo es inequívoca por archivo y nombre.
- `docs/testing/TEST-RUNS.md` registra la ejecución de esta entrega: revisión, entorno, comando, alcance, resultado y salida. No reutiliza un conteo viejo como si acreditara este cambio.
- [TESTING.md](../testing/TESTING.md), [AGENTS.md](../../AGENTS.md) y el README apuntan al catálogo y a la política. Un agente futuro sabe dónde documentar una prueba nueva.
- No aparece un ID para el botón Salir, el contraste, un análisis del disco real ni una limpieza restaurada. Esos faltantes se nombran, no se inventan.
- No cambia qué se analiza ni qué se mueve a la Papelera.

Fuera de alcance: suite nueva, Docker, CI, umbral de cobertura y reconstruir los planes cerrados.

## Fuentes, estado actual, supuestos y preguntas

Revisados en el marco, commit de `main` del 2026-10-15: `docs/testing/TESTING.md`, `docs/decisions/RD-0002-documentacion-de-pruebas.md`, `docs/plans/PLAN-0009-documentacion-de-pruebas.md`, `docs/development/documentation-policy.md` y `docs/development/agent-planning.md`.

Revisados aquí: [TESTING.md](../testing/TESTING.md), las pruebas de `safety.rs`, `apps.rs`, `size.rs` y `src/selection.test.js`, y [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md). Hoy el documento local dice que no hay IDs `TEST-...` y mezcla la estrategia con un conteo del mismo día. Hay 11 pruebas automatizadas. No hay preguntas abiertas: la persona pidió implementar la guía, no añadir casos.

Supuesto: el catálogo vive en este repositorio, no en el marco. El marco no debe recibir fichas de Cristal.

## Diseño, responsabilidades, contratos y alternativas

- El catálogo describe el caso. El registro de ejecución demuestra lo observado. [TESTING.md](../testing/TESTING.md) queda como estrategia local y enlace, no como lista de resultados.
- Componentes de ID: `safety`, `apps`, `size` y `selection`, en el orden del código. No se reutilizan.
- La reproducción dice el comando en el host. Docker queda fuera por [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md); copiar aquí la receta del marco afirmaría un contenedor que esta app no usa.
- Un agente mantiene todos los archivos. No hay delegación.
- Alternativa rechazada: dejar los nombres solo en [TESTING.md](../testing/TESTING.md). No cumple los campos ni separa el caso de la ejecución.
- Alternativa rechazada: inventar fichas de interfaz para cuadrar el formato. El marco lo prohíbe.
- No corresponde un RD local. [REQ-0007](../requirements/baseline.md) ya pide seguir el marco; el cambio de requisito está en el RD-0002 de Arquitectura_agent. No cambia la limpieza ni la arquitectura.

## Pasos y entregables

1. Guardar este plan y su listado, y fichar las 11 pruebas — [TASK-0009-01](TASKS-0009-documentacion-de-pruebas.md#task-0009-01--fichar-las-pruebas-existentes).
2. Alinear las fuentes, ejecutar las suites, registrar la salida y cerrar — [TASK-0009-02](TASKS-0009-documentacion-de-pruebas.md#task-0009-02--integrar-y-verificar). Depende del paso 1.

## Riesgos, seguridad y bloqueos

- Riesgo: tratar un caso escrito como prueba aprobada, o un porcentaje inexistente como inventario. Mitigación: el estado de implementación y la evidencia van en campos distintos.
- Las pruebas de ruta usan `/Users/test` o un directorio temporal. No se documentan rutas reales del disco de la persona.
- No hay secretos. No hay bloqueo.

## Validación y evidencia esperada

`cargo test --lib` en `src-tauri` y `npm test` en la raíz, en el host, después de enlazar los IDs. La salida entra en [TEST-RUNS.md](../testing/TEST-RUNS.md). Además, enlaces locales de los archivos de este plan, IDs únicos y `git diff --check`. Falla si una prueba del código no tiene ficha, si una ficha no tiene test, o si se asigna un ID a un procedimiento que no existe.

## Entrega, documentación, decisiones y recuperación

Se actualizan [TESTING.md](../testing/TESTING.md), [AGENTS.md](../../AGENTS.md), [README.md](../../README.md), [CHANGELOG.md](../../CHANGELOG.md) y la nota de [REQ-0007](../requirements/baseline.md). La frase de [RD-0001](../decisions/RD-0001-salir-readme-github.md) que dejaba un `TEST-...` pendiente pasa a señalar el faltante, sin crear el ID. [SECURITY.md](../security/SECURITY.md) remite al catálogo cuando cambien las pruebas de rutas. Sin ADR ni migración. Recuperación: revertir este cambio documental; las pruebas de código siguen igual salvo el comentario del ID.

## Seguimiento y resultados

Completado el 2026-10-15. Las 11 pruebas tienen ficha e ID en el fuente. [RUN-2026-10-15-01](../testing/TEST-RUNS.md#run-2026-10-15-01) aprobó las dos suites en el host, sobre `4688269` más comentarios de ID. No se fichó Salir, el contraste, el disco real ni la Papelera.

Comprobación de esta revisión: los enlaces locales nuevos resuelven. `git diff --check` no reportó espacios conflictivos. Limitación ajena, no corregida: [PLAN-0006](PLAN-0006-no-bloquear-copia-dentro-del-contenedor.md) enlaza `duena` sin eñe y el título de la tarea lleva `dueña`.
