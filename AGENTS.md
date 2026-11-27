# Guía del repositorio

## Estructura y organización

WTZ es un lenguaje educativo basado en pila, con compilador y máquina virtual previstos en Rust. Actualmente el repositorio contiene únicamente documentación de diseño:

- `README.md`: presentación, hoja de ruta y estructura propuesta.
- `docs/Arquitectura.md`: responsabilidades, formato de bytecode y decisiones pendientes.

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

El historial usa resúmenes breves en inglés, como `Update README.md`, sin prefijos establecidos. Para nuevas contribuciones, usar ramas por tarea y commits concretos; por ejemplo, `docs/guia-colaboracion` y `docs: traducir guia de contribucion`. Describir en cada PR el cambio, las decisiones relevantes, las incidencias relacionadas y la validación realizada. Indicar comprobaciones pendientes.

## Arquitectura

Consultar `docs/Arquitectura.md` antes de modificar el lenguaje o bytecode. Documentar opcodes, tamaños de operandos, orden de bytes y errores. Separar compilación y ejecución; el diseño inicial no contempla una representación intermedia independiente.
