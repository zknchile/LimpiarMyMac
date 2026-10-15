# Seguridad de Cristal

Cristal corre en la sesión de la persona usuaria y puede mover archivos de su carpeta personal. No tiene cuenta, red propia ni secretos de aplicación.

## Datos

- Lee nombres y tamaños de cachés, restos de apps, registros e instaladores. No lee el contenido de documentos para mostrarlo.
- El informe de análisis permanece en memoria. No se escribe un historial de rutas en disco.
- No se versionan `.env`, llaveros ni credenciales. `.cargo` está fuera de la lista blanca, excepto `~/.cargo/registry/cache`.

## Controles en código

La fuente vigente es `src-tauri/src/safety.rs`, decidida en [ADR-0002](../decisions/ADR-0002-lista-blanca-y-papelera.md).

- Lista blanca de carpetas. La carpeta de inicio, Documentos, Fotos, Escritorio, Llavero y datos de Apple fuera de cachés y registros no se pueden mover.
- La limpieza solo acepta rutas que estaban en el último informe.
- Los enlaces no se siguen si el destino sale de una carpeta permitida. Un contenedor de app no se entrega entero a Finder: se mueven los archivos reales y se quita el enlace, no el Escritorio, las Descargas ni el Llavero a los que apunta.
- El registro `.com.apple.containermanagerd.*.plist` no se mueve ni se cuenta como espacio recuperable. macOS no deja borrarlo; dejarlo no es un fallo de la limpieza.
- Un archivo `com.apple…` dentro del contenedor de otra app es una copia de esa app, no datos de Apple. Siguen prohibidos el contenedor, el grupo o la preferencia cuyo propio nombre es de Apple.
- El destino normal es la Papelera. Vaciarla es explícito y lo hace Finder.
- En Descargas solo se proponen `.dmg` y `.pkg` con más de 30 días, y se vuelve a comprobar la edad al moverlos.

## Riesgo que sigue abierto

El detector de restos compara nombres con apps instaladas. Puede proponer una carpeta de documentos de una herramienta que no tenga `.app`. Esas filas no vienen marcadas y dicen que hay que revisarlas. No es una garantía de que la persona las lea.

Ampliar la lista blanca o marcar restos por defecto exige actualizar este documento, las pruebas de `safety.rs` y su ficha en [casos](../testing/TEST-CASES.md) en el mismo cambio.
