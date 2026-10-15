# Cristal

Aplicación de escritorio para macOS. Analiza la carpeta del usuario y mueve a la Papelera solo lo que la persona confirma. El método de trabajo de los agentes sigue [Arquitectura_agent](https://github.com/zknchile/Arquitectura_agent). Este repositorio describe el producto implementado; no copies aquí el marco como si Cristal fuera una SPA SolidJS ni un servicio HTTP en contenedores.

# Sources of truth

- Marco de trabajo: [planificación](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/development/agent-planning.md), [decisiones](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/decisions/README.md), [política documental](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/development/documentation-policy.md) y [Liquid Glass](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/design/liquid-glass.md).
- Producto: [requisitos](docs/requirements/baseline.md), [decisiones locales](docs/decisions/README.md), [diseño aplicado](docs/design/liquid-glass.md), [seguridad](docs/security/SECURITY.md) y [pruebas](docs/testing/TESTING.md).
- Trabajo: [planes](docs/plans/PLAN-0001-documentar-desarrollo.md). El desarrollo inicial está reconstruido en [PLAN-0002](docs/plans/PLAN-0002-registro-desarrollo-cristal.md); no era un plan previo a implementar. El tamaño de la selección se corrigió en [PLAN-0003](docs/plans/PLAN-0003-actualizar-tamano-seleccion.md). Los contenedores con enlaces se limpian según [PLAN-0004](docs/plans/PLAN-0004-contenedor-sin-seguir-enlaces.md). El registro interno que macOS no deja borrar se omite según [PLAN-0005](docs/plans/PLAN-0005-omitir-metadata-de-contenedor.md). Una copia `com.apple` dentro de otra app no se trata como dato de Apple según [PLAN-0006](docs/plans/PLAN-0006-no-bloquear-copia-dentro-del-contenedor.md). La nota de ese registro se muestra entera según [PLAN-0007](docs/plans/PLAN-0007-nota-del-registro-visible.md).
- Evolución: [CHANGELOG.md](CHANGELOG.md).

# Documentation routing

- Antes de cambiar código, guarda un plan y su listado en `docs/plans/`, con enlaces recíprocos, según el marco. Una lista en el chat no basta.
- Si cambia un requisito confirmado, actualiza la línea base y registra un RD cuando haya un requisito anterior identificable.
- Si cambia una decisión arquitectónica, actualiza el ADR vigente o crea el siguiente. No redactes decisiones retrospectivas sin contexto, alternativa y motivo comprobables.
- Para la interfaz, lee el diseño aplicado y la referencia Liquid Glass del marco. No atribuyas a CSS el material nativo de Apple.
- Para borrado, rutas o permisos, lee seguridad y `src-tauri/src/safety.rs` antes de ampliar la lista de rutas.
- No confirmes secretos ni archivos `.env`.

# Hechos de este repositorio

- La lógica de análisis y limpieza está en Rust, dentro de `src-tauri/`. No hay componente de IA ni runtime Python.
- La ventana es Tauri 2. El frontend es HTML, CSS y JavaScript, no SolidJS ni Atomic Design.
- Frontend y backend no se hablan por HTTP. Tauri invoca comandos Rust en el mismo proceso.
- El desarrollo y las pruebas se ejecutaron en el host. Un contenedor no abre la ventana Liquid Glass de macOS. La desviación está en [ADR-0001](docs/decisions/ADR-0001-app-escritorio-tauri.md).
