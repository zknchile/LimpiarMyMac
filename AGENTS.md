# Cristal

Aplicación de escritorio para macOS. Analiza la carpeta del usuario y mueve a la Papelera solo lo que la persona confirma. El método de trabajo de los agentes sigue [Arquitectura_agent](https://github.com/zknchile/Arquitectura_agent). Este repositorio describe el producto implementado; no copies aquí el marco como si Cristal fuera una SPA SolidJS ni un servicio HTTP en contenedores.

# Sources of truth

- Marco de trabajo: [planificación](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/development/agent-planning.md), [decisiones](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/decisions/README.md), [política documental](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/development/documentation-policy.md), [pruebas](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/testing/TESTING.md#documentación-obligatoria-de-pruebas) y [Liquid Glass](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/design/liquid-glass.md).
- Producto: [requisitos](docs/requirements/baseline.md), [decisiones locales](docs/decisions/README.md), [diseño aplicado](docs/design/liquid-glass.md), [seguridad](docs/security/SECURITY.md), [pruebas](docs/testing/TESTING.md) y [casos](docs/testing/TEST-CASES.md).
- Trabajo: [planes](docs/plans/PLAN-0001-documentar-desarrollo.md). El desarrollo inicial está reconstruido en [PLAN-0002](docs/plans/PLAN-0002-registro-desarrollo-cristal.md); no era un plan previo a implementar. El tamaño de la selección se corrigió en [PLAN-0003](docs/plans/PLAN-0003-actualizar-tamano-seleccion.md). Los contenedores con enlaces se limpian según [PLAN-0004](docs/plans/PLAN-0004-contenedor-sin-seguir-enlaces.md). El registro interno que macOS no deja borrar se omite según [PLAN-0005](docs/plans/PLAN-0005-omitir-metadata-de-contenedor.md). Una copia `com.apple` dentro de otra app no se trata como dato de Apple según [PLAN-0006](docs/plans/PLAN-0006-no-bloquear-copia-dentro-del-contenedor.md). La nota de ese registro se muestra entera según [PLAN-0007](docs/plans/PLAN-0007-nota-del-registro-visible.md). Salir está abajo a la derecha de Analizar según [PLAN-0008](docs/plans/PLAN-0008-salir-junto-a-analizar.md). Las pruebas automatizadas se fichan según [PLAN-0009](docs/plans/PLAN-0009-documentacion-de-pruebas.md). La grilla no sube al marcar, según [PLAN-0010](docs/plans/PLAN-0010-grilla-sin-saltar.md). El diagrama de componentes está en el README según [PLAN-0011](docs/plans/PLAN-0011-diagrama-de-componentes.md). La vista pasó a GPUI según [PLAN-0012](docs/plans/PLAN-0012-migrar-vista-a-gpui.md). No aplica Liquid Glass y no cumple REQ-0003. La 0.2.0 se publicó según [PLAN-0013](docs/plans/PLAN-0013-publicar-0.2.0.md). La 0.2.1 se publicó según [PLAN-0020](docs/plans/PLAN-0020-publicar-0.2.1.md). El diagrama del README describe un solo binario según [PLAN-0014](docs/plans/PLAN-0014-diagrama-de-la-ventana-gpui.md). Los colores de la ventana son la paleta de día lluvioso según [PLAN-0017](docs/plans/PLAN-0017-paleta-dia-lluvioso.md). El velo no es Liquid Glass. Los botones tienen sombra según [PLAN-0018](docs/plans/PLAN-0018-profundidad-de-botones.md). Esa sombra tampoco es vidrio. La franja del menú es un poco más oscura según [PLAN-0019](docs/plans/PLAN-0019-franja-del-menu.md). No es vidrio.
- Evolución: [CHANGELOG.md](CHANGELOG.md).

# Documentation routing

- Antes de cambiar código, guarda un plan y su listado en `docs/plans/`, con enlaces recíprocos, según el marco. Una lista en el chat no basta.
- Si cambia un requisito confirmado, actualiza la línea base y registra un RD cuando haya un requisito anterior identificable.
- Si cambia una decisión arquitectónica, actualiza el ADR vigente o crea el siguiente. No redactes decisiones retrospectivas sin contexto, alternativa y motivo comprobables.
- Para la interfaz, lee el diseño aplicado y la referencia Liquid Glass del marco. No atribuyas a CSS el material nativo de Apple.
- Para borrado, rutas o permisos, lee seguridad y `src-tauri/src/safety.rs` antes de ampliar la lista de rutas.
- Si añades, cambias o retiras una prueba, actualiza [casos](docs/testing/TEST-CASES.md) en el mismo cambio y registra la ejecución en la tarea o en [TEST-RUNS.md](docs/testing/TEST-RUNS.md). No inventes casos ni des por aprobada una ficha sin salida del runner.
- No confirmes secretos ni archivos `.env`.

# Hechos de este repositorio

- La lógica de análisis y limpieza está en Rust, dentro de `src-tauri/`. No hay componente de IA ni runtime Python.
- `cargo run` en `src-tauri` abre una ventana GPUI. No es SolidJS, ni Atomic Design, ni HTML. No usa `NSGlassEffectView` y no cumple [REQ-0003](docs/requirements/baseline.md). La decisión vigente es [ADR-0003](docs/decisions/ADR-0003-vista-gpui.md). [ADR-0001](docs/decisions/ADR-0001-app-escritorio-tauri.md) quedó sustituido.
- No hay servicio HTTP. El análisis y la vista están en el mismo proceso.
- El desarrollo y las pruebas se ejecutan en el host. Un contenedor no abre la ventana de macOS. La desviación está en [ADR-0003](docs/decisions/ADR-0003-vista-gpui.md), que conserva lo de [ADR-0001](docs/decisions/ADR-0001-app-escritorio-tauri.md) sobre el host, HTTP y Docker.
