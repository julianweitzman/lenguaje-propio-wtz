# Guía del repositorio

## Estructura y organización

WTZ es un lenguaje educativo con compilador y máquina virtual basada en pila previstos en Rust. Actualmente el repositorio contiene únicamente documentación de diseño:

- `README.md`: presentación breve, estado y enlaces a documentación.
- `docs/arquitectura.md`: responsabilidades, formato de bytecode y decisiones pendientes.
- `docs/sintaxis-codigo-fuente.md`: propuesta del lenguaje fuente y sus tipos.
- `docs/sintaxis-bytecode.md`: vista textual de las instrucciones que se codificarán en binario.
- `docs/decisiones-pendientes.md`: inventario histórico de alternativas y detalles que siguen pendientes.
- `docs/formulario-decisiones.html`: formulario autónomo para elegir y exportar decisiones.
- `docs/respuestas-diseno.json`: respuestas recibidas y aclaraciones posteriores; consultar la sintaxis para las reglas vigentes.

Todavía no existen `Cargo.toml`, código, pruebas ni recursos gráficos. Al implementar, seguir la separación propuesta: `src/compiler/` para análisis y generación de código, `src/bytecode/` para opcodes y codificación binaria, y `src/vm/` para ejecución. Ubicar pruebas de integración en `tests/` y programas `.wtz` de ejemplo en `examples/`.

## Compilación y desarrollo

Actualmente no hay comandos de compilación o ejecución disponibles. Cuando exista el paquete Cargo, ejecutar desde su raíz:

- `cargo build`: compilar el proyecto.
- `cargo test`: ejecutar pruebas unitarias y de integración.
- `cargo run -- <argumentos>`: ejecutar el binario con los argumentos documentados.
- `cargo fmt --check`: comprobar el formato de Rust.
- `cargo clippy --all-targets -- -D warnings`: detectar advertencias de análisis estático.

Estos comandos son orientativos; todavía no hay configuración de herramientas ni integración continua versionada.

## Estilo y nombres

Para código Rust nuevo, usar cuatro espacios de sangría y formato estándar de `rustfmt`. Emplear `snake_case` para módulos y funciones, `PascalCase` para tipos y `SCREAMING_SNAKE_CASE` para constantes. Escribir instrucciones WTZ en mayúsculas: `PUSH`, `ADD`, `HALT`. Mantener la documentación en español y distinguir propuestas de comportamientos implementados.

## Pruebas

Usar el sistema de pruebas integrado de Rust cuando exista código. Colocar pruebas unitarias junto al módulo correspondiente y pruebas de integración en `tests/`. Nombrarlas por comportamiento observable, como `rejects_truncated_bytecode`. Cubrir validación del compilador, concordancia entre codificación y decodificación, pila insuficiente, opcodes inválidos y ejecución completa conforme se implementen. No hay umbral de cobertura definido.

## Commits y pull requests

Los agentes deben usar la habilidad `git` para gestionar ramas, commits y publicaciones, siguiendo sus instrucciones y conservando el trabajo existente.

Usar ramas por tarea y commits concretos; por ejemplo, `docs/guia-colaboracion` y `docs: traducir guia de contribucion`. Describir en cada PR el cambio, las decisiones relevantes, las incidencias relacionadas y la validación realizada. Indicar comprobaciones pendientes.

## Arquitectura

Consultar `docs/arquitectura.md` antes de modificar el lenguaje o bytecode. Documentar opcodes, tamaños de operandos, orden de bytes y errores. Separar compilación y ejecución; el diseño contempla instrucciones intermedias de pila con una vista textual y codificación binaria.

## Flujo acordado con el usuario

Registrar en la documentación las decisiones de diseño que indique el usuario.
Después de cada actualización, verificar los cambios, crear un commit y hacer
push de la rama de trabajo. Preferencia indicada el 2026-11-27.
