# Pruebas

La política de fichas y de ejecuciones es la del marco: [documentación obligatoria de pruebas](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/testing/TESTING.md#documentación-obligatoria-de-pruebas). Aquí se anota cómo corre Cristal, que no es una SPA ni un servicio en Docker.

- Casos: [TEST-CASES.md](TEST-CASES.md). Cada prueba automatizada tiene un ID `TEST-<componente>-<consecutivo>`.
- Ejecuciones usadas como evidencia: [TEST-RUNS.md](TEST-RUNS.md). Un conteo en un plan cerrado no sustituye ese registro.
- Una ficha no significa que la prueba haya pasado. El estado de implementación y la evidencia van separados.

No se inventan casos de interfaz, de análisis del disco real ni de restauración desde la Papelera. Esos faltantes están al final del catálogo, sin ID.

## Dónde se ejecutan

En el host, no en Docker. Un contenedor no abre la ventana Liquid Glass. La desviación está en [ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md).

Reglas de rutas, inventario y tamaño:

```bash
cd src-tauri
cargo test --lib
```

Presentación de la selección, sin abrir la ventana:

```bash
npm test
```

`cargo check` comprueba que el crate compila. No es un caso de comportamiento y no tiene ID.

## Qué cubren y qué no

Las fichas cubren la lista blanca, el plan de un contenedor de prueba, el emparejamiento de nombres, el texto del tamaño seleccionado y la decisión de no subir la grilla al marcar. No cubren un análisis del disco de la persona, el comando que mueve a la Papelera ni el clic en la ventana.

## Manual

La persona usuaria abrió la app el 2026-10-15 y dijo que la ventana quedó muy buena. No hay guion ni captura, así que no es un caso del catálogo.

Pendiente, sin afirmar que se haya hecho: contraste en claro y oscuro, `prefers-reduced-transparency`, y una limpieza real con restauración desde la Papelera.

## Al cambiar una prueba

En el mismo cambio: actualizar la ficha, el comentario `TEST-...` del fuente y, si la ejecución sirve de evidencia, [TEST-RUNS.md](TEST-RUNS.md) o la tarea que la cierra. No reutilizar un ID retirado. No atribuir a una revisión nueva un resultado anterior.
