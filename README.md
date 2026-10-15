# Cristal

Limpieza clara para tu Mac. Cristal analiza la carpeta de tu usuario, te muestra qué se puede liberar y solo mueve a la Papelera lo que confirmas.

La ventana usa Liquid Glass en macOS 26 o posterior.

## Qué limpia

| Categoría | Qué incluye | Marcada por defecto |
| --- | --- | --- |
| Navegadores | Caché y archivos temporales de Chrome, Safari, Firefox, Edge, Brave y otros. Opcionalmente, cookies. | Solo la caché grande |
| Cachés de apps | `~/Library/Caches`. Las apps pueden volver a crearlos. | Si pesan más de 512 KB |
| Restos de apps | Carpetas, preferencias y agentes de apps que ya no están instaladas. | No |
| Registros | Logs de `~/Library/Logs`. | Sí, excepto informes de fallos |
| Desarrollo | Xcode, Homebrew, npm, pip, Cargo y similares. | No |
| Instaladores | `.dmg` y `.pkg` en Descargas con más de 30 días. | No |
| Papelera | Vaciar lo que ya tiraste. Esta acción es permanente. | No |

No toca Documentos, Fotos, el Escritorio, el Llavero, contraseñas, marcadores ni el historial. Tampoco borra datos de Apple (`com.apple.*`) fuera de cachés y registros. Un archivo `com.apple…` que vive dentro del contenedor de otra app sí se puede mover: es una copia de esa app, no Mail ni las preferencias del sistema.

Los restos de una app no se mandan enteros a Finder. Si el contenedor tiene enlaces al Escritorio, a Descargas o al Llavero, se mueven los archivos reales y solo se quita el enlace. macOS puede dejar un registro interno (`.com.apple.containermanagerd.metadata.plist`); no se cuenta como fallo ni como espacio recuperable.

Los elementos marcados como **Revisar** pueden incluir documentos. Ábrelos con **Ver** antes de limpiarlos.

## Cómo usarla

1. Pulsa **Analizar esta Mac**. Todavía no se borra nada.
2. Revisa la selección. Lo seguro ya viene marcado. El tamaño cambia al marcar o desmarcar; si no queda nada, muestra 0 B. La barra dice cuánto hay seleccionado de lo encontrado, por ejemplo `4.2 GB de 6.1 GB`.
3. Pulsa **Revisar y limpiar** y confirma.

**Salir** cierra la app. Está siempre abajo a la derecha, a la derecha de **Analizar**.

## Requisitos

- macOS
- [Rust](https://rustup.rs)
- Node.js 18 o posterior

Liquid Glass se aplica en macOS 26+. En versiones anteriores, la ventana usa el efecto de vibrancy.

## Ejecutar en desarrollo

```bash
npm install
npm test
npm run tauri dev
```

`npm test` comprueba el tamaño de la selección, sin abrir la ventana. Las reglas de rutas se prueban con `cargo test --lib` dentro de `src-tauri`.

La primera compilación tarda un poco: Tauri descarga y compila sus dependencias.

## Generar la app

```bash
npm run tauri build
```

El `.app` queda en `src-tauri/target/release/bundle/macos/`.

## Documentación

El método de trabajo sigue [Arquitectura_agent](https://github.com/zknchile/Arquitectura_agent). En este repositorio:

- [AGENTS.md](AGENTS.md) — por dónde empezar.
- [Requisitos](docs/requirements/baseline.md), [decisiones](docs/decisions/README.md) y [evolución](CHANGELOG.md).
- [Planes](docs/plans/PLAN-0001-documentar-desarrollo.md). El desarrollo inicial está reconstruido en [PLAN-0002](docs/plans/PLAN-0002-registro-desarrollo-cristal.md); no se escribió antes de programar. El tamaño de la selección se corrigió en [PLAN-0003](docs/plans/PLAN-0003-actualizar-tamano-seleccion.md). Los contenedores con enlaces, el registro interno de macOS y las copias `com.apple` dentro de otra app quedaron en [PLAN-0004](docs/plans/PLAN-0004-contenedor-sin-seguir-enlaces.md), [PLAN-0005](docs/plans/PLAN-0005-omitir-metadata-de-contenedor.md), [PLAN-0006](docs/plans/PLAN-0006-no-bloquear-copia-dentro-del-contenedor.md) y [PLAN-0007](docs/plans/PLAN-0007-nota-del-registro-visible.md). Salir quedó abajo a la derecha según [PLAN-0008](docs/plans/PLAN-0008-salir-junto-a-analizar.md).

## Cómo está armada

- `src-tauri/`: lógica en Rust. Analiza, calcula tamaños y mueve a la Papelera.
- `src/`: interfaz. Primero resume, después deja revisar cada categoría.
- `safety.rs`: lista de rutas permitidas. Aunque la interfaz pidiera otra cosa, Rust rechaza lo que está fuera.

La limpieza y el análisis usan las mismas reglas. Un archivo solo se puede mover si estaba en el último análisis y vive en una carpeta permitida.
