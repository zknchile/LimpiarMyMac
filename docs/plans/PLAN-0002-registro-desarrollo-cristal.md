# PLAN-0002 — Registro del desarrollo de Cristal

- **Fecha del registro:** 2026-10-15
- **Estado:** completado como registro, no como plan previo
- **Solicitud original:** app visual en Rust para limpiar el Mac, con análisis previo y estilo Liquid Glass. Después: botón Salir, README y GitHub.
- **Tareas:** [TASKS-0002](TASKS-0002-registro-desarrollo-cristal.md)

Este archivo se escribió después de implementar, al documentar el trabajo con el método de Arquitectura_agent. No existía cuando se escribió el código. No es fuente normativa: el comportamiento vigente está en el código, los [requisitos](../requirements/baseline.md) y las [decisiones](../decisions/README.md).

## Objetivo, alcance y criterios de aceptación

Reconstruir, con evidencia, las tareas que produjeron el commit `c9e6f67`.

Criterios que sí se cumplieron, con evidencia:

- La app abre una ventana Liquid Glass y la persona usuaria la aceptó visualmente.
- El análisis ocurre antes de la limpieza y hace falta confirmar.
- `cargo test` pasó las pruebas de lista blanca y de nombres de apps.
- Salir, README y GitHub quedaron en `c9e6f67`.

Criterios no comprobados, y por eso no se dan por hechos:

- Que una limpieza real haya restaurado archivos desde la Papelera.
- Contraste WCAG o `prefers-reduced-transparency`.

## Fuentes, estado actual, supuestos y preguntas

Fuentes: pedidos de la sesión del 2026-10-15, código en `src-tauri/src/` y `src/`, commit `c9e6f67`. El marco se consultó después, no como entrada de aquella implementación.

Supuesto al construir: una app de escritorio local bastaba. No quedó ninguna pregunta de producto abierta en ese alcance.

## Diseño, responsabilidades, contratos y alternativas

Quedó registrado en [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md) y [ADR-0002](../decisions/ADR-0002-lista-blanca-y-papelera.md). El perfil es Rust de escritorio, sin responsabilidad de IA y sin runtime Python. El contrato interno son los comandos Tauri, no HTTP.

## Pasos y entregables

1. Instalar Rust y elegir Tauri — [TASK-0002-01](TASKS-0002-registro-desarrollo-cristal.md#task-0002-01--preparar-rust-y-el-stack).
2. Implementar análisis, lista blanca y emparejamiento de apps — [TASK-0002-02](TASKS-0002-registro-desarrollo-cristal.md#task-0002-02--analizar-con-lista-blanca).
3. Mover a la Papelera solo lo confirmado — [TASK-0002-03](TASKS-0002-registro-desarrollo-cristal.md#task-0002-03--limpiar-hacia-la-papelera).
4. Interfaz y material Liquid Glass — [TASK-0002-04](TASKS-0002-registro-desarrollo-cristal.md#task-0002-04--interfaz-liquid-glass).
5. Salir, README y publicación — [TASK-0002-05](TASKS-0002-registro-desarrollo-cristal.md#task-0002-05--salir-readme-y-github).
6. Verificación de cierre — [TASK-0002-06](TASKS-0002-registro-desarrollo-cristal.md#task-0002-06--verificación-del-desarrollo).

## Riesgos, seguridad y bloqueos

El riesgo principal, ya mitigado en código y no eliminado, es marcar como resto una carpeta con documentos. Ver [seguridad](../security/SECURITY.md). No hubo bloqueo de autorización: la persona pidió la app y luego los tres cambios.

## Validación y evidencia esperada

`cargo test` del 2026-10-15 y la frase de la persona usuaria de que la app se desplegó y quedó muy buena. Detalle en [pruebas](../testing/TESTING.md).

## Entrega, documentación, decisiones y recuperación

Entrega de código: https://github.com/zknchile/LimpiarMyMac commit `c9e6f67`. Este registro es la entrega documental posterior, pedida en [PLAN-0001](PLAN-0001-documentar-desarrollo.md). Recuperar el producto es revertir ese commit; recuperar solo la documentación es revertir el commit que añade `docs/`.

## Seguimiento y resultados

Registro cerrado. Desviación respecto del marco: no hubo plan ni listado antes de implementar. Se conserva esa limitación en lugar de reescribir la historia.
