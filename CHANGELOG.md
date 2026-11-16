# Evolución del proyecto

## 2026-11-16

- Se publicó Cristal 0.2.2 para Apple Silicon. Conserva la ventana GPUI y sustituye la gota de Tauri por el cristal plano. No aplica Liquid Glass. El disco no está firmado con un certificado de Apple ni notarizado. [PLAN-0022](docs/plans/PLAN-0022-publicar-0.2.2.md).
- El Dock y el `.app` muestran un cristal plano de la paleta de día lluvioso. No es la gota de Tauri ni es vidrio, y no cumple [REQ-0003](docs/requirements/baseline.md). [PLAN-0021](docs/plans/PLAN-0021-icono-de-cristal.md).
- Se publicó Cristal 0.2.1 para Apple Silicon. Conserva la ventana GPUI y aplica la paleta de día lluvioso, la sombra de los botones y la franja del menú. No aplica Liquid Glass. El disco no está firmado con un certificado de Apple ni notarizado. [PLAN-0020](docs/plans/PLAN-0020-publicar-0.2.1.md).
- La franja del menú es `#D2DAE5` al 88 %, un poco más oscura que el velo. No es vidrio y no cumple [REQ-0003](docs/requirements/baseline.md). [PLAN-0019](docs/plans/PLAN-0019-franja-del-menu.md).
- Los botones habilitados llevan sombra de tinta. Es `BoxShadow` de GPUI, no vidrio, y no cumple [REQ-0003](docs/requirements/baseline.md). [PLAN-0018](docs/plans/PLAN-0018-profundidad-de-botones.md).
- La ventana usa la paleta de día lluvioso, con el fondo `#E2E8F0` al 88 %. La transparencia es el alfa de GPUI, no vidrio, y no cumple [REQ-0003](docs/requirements/baseline.md). [PLAN-0017](docs/plans/PLAN-0017-paleta-dia-lluvioso.md).
- La ventana usa la paleta Ocean Glass: fondo `#ECFEFF`, texto `#083344`, sello `#22D3EE`. El nombre no es vidrio y no cumple [REQ-0003](docs/requirements/baseline.md). [PLAN-0016](docs/plans/PLAN-0016-paleta-ocean-glass.md).
- La ventana GPUI usa una paleta cálida y poco saturada. El botón principal deja el azul `#1d4ed8`. No es vidrio y no cumple [REQ-0003](docs/requirements/baseline.md). [PLAN-0015](docs/plans/PLAN-0015-colores-mas-suaves.md).
- El README describe la arquitectura como un solo binario GPUI. No hay HTML ni comandos de Tauri. El análisis y la limpieza quedan fuera del hilo de pintado. [PLAN-0014](docs/plans/PLAN-0014-diagrama-de-la-ventana-gpui.md).
- Se publicó Cristal 0.2.0 para Apple Silicon. La ventana es GPUI y no aplica Liquid Glass. El disco no está firmado con un certificado de Apple ni notarizado. [PLAN-0013](docs/plans/PLAN-0013-publicar-0.2.0.md).
- La ventana que abre `cargo run` pasó a GPUI. Se quitó la vista WebKit. No se aplica `NSGlassEffectView` y no se cumple [REQ-0003](docs/requirements/baseline.md). [ADR-0003](docs/decisions/ADR-0003-vista-gpui.md) pasó a `Accepted`. [ADR-0001](docs/decisions/ADR-0001-app-escritorio-tauri.md) quedó `Superseded by ADR-0003`. [PLAN-0012](docs/plans/PLAN-0012-migrar-vista-a-gpui.md). Esa migración no subió la versión; la publicación es el plan siguiente.

## 2026-10-15

- El README muestra el diagrama de componentes: interfaz, comandos de Tauri, módulos Rust y macOS. No hay servicio HTTP. [PLAN-0011](docs/plans/PLAN-0011-diagrama-de-componentes.md).
- Marcar un elemento ya no sube la grilla al inicio. La lista se queda donde estaba el clic. [PLAN-0010](docs/plans/PLAN-0010-grilla-sin-saltar.md). La build para probarlo es la 0.1.1.
- Las pruebas automatizadas quedaron fichadas con ID, reproducción y una ejecución registrada, según la documentación de pruebas de Arquitectura_agent. [PLAN-0009](docs/plans/PLAN-0009-documentacion-de-pruebas.md). No se inventaron casos de interfaz ni de limpieza real.
- Salir quedó en la esquina inferior derecha, a la derecha de Analizar, y se ve en todas las pantallas. [PLAN-0008](docs/plans/PLAN-0008-salir-junto-a-analizar.md).
- La nota del registro interno ya no se corta. Si un contenedor solo conserva ese registro y un `.DS_Store`, no vuelve a listarse. [PLAN-0007](docs/plans/PLAN-0007-nota-del-registro-visible.md).
- Una copia `com.apple…` dentro de otra app ya no tumba la limpieza. Mail y las preferencias de Apple siguen prohibidos. Si algo se rechaza, el aviso dice la ruta. [PLAN-0006](docs/plans/PLAN-0006-no-bloquear-copia-dentro-del-contenedor.md).
- El registro interno de un contenedor ya no se cuenta como fallo. macOS no deja mover `.com.apple.containermanagerd.metadata.plist`; el resto de `~/Library/Containers/com.utmapp.UTM` sí. [PLAN-0005](docs/plans/PLAN-0005-omitir-metadata-de-contenedor.md).
- Un contenedor de app ya no se manda entero a Finder. Así se puede limpiar `~/Library/Containers/com.utmapp.UTM` sin seguir los enlaces al Escritorio, las Descargas o el Llavero. [PLAN-0004](docs/plans/PLAN-0004-contenedor-sin-seguir-enlaces.md).
- El tamaño a limpiar se actualiza al marcar o desmarcar. La barra ya no muestra solo el total encontrado, y 0 B ya no se sustituye por ese total. [PLAN-0003](docs/plans/PLAN-0003-actualizar-tamano-seleccion.md).
- Se documentó el desarrollo con el método de [Arquitectura_agent](https://github.com/zknchile/Arquitectura_agent): requisitos confirmados, ADR, RD, diseño aplicado, seguridad, pruebas y un registro de tareas. [PLAN-0001](docs/plans/PLAN-0001-documentar-desarrollo.md) es el plan de esa documentación. [PLAN-0002](docs/plans/PLAN-0002-registro-desarrollo-cristal.md) reconstruye el trabajo ya hecho; no existía antes de implementar.
- Se publicó Cristal en https://github.com/zknchile/LimpiarMyMac, commit `c9e6f67`. Ese commit incluye la app, el botón Salir y el README. La persona usuaria había pedido esos tres cambios después de probar la ventana.
- Se implementó la app pedida el mismo día: análisis previo, selección y movimiento a la Papelera, con ventana Liquid Glass. Las pruebas automáticas cubren la lista de rutas y el emparejamiento de apps. La persona usuaria confirmó que la ventana se abrió y le gustó.

Este registro empieza con la línea base documental. No reconstruye cambios anteriores a esa fecha porque el repositorio no existía.
