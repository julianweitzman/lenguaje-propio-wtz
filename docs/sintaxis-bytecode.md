# Sintaxis textual del bytecode

WTZ utilizará instrucciones intermedias basadas en pila que se codificarán en
un archivo binario para la VM. Este documento muestra cómo se verán esas
instrucciones en texto para documentarlas, inspeccionarlas y, en el futuro,
desensamblarlas. **La VM ejecutará el binario, no este texto.**

El diseño es una propuesta y todavía no está implementado. No se exige generar
un archivo de texto como paso de compilación ni se define aún un ensamblador.
El lenguaje que escribirá el usuario se describe en
[sintaxis-codigo-fuente.md](sintaxis-codigo-fuente.md).

## Forma textual

Se mostrará una instrucción por línea, en mayúsculas, seguida de sus operandos
separados por espacios. Las etiquetas simbólicas terminarán en `:`. Esta
notación no fija tamaños binarios ni posiciones en bytes.

```asm
PUSH_I32 10
PUSH_I32 20
ADD_I32
PRINT_I32
HALT
```

Representa la compilación de `print(10 + 20);` y mostraría `30`.

## Tipos y pila

Se proponen instrucciones tipadas para enteros `i32`, booleanos y referencias
a strings. El compilador comprobará los tipos del código fuente; la validación
del bytecode y las protecciones de la VM deberán rechazar estados incompatibles.
La disposición física de los valores en la pila queda pendiente.

En la notación `( a b -- resultado )`, la cima está a la derecha. Una operación
binaria retira primero `b` y luego `a`, y calcula `a operación b`.

| Instrucción propuesta | Efecto en la pila | Comportamiento |
| --- | --- | --- |
| `PUSH_I32 n` | `( -- n )` | Apila un entero con signo de 32 bits. |
| `PUSH_BOOL valor` | `( -- valor )` | Apila `true` o `false`. |
| `POP` | `( a -- )` | Descarta la cima. |
| `DUP` | `( a -- a a )` | Duplica el valor; para strings copia la referencia. |
| `SWAP` | `( a b -- b a )` | Intercambia los dos valores superiores. |
| `ADD_I32` | `( a b -- suma )` | Suma dos enteros `i32`. |
| `SUB_I32` | `( a b -- resta )` | Calcula `a - b`. |
| `MUL_I32` | `( a b -- producto )` | Multiplica dos enteros `i32`. |
| `PUSH_CONST indice` | `( -- referencia )` | Apila una referencia a un string constante. |
| `PRINT_I32` | `( n -- )` | Muestra y consume un entero. |
| `PRINT_STRING` | `( referencia -- )` | Muestra y consume una referencia a texto. |
| `HALT` | Sin cambio | Finaliza la ejecución. |

Estas propuestas sustituyen los ejemplos genéricos anteriores de `PUSH`, `ADD`
y `PRINT`. No hay asignación definitiva de opcodes. La propuesta de impresión
consume su argumento; el formato exacto de salida sigue pendiente.

## Strings y constantes

Los literales de texto se guardarían en una tabla de constantes del binario.
La siguiente es una vista textual ilustrativa de esa tabla y del código:

```text
CONSTANTS
0: string "Ana"

CODE
PUSH_CONST 0
PRINT_STRING
HALT
```

Corresponde a `print("Ana");`. `CONSTANTS` y `CODE` son rótulos de la vista,
no instrucciones ejecutables. El índice `0` identifica una entrada de la tabla;
no es una dirección de memoria. El string permanece en la tabla aunque se
retire su referencia de la pila. Codificación del texto, longitudes, tamaño de
referencias y creación de strings dinámicos quedan pendientes.

## Saltos y etiquetas

Se prevén `JMP etiqueta`, `JZ etiqueta` y `JNZ etiqueta`. Las etiquetas serán
nombres legibles para destinos de instrucciones; en el binario se codificarán
como direcciones o desplazamientos, según el formato que se defina.

Se propone que `JZ` y `JNZ` consuman un `i32` y prueben si es cero o distinto
de cero. La traducción de condiciones `bool` a saltos sigue pendiente.

```asm
PUSH_I32 3
bucle:
DUP
PRINT_I32
PUSH_I32 1
SUB_I32
DUP
JNZ bucle
POP
HALT
```

Mostraría `3`, `2` y `1`. `DUP` conserva el contador para las operaciones que
consumen valores; al terminar, `POP` descarta el cero restante.

## Instrucciones adicionales pendientes

- `CALL etiqueta` y `RET`: llamadas, direcciones de retorno y contratos de pila.
- `LOAD` y `STORE`: posiciones de memoria, tipos y alcance del almacenamiento.
- `READ`: tipo de entrada, formato y tratamiento de errores.
- División, módulo, comparaciones y operaciones booleanas tipadas.

## Codificación y errores

Antes de implementar se definirán los opcodes, tamaños de operandos, orden de
bytes, cabecera, versión y extensión del archivo binario. El inmediato de
`PUSH_I32` representará un entero de 32 bits; su orden de bytes sigue pendiente.
Las etiquetas y los rótulos de esta vista no se almacenarán como instrucciones.

Se deberán detectar opcodes desconocidos, operandos truncados, índices de
constantes inválidos, destinos fuera de instrucciones, pila insuficiente y tipos
incompatibles. Los caminos que confluyan deberán respetar un contrato de pila
compatible. Desbordamientos y errores aritméticos requieren reglas adicionales.

Consultar la [arquitectura](arquitectura.md) para el flujo de compilación y ejecución.
