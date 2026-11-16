# Pruebas

La política de fichas y de ejecuciones es la del marco: [documentación obligatoria de pruebas](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/testing/TESTING.md#documentación-obligatoria-de-pruebas). Aquí se anota cómo corre Cristal, que no es una SPA ni un servicio en Docker.

- Casos: [TEST-CASES.md](TEST-CASES.md). Cada prueba automatizada tiene un ID `TEST-<componente>-<consecutivo>`.
- Ejecuciones usadas como evidencia: [TEST-RUNS.md](TEST-RUNS.md). Un conteo en un plan cerrado no sustituye ese registro.
- Una ficha no significa que la prueba haya pasado. El estado de implementación y la evidencia van separados.

No se inventan casos de interfaz, de análisis del disco real ni de restauración desde la Papelera. Esos faltantes están al final del catálogo, sin ID.

## Dónde se ejecutan

En el host, no en Docker. Un contenedor no abre la ventana de macOS. La desviación está en [ADR-0003](../decisions/ADR-0003-vista-gpui.md).

La suite es una sola:

```bash
cd src-tauri
cargo test --lib
```

Cubre la lista blanca, el inventario de apps, el tamaño, la selección y el sitio de la grilla. No abre la ventana. Node no forma parte de los requisitos.

`cargo check` comprueba que el crate compila. No es un caso de comportamiento y no tiene ID.

## Qué cubren y qué no

Las fichas cubren la lista blanca, el plan de un contenedor de prueba, el emparejamiento de nombres, el texto del tamaño seleccionado y la decisión de no subir la grilla al marcar. No cubren un análisis del disco de la persona, el comando que mueve a la Papelera ni el clic en la ventana.

## Manual

La persona usuaria abrió la ventana de Tauri el 2026-10-15 y dijo que quedó muy buena. Esa ventana ya no es la del binario. No hay guion ni captura, así que no es un caso del catálogo. El cierre de [PLAN-0012](../plans/PLAN-0012-migrar-vista-a-gpui.md) anota que la ventana GPUI se abrió; eso tampoco es un caso del catálogo.

Pendiente, sin afirmar que se haya hecho: contraste en claro y oscuro, `prefers-reduced-transparency`, y una limpieza real con restauración desde la Papelera.

## Al cambiar una prueba

En el mismo cambio: actualizar la ficha, el comentario `TEST-...` del fuente y, si la ejecución sirve de evidencia, [TEST-RUNS.md](TEST-RUNS.md) o la tarea que la cierra. No reutilizar un ID retirado. No atribuir a una revisión nueva un resultado anterior.
