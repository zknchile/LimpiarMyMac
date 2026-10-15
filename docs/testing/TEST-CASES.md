# Casos de prueba

Catálogo de las pruebas automatizadas que existen en el código. La política es la del marco: [documentación obligatoria de pruebas](https://github.com/zknchile/Arquitectura_agent/blob/main/docs/testing/TESTING.md#documentación-obligatoria-de-pruebas). La estrategia local está en [TESTING.md](TESTING.md). Las ejecuciones usadas como evidencia están en [TEST-RUNS.md](TEST-RUNS.md).

El responsable de mantenimiento de todos estos casos es el mantenedor del repositorio. No hay suite de interfaz, de análisis del disco real ni de movimiento a la Papelera. Esos faltantes no tienen ID.

| ID | Componente | Test |
| --- | --- | --- |
| [TEST-safety-001](#test-safety-001) | safety | `allows_cache_child_and_rejects_documents` |
| [TEST-safety-002](#test-safety-002) | safety | `rejects_parent_escape_and_allows_installer` |
| [TEST-safety-003](#test-safety-003) | safety | `apple_support_is_private_but_apple_cache_is_not` |
| [TEST-safety-004](#test-safety-004) | safety | `container_plan_trashes_real_files_and_unlinks_escaping_links` |
| [TEST-safety-005](#test-safety-005) | safety | `nested_apple_file_in_third_party_container_is_not_private` |
| [TEST-safety-006](#test-safety-006) | safety | `container_plan_skips_containermanager_metadata` |
| [TEST-apps-001](#test-apps-001) | apps | `matches_apps_and_cli_tools` |
| [TEST-size-001](#test-size-001) | size | `path_size_ignores_container_metadata` |
| [TEST-selection-001](#test-selection-001) | selection | selección vacía en 0 B |
| [TEST-selection-002](#test-selection-002) | selection | marcar y desmarcar cambia el tamaño |
| [TEST-selection-003](#test-selection-003) | selection | 80 MB cambian un texto en GB |
| [TEST-place-001](#test-place-001) | place | la misma categoría conserva la grilla |
| [TEST-place-002](#test-place-002) | place | otra categoría no hereda la lista |
| [TEST-place-003](#test-place-003) | place | otra pantalla no reutiliza el sitio |

## TEST-safety-001

| Campo | Contenido |
| --- | --- |
| Identificación | TEST-safety-001. Un hijo de Caches se puede borrar; Documentos, el Llavero, la carpeta de inicio y la raíz de Caches no. Componente `safety`. Unitaria. |
| Objetivo y trazabilidad | Comprueba la lista blanca de [ADR-0002](../decisions/ADR-0002-lista-blanca-y-papelera.md) y [seguridad](../security/SECURITY.md) para [REQ-0002](../requirements/baseline.md). La introducción de estas reglas está reconstruida en [TASK-0002-02](../plans/TASKS-0002-registro-desarrollo-cristal.md#task-0002-02--analizar-con-lista-blanca); no era un plan previo. |
| Implementación | `src-tauri/src/safety.rs`, `safety::tests::allows_cache_child_and_rejects_documents`. |
| Preparación | Hogar sintético `/Users/test`. No crea archivos ni usa el disco de la persona. |
| Acción y resultado esperado | `is_deletable` acepta `/Users/test/Library/Caches/Google`. Rechaza `Documents/nota.txt`, `Library/Keychains/login.keychain-db`, `/Users/test` y `/Users/test/Library/Caches`. |
| Reproducción | En el host, no en Docker ([ADR-0001](../decisions/ADR-0001-app-escritorio-tauri.md)): `cargo test --lib safety::tests::allows_cache_child_and_rejects_documents -- --exact` desde `src-tauri`. |
| Estado de implementación | Implementada. |
| Evidencia de ejecución | [RUN-2026-10-15-01](TEST-RUNS.md#run-2026-10-15-01). |

## TEST-safety-002

| Campo | Contenido |
| --- | --- |
| Identificación | TEST-safety-002. Un `..` no saca la ruta de la lista blanca; un instalador suelto y viejo en forma de nombre sí entra, y un zip o un `.dmg` anidado no. Componente `safety`. Unitaria. |
| Objetivo y trazabilidad | Misma lista blanca que TEST-safety-001. Cubre el escape hacia Documentos y la regla de instaladores de [seguridad](../security/SECURITY.md). |
| Implementación | `src-tauri/src/safety.rs`, `safety::tests::rejects_parent_escape_and_allows_installer`. |
| Preparación | Hogar sintético `/Users/test`. No mira la fecha real del archivo: la función de instalador, en este caso, solo distingue extensión y carpeta. |
| Acción y resultado esperado | Rechaza `Library/Caches/../Documents/secreto`. Acepta `Downloads/Instalador.dmg`. Rechaza `Downloads/vacaciones.zip` y `Downloads/Carpeta/app.dmg`. |
| Reproducción | `cargo test --lib safety::tests::rejects_parent_escape_and_allows_installer -- --exact` desde `src-tauri`, en el host. |
| Estado de implementación | Implementada. |
| Evidencia de ejecución | [RUN-2026-10-15-01](TEST-RUNS.md#run-2026-10-15-01). |

## TEST-safety-003

| Campo | Contenido |
| --- | --- |
| Identificación | TEST-safety-003. El contenedor de Mail es dato de Apple; la caché de Safari no. Nombres de soporte protegidos. Componente `safety`. Unitaria. |
| Objetivo y trazabilidad | [ADR-0002](../decisions/ADR-0002-lista-blanca-y-papelera.md) y [seguridad](../security/SECURITY.md): `com.apple.*` fuera de cachés y registros no se mueve. |
| Implementación | `src-tauri/src/safety.rs`, `safety::tests::apple_support_is_private_but_apple_cache_is_not`. |
| Preparación | Hogar sintético `/Users/test`. Sin disco. |
| Acción y resultado esperado | `is_apple_private` es verdadero para `Library/Containers/com.apple.mail` y falso para `Library/Caches/com.apple.Safari`. `protected_support_name` acepta `MobileSync` y `com.apple.spotlight`, y rechaza `Slack`. |
| Reproducción | `cargo test --lib safety::tests::apple_support_is_private_but_apple_cache_is_not -- --exact` desde `src-tauri`, en el host. |
| Estado de implementación | Implementada. |
| Evidencia de ejecución | [RUN-2026-10-15-01](TEST-RUNS.md#run-2026-10-15-01). |

## TEST-safety-004

| Campo | Contenido |
| --- | --- |
| Identificación | TEST-safety-004. Un contenedor con un enlace hacia fuera no se manda entero a la Papelera: se planea tirar los archivos reales y quitar solo el enlace. Componente `safety`. Unitaria con directorio temporal. |
| Objetivo y trazabilidad | [PLAN-0004](../plans/PLAN-0004-contenedor-sin-seguir-enlaces.md), [TASK-0004-02](../plans/TASKS-0004-contenedor-sin-seguir-enlaces.md#task-0004-02--no-seguir-enlaces-del-contenedor) y la consecuencia de [ADR-0002](../decisions/ADR-0002-lista-blanca-y-papelera.md). |
| Implementación | `src-tauri/src/safety.rs`, `safety::tests::container_plan_trashes_real_files_and_unlinks_escaping_links`. |
| Preparación | Directorio temporal `cristal-safety-<nanos>` con `Library/Containers/com.example.app/Data/Documents/nota.txt` y un enlace `Data/Desktop` hacia `Desktop/secreto.txt`. Se borra al terminar. No toca el contenedor real de UTM. |
| Acción y resultado esperado | `disposal_plan` incluye `Trash` de `Data/Documents` y `Unlink` de `Data/Desktop`. Ningún paso apunta al Escritorio de fuera. El contenedor entero no va a la Papelera. |
| Reproducción | `cargo test --lib safety::tests::container_plan_trashes_real_files_and_unlinks_escaping_links -- --exact` desde `src-tauri`, en el host. |
| Estado de implementación | Implementada. |
| Evidencia de ejecución | [RUN-2026-10-15-01](TEST-RUNS.md#run-2026-10-15-01). |

## TEST-safety-005

| Campo | Contenido |
| --- | --- |
| Identificación | TEST-safety-005. Un archivo `com.apple…` dentro del contenedor de otra app no es dato de Apple. El contenedor o la preferencia cuyo propio nombre es de Apple sí. Componente `safety`. Unitaria. |
| Objetivo y trazabilidad | [PLAN-0006](../plans/PLAN-0006-no-bloquear-copia-dentro-del-contenedor.md), [TASK-0006-02](../plans/TASKS-0006-no-bloquear-copia-dentro-del-contenedor.md#task-0006-02--juzgar-la-app-dueña) y [seguridad](../security/SECURITY.md). |
| Implementación | `src-tauri/src/safety.rs`, `safety::tests::nested_apple_file_in_third_party_container_is_not_private`. |
| Preparación | Hogar sintético `/Users/test`. Sin disco. |
| Acción y resultado esperado | No es privado `Containers/com.utmapp.QEMUHelper/Data/Library/Preferences/com.apple.security.plist`. Sí lo son `Library/Preferences/com.apple.security.plist`, `Library/Preferences/ByHost/com.apple.finder.plist` y `Containers/com.apple.mail/Data/Library`. |
| Reproducción | `cargo test --lib safety::tests::nested_apple_file_in_third_party_container_is_not_private -- --exact` desde `src-tauri`, en el host. |
| Estado de implementación | Implementada. |
| Evidencia de ejecución | [RUN-2026-10-15-01](TEST-RUNS.md#run-2026-10-15-01). |

## TEST-safety-006

| Campo | Contenido |
| --- | --- |
| Identificación | TEST-safety-006. El plan de un contenedor no incluye el registro interno de macOS. Componente `safety`. Unitaria con directorio temporal. |
| Objetivo y trazabilidad | [PLAN-0005](../plans/PLAN-0005-omitir-metadata-de-contenedor.md), [TASK-0005-02](../plans/TASKS-0005-omitir-metadata-de-contenedor.md#task-0005-02--no-tratar-el-registro-como-fallo) y [seguridad](../security/SECURITY.md). |
| Implementación | `src-tauri/src/safety.rs`, `safety::tests::container_plan_skips_containermanager_metadata`. |
| Preparación | Directorio temporal `cristal-meta-<nanos>` con `nota.txt` y `.com.apple.containermanagerd.metadata.plist` de 30 000 bytes. Se borra al terminar. |
| Acción y resultado esperado | Ningún paso de `disposal_plan` apunta a ese plist. Algún paso tira `Data` o `Data/Documents`. El contenedor entero no va a la Papelera. |
| Reproducción | `cargo test --lib safety::tests::container_plan_skips_containermanager_metadata -- --exact` desde `src-tauri`, en el host. |
| Estado de implementación | Implementada. |
| Evidencia de ejecución | [RUN-2026-10-15-01](TEST-RUNS.md#run-2026-10-15-01). |

## TEST-apps-001

| Campo | Contenido |
| --- | --- |
| Identificación | TEST-apps-001. El inventario de ejemplo reconoce apps, binarios y prefijos de fabricante, y no marca como instalado un identificador ajeno. Componente `apps`. Unitaria. |
| Objetivo y trazabilidad | Emparejamiento usado para no tratar como resto una app que sigue instalada. Reconstruido en [TASK-0002-02](../plans/TASKS-0002-registro-desarrollo-cristal.md#task-0002-02--analizar-con-lista-blanca). El riesgo de un falso positivo queda en [seguridad](../security/SECURITY.md); esta prueba no lo cierra. |
| Implementación | `src-tauri/src/apps.rs`, `apps::tests::matches_apps_and_cli_tools`. |
| Preparación | `Inventory` en memoria: nombres `code`, `visualstudiocode`, `microsoftword`, `googlechrome`, `zoomus`; bundles `com.microsoft.word` y `com.google.chrome`; binario `codex`. No lee `/Applications`. |
| Acción y resultado esperado | `owns_exact` acepta `Code`, `zoom.us` y `codex`. `is_vendor_prefix` acepta `Microsoft` y `Google`, y rechaza `go` por ser más corto que tres caracteres. `owns_identifier` acepta `com.google.Chrome.helper` y `com.apple.mail` —este último porque todo `com.apple.` se trata como presente, no porque Mail esté en el ejemplo— y rechaza `com.abandoned.editor`. |
| Reproducción | `cargo test --lib apps::tests::matches_apps_and_cli_tools -- --exact` desde `src-tauri`, en el host. |
| Estado de implementación | Implementada. |
| Evidencia de ejecución | [RUN-2026-10-15-01](TEST-RUNS.md#run-2026-10-15-01). |

## TEST-size-001

| Campo | Contenido |
| --- | --- |
| Identificación | TEST-size-001. El tamaño recuperable no cuenta el registro interno del contenedor ni `.DS_Store`. Componente `size`. Unitaria con directorio temporal. |
| Objetivo y trazabilidad | [PLAN-0005](../plans/PLAN-0005-omitir-metadata-de-contenedor.md) y [TASK-0005-02](../plans/TASKS-0005-omitir-metadata-de-contenedor.md#task-0005-02--no-tratar-el-registro-como-fallo). [PLAN-0007](../plans/PLAN-0007-nota-del-registro-visible.md) no cambia este cálculo. |
| Implementación | `src-tauri/src/size.rs`, `size::tests::path_size_ignores_container_metadata`. |
| Preparación | Directorio temporal `cristal-size-<nanos>` con `nota.txt` de 100 bytes, el plist de containermanagerd de 30 000 bytes y `.DS_Store` de 6 148 bytes. Se borra al terminar. |
| Acción y resultado esperado | `path_size` del directorio es 100. |
| Reproducción | `cargo test --lib size::tests::path_size_ignores_container_metadata -- --exact` desde `src-tauri`, en el host. |
| Estado de implementación | Implementada. |
| Evidencia de ejecución | [RUN-2026-10-15-01](TEST-RUNS.md#run-2026-10-15-01). |

## TEST-selection-001

| Campo | Contenido |
| --- | --- |
| Identificación | TEST-selection-001. Una selección vacía no hereda el total encontrado. Componente `selection`. Unitaria, sin ventana. |
| Objetivo y trazabilidad | [REQ-0002](../requirements/baseline.md), presentado en la barra. Corregido en [PLAN-0003](../plans/PLAN-0003-actualizar-tamano-seleccion.md) y [TASK-0003-02](../plans/TASKS-0003-actualizar-tamano-seleccion.md#task-0003-02--mostrar-el-tamaño-seleccionado). |
| Implementación | `src/selection.test.js`, nombre `una selección vacía no hereda el total encontrado`. Llama a `selectionOf` y `formatBytes` de `src/selection.js`. |
| Preparación | Dos elementos sintéticos, 2 GiB y 400 MiB. Conjunto de IDs vacío. Sin DOM ni Tauri. |
| Acción y resultado esperado | `count` 0, `bytes` 0, `total` igual a la suma de ambos, y `formatBytes(0)` es `0 B`. |
| Reproducción | En la raíz, en el host: `node --test --test-name-pattern "una selección vacía no hereda el total encontrado" src/selection.test.js`. |
| Estado de implementación | Implementada. |
| Evidencia de ejecución | [RUN-2026-10-15-01](TEST-RUNS.md#run-2026-10-15-01). |

## TEST-selection-002

| Campo | Contenido |
| --- | --- |
| Identificación | TEST-selection-002. Marcar y desmarcar cambia el tamaño seleccionado. Componente `selection`. Unitaria, sin ventana. |
| Objetivo y trazabilidad | Mismo defecto de presentación que TEST-selection-001. |
| Implementación | `src/selection.test.js`, nombre `marcar y desmarcar cambia el tamaño seleccionado`. |
| Preparación | Elementos de 2 GiB y 80 MiB. Sin DOM. |
| Acción y resultado esperado | Con solo el primero, `bytes` es 2 GiB. Con ambos, es 2 GiB + 80 MiB. Los textos formateados no coinciden. |
| Reproducción | `node --test --test-name-pattern "marcar y desmarcar cambia el tamaño seleccionado" src/selection.test.js` en la raíz, en el host. |
| Estado de implementación | Implementada. |
| Evidencia de ejecución | [RUN-2026-10-15-01](TEST-RUNS.md#run-2026-10-15-01). |

## TEST-selection-003

| Campo | Contenido |
| --- | --- |
| Identificación | TEST-selection-003. Restar 80 MiB a un total en GiB cambia el texto y ambos siguen en GB. Componente `selection`. Unitaria, sin ventana. |
| Objetivo y trazabilidad | [PLAN-0003](../plans/PLAN-0003-actualizar-tamano-seleccion.md). No fija el redondeo de un ajuste de 10 MB; eso sigue fuera de esta ficha. |
| Implementación | `src/selection.test.js`, nombre `ochenta MB sobre un total en GB cambian el texto`. |
| Preparación | Ninguna. Solo `formatBytes`. |
| Acción y resultado esperado | `formatBytes(15 GiB)` y `formatBytes(15 GiB − 80 MiB)` son distintos y ambos terminan en `GB`. |
| Reproducción | `node --test --test-name-pattern "ochenta MB sobre un total en GB cambian el texto" src/selection.test.js` en la raíz, en el host. |
| Estado de implementación | Implementada. |
| Evidencia de ejecución | [RUN-2026-10-15-01](TEST-RUNS.md#run-2026-10-15-01). |

## TEST-place-001

| Campo | Contenido |
| --- | --- |
| Identificación | TEST-place-001. Al marcar en la misma categoría, la grilla, la barra y el foco se quedan. Componente `place`. Unitaria, sin ventana. |
| Objetivo y trazabilidad | [PLAN-0010](../plans/PLAN-0010-grilla-sin-saltar.md), [TASK-0010-02](../plans/TASKS-0010-grilla-sin-saltar.md#task-0010-02--conservar-el-desplazamiento). No cambia [REQ-0002](../requirements/baseline.md): solo evita que el redibujo suba la lista. |
| Implementación | `src/place.test.js`, nombre `la misma categoría conserva la grilla y el foco`. Llama a `placeToKeep` de `src/place.js`. |
| Preparación | Categoría `caches`, lista en 480, barra en 120, foco en `caches-4`. Sin DOM ni Tauri. |
| Acción y resultado esperado | Si la pantalla sigue en resultados y la categoría no cambia, se conservan 480, 120 y `caches-4`. |
| Reproducción | En la raíz, en el host: `node --test --test-name-pattern "la misma categoría conserva la grilla y el foco" src/place.test.js`. |
| Estado de implementación | Implementada. |
| Evidencia de ejecución | [RUN-2026-10-15-02](TEST-RUNS.md#run-2026-10-15-02). |

## TEST-place-002

| Campo | Contenido |
| --- | --- |
| Identificación | TEST-place-002. Abrir otra categoría no hereda el desplazamiento de la lista anterior. La barra sí se queda. Componente `place`. Unitaria, sin ventana. |
| Objetivo y trazabilidad | Mismo defecto de [PLAN-0010](../plans/PLAN-0010-grilla-sin-saltar.md). |
| Implementación | `src/place.test.js`, nombre `otra categoría no hereda el desplazamiento de la lista`. |
| Preparación | El mismo punto de partida que TEST-place-001. La categoría siguiente es `logs`. |
| Acción y resultado esperado | `list` es nulo y no hay foco que devolver. `sidebar` sigue en 120. |
| Reproducción | `node --test --test-name-pattern "otra categoría no hereda el desplazamiento de la lista" src/place.test.js` en la raíz, en el host. |
| Estado de implementación | Implementada. |
| Evidencia de ejecución | [RUN-2026-10-15-02](TEST-RUNS.md#run-2026-10-15-02). |

## TEST-place-003

| Campo | Contenido |
| --- | --- |
| Identificación | TEST-place-003. Salir de resultados no reutiliza el sitio. Abrir la confirmación conserva la lista, pero no devuelve el foco a la casilla. Componente `place`. Unitaria, sin ventana. |
| Objetivo y trazabilidad | [PLAN-0010](../plans/PLAN-0010-grilla-sin-saltar.md). La hoja no debe perder sus controles. |
| Implementación | `src/place.test.js`, nombre `salir de resultados o abrir la confirmación no reutiliza el sitio`. |
| Preparación | El mismo punto de partida. Variantes: pantalla `cleaning`, y resultados con `confirm`. |
| Acción y resultado esperado | Al limpiar, lista, barra y foco quedan vacíos. Con la hoja abierta, la lista sigue en 480 y el foco no vuelve a la casilla. |
| Reproducción | `node --test --test-name-pattern "salir de resultados o abrir la confirmación no reutiliza el sitio" src/place.test.js` en la raíz, en el host. |
| Estado de implementación | Implementada. |
| Evidencia de ejecución | [RUN-2026-10-15-02](TEST-RUNS.md#run-2026-10-15-02). |

## Sin ficha

No se asigna ID a lo que no tiene prueba ni procedimiento escrito:

- Botón Salir y su posición. [RD-0001](../decisions/RD-0001-salir-readme-github.md) y [RD-0002](../decisions/RD-0002-salir-abajo-a-la-derecha.md) lo dejan como comprobación visual.
- Movimiento real a la Papelera y restauración.
- Análisis del disco de la persona.
- Contraste, tema claro y oscuro, y `prefers-reduced-transparency`.
- `cargo check`. Es compilación, no un caso de comportamiento.
