# Evolución del proyecto

## 2026-10-15

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
