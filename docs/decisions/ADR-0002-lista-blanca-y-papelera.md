# ADR-0002 — Limpieza con lista blanca y Papelera

- **Status:** Accepted
- **Date:** 2026-10-15

## Context

[REQ-0002](../requirements/baseline.md) pide analizar, mostrar qué se puede borrar y solo entonces ejecutar. El pedido nombra datos de navegadores y restos de apps desinstaladas. No decía si el borrado debía ser permanente.

El análisis heurístico puede marcar carpetas que todavía contienen documentos, sobre todo si la app no está en `/Applications` pero sus datos siguen siendo útiles. Borrar eso de forma irreversible no se puede justificar con el pedido.

## Decision

- El análisis y la limpieza usan la misma lista blanca de `src-tauri/src/safety.rs`. Una ruta que no pasa esa lista no se propone y no se mueve.
- Solo se puede limpiar un elemento que esté en el último informe en memoria.
- Lo habitual es mover a la Papelera con la crate `trash`, no `rm`.
- Cachés y registros grandes pueden venir marcados. Restos de apps, cookies, instaladores y la Papelera no.
- Vaciar la Papelera es otra acción, desmarcada, y llama a Finder. Finder puede pedir su propia confirmación.
- No se siguen enlaces que salgan de una carpeta permitida. No se tocan Documentos, Fotos, Escritorio, Llavero, `com.apple.*` fuera de cachés y registros, ni perfiles de navegador completos.

## Alternatives Considered

- Borrar en el acto los archivos detectados. Cumple “limpiar”, pero contradice el pedido de decidir después del análisis y no permite recuperar un falso positivo.
- Mover todo lo detectado a la Papelera sin casillas. Reduce pasos, pero no deja revisar restos de apps. Se rechazó: la confirmación muestra categorías y los elementos más grandes.
- Incluir el perfil completo del navegador. Liberaría más espacio, pero arrastra contraseñas, marcadores e historial, que el pedido no autorizó.

## Consequences

- Un falso positivo de “app desinstalada” puede seguir apareciendo. Queda en Revisar y sin marcar. La persona puede usar Ver antes de aceptarlo.
- El informe vive en memoria. Cerrar la app obliga a analizar de nuevo antes de limpiar.
- Vaciar la Papelera no se puede deshacer. La interfaz lo dice y usa otro botón si esa opción está incluida.
- Desde el 2026-10-15, una carpeta con enlaces hacia fuera no se entrega entera a Finder. El caso comprobado es `~/Library/Containers/com.utmapp.UTM`: Finder lo rechazaba por permisos y adentro hay enlaces al Escritorio y al Llavero. Se mueven los archivos reales y se quita solo el enlace.
- Ese mismo contenedor conserva `.com.apple.containermanagerd.metadata.plist`. Ni Finder ni `rm` pueden quitarlo. No se cuenta como fallo ni como espacio recuperable.
- Un hijo llamado `com.apple…` dentro de una app de terceros no es dato de Apple. La prohibición mira el nombre del contenedor o de la preferencia, no cada archivo interno.

## Related Requirements

[REQ-0001](../requirements/baseline.md), [REQ-0002](../requirements/baseline.md).

## Related ADRs

[ADR-0001](ADR-0001-app-escritorio-tauri.md).
