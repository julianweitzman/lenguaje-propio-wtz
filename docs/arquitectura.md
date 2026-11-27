# Arquitectura de WTZ

## 1. Propósito y estado

WTZ es un proyecto educativo con un lenguaje fuente de mayor nivel, un lenguaje
intermedio de instrucciones de pila, bytecode binario propio y una máquina
virtual. La implementación se prevé en Rust; actualmente solo existe documentación.

La separación entre código fuente y bytecode busca mejorar la legibilidad de los
programas y comprobar sus tipos antes de ejecutarlos. Reemplaza el diseño inicial
en el que el código fuente era una lista de instrucciones similar a Assembly.

## 2. Flujo general

```text
Código fuente (.wtz)
        ↓
Lexer y parser
        ↓
Validación semántica y de tipos
        ↓
Instrucciones intermedias de pila
        ↓
Codificador de bytecode
        ↓
Archivo binario
        ↓
Cargador y validador
        ↓
Máquina virtual → Ejecución
```

Las instrucciones intermedias tendrán una vista textual legible. Esa vista no
será un archivo obligatorio entre etapas: el compilador podrá construir las
instrucciones en memoria y codificarlas directamente. No se propone otra IR
adicional a estas instrucciones de pila.

## 3. Compilador

- **Lexer:** reconoce palabras reservadas, identificadores, literales, operadores
  y delimitadores conforme se cierre la sintaxis fuente.
- **Parser:** construye una representación de declaraciones y expresiones.
- **Validación semántica:** resuelve nombres y alcances y comprueba tipos,
  incluyendo compatibilidad de asignaciones y operaciones.
- **Generación de instrucciones:** transforma expresiones en operaciones de pila,
  selecciona instrucciones tipadas y reúne las constantes de texto.
- **Codificación:** convierte las instrucciones a opcodes y operandos binarios,
  y resuelve los destinos simbólicos según el formato acordado.

El detalle del lexer y el parser se documentará cuando se diseñen esas etapas.
Las reglas del lenguaje se mantienen en
[sintaxis-codigo-fuente.md](sintaxis-codigo-fuente.md).

## 4. Lenguaje intermedio y bytecode

Las operaciones intermedias se describen en
[sintaxis-bytecode.md](sintaxis-bytecode.md), mediante una representación textual
similar a Assembly. La VM consumirá su codificación binaria.

El generador producirá instrucciones y referencias a constantes. El codificador
será responsable de convertirlas al formato binario y resolver los destinos de
control de flujo. La representación concreta de valores y almacenamiento sigue
pendiente de diseño.

Los números de opcode, endianness, tamaños de índices y destinos, codificación
de strings, cabecera, versión y extensión del binario no están definidos. No
se asignan valores numéricos definitivos en esta etapa.

## 5. Máquina virtual

La VM se organizará alrededor de una pila de datos, un contador de programa,
el código binario cargado, una tabla de constantes y el estado de ejecución.
El almacenamiento de variables y la pila de llamadas quedan pendientes de diseño.

El ciclo de ejecución obtendrá una instrucción, decodificará sus operandos,
realizará la operación y actualizará el contador de programa hasta la finalización del programa o un
error.

El cargador y la VM deberán protegerse frente a:

- Archivos mal formados, versiones no admitidas y bytecode truncado.
- Opcodes desconocidos y operandos inválidos.
- Referencias a constantes o destinos de salto inválidos.
- Pila insuficiente y valores incompatibles con la instrucción.
- Accesos inválidos a memoria y retornos sin una llamada válida.
- Errores de entrada y aritméticos según las reglas que se definan.

La comprobación estática del código fuente no sustituye la validación del
binario. Los caminos de control que confluyan deberán tener estados de pila
compatibles; queda pendiente cómo se verificará ese contrato.

## 6. Organización prevista

- `src/compiler/`: lexer, parser, validación semántica y generación de instrucciones.
- `src/bytecode/`: definición de instrucciones, codificación y decodificación.
- `src/vm/`: pila, ejecución y almacenamiento.
- `tests/`: pruebas de integración cuando exista implementación.
- `examples/`: programas fuente `.wtz`.

Esta estructura es una propuesta; todavía no existen esos módulos.

## 7. Decisiones pendientes

- Diseño detallado del lexer, parser y validación semántica.
- Traducción del lenguaje fuente a instrucciones intermedias.
- Representación de valores, booleanos, strings y referencias en la VM.
- Contratos de llamadas, memoria, entrada y salida.
- Formato binario completo y validación de tipos y flujo de control.
- Strings dinámicos y administración de su memoria.
- Convenciones y mensajes de errores de compilación y ejecución.

## 8. Principios

Mantener separadas la compilación y la ejecución, definir un contrato explícito
entre codificador y VM y evitar depender de la representación interna de Rust.
La implementación será incremental y priorizará comprender cada componente sin
delegar sus responsabilidades centrales en bibliotecas externas.
