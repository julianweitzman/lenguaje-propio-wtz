# Arquitectura de WTZ

## 1. Propósito

Este documento describe la arquitectura inicial de WTZ: un lenguaje de
programación de sintaxis similar a Assembly, un compilador que genera
bytecode propio y una máquina virtual basada en pila que ejecuta ese
bytecode.

La arquitectura es inicial y podrá modificarse cuando las decisiones de
diseño se concreten durante la implementación.

## 2. Visión general

WTZ tendrá tres componentes principales:

1.  **Compilador:** transforma el código fuente `.wtz` en bytecode.
2.  **Bytecode:** define la representación binaria de las instrucciones.
3.  **Máquina virtual:** carga, decodifica y ejecuta el bytecode.

Flujo general:

``` text
┌──────────────────────┐
│ Código fuente (.wtz) │
└──────────┬───────────┘
           v
┌──────────────────────┐
│         Lexer        │
└──────────┬───────────┘
           v
┌──────────────────────┐
│        Parser        │
└──────────┬───────────┘
           v
┌──────────────────────┐
│ Validación semántica │
└──────────┬───────────┘
           v
┌──────────────────────┐
│ Generador de bytecode│
└──────────┬───────────┘
           v
┌──────────────────────┐
│       Bytecode       │
└──────────┬───────────┘
           v
┌──────────────────────┐
│ Máquina virtual (VM) │
└──────────┬───────────┘
           v
       Ejecución
```

No se contempla una IR independiente en la primera versión. El
compilador analizará el código y generará bytecode directamente. Para
organizar el proceso, podrá utilizar estructuras internas de Rust que
representen instrucciones y operandos; estas estructuras serán una
representación de implementación, no una etapa intermedia independiente
del diseño del lenguaje.

## 3. Lenguaje fuente

Los programas se escribirán en archivos con extensión **`.wtz`**.

La sintaxis estará inspirada en Assembly: cada línea contendrá, en
general, una instrucción y sus operandos.

Ejemplo:

``` asm
PUSH 10
PUSH 20
ADD
PRINT
HALT
```

El lenguaje comenzará con un conjunto reducido de instrucciones. Su
sintaxis, reglas, operandos y comportamiento deberán definirse antes de
cerrar el compilador.

### 3.1 Instrucciones previstas

La siguiente lista es una propuesta inicial, no una especificación
definitiva.

  Categoría          Instrucciones
  ------------------ -----------------------------------
  Pila               `PUSH`, `POP`, `DUP`, `SWAP`
  Aritmética         `ADD`, `SUB`, `MUL`, `DIV`, `MOD`
  Control de flujo   `JMP`, `JZ`, `JNZ`
  Funciones          `CALL`, `RET`
  Memoria            `LOAD`, `STORE`
  Entrada y salida   `PRINT`, `READ`
  Ejecución          `HALT`

## 4. Compilador

El compilador será responsable de transformar un archivo `.wtz` en
bytecode.

### 4.1 Lexer

El lexer recorrerá los caracteres del archivo fuente y producirá tokens.

Deberá reconocer, entre otros elementos:

-   Nombres de instrucciones.
-   Literales numéricos.
-   Identificadores y etiquetas.
-   Separadores y saltos de línea.
-   Comentarios, si se incorporan a la sintaxis.

El lexer no determinará si una instrucción es válida semánticamente; su
responsabilidad será reconocer los elementos léxicos.

### 4.2 Parser

El parser recibirá los tokens y comprobará que respeten la gramática del
lenguaje.

Construirá una representación estructurada de las instrucciones y sus
operandos. Por ejemplo, una instrucción `PUSH 10` podrá representarse
internamente como una variante equivalente a `Push(10)`.

### 4.3 Validación semántica

Esta etapa comprobará las reglas que no pueden determinarse únicamente
mediante la gramática.

Entre otras cosas, podrá verificar:

-   Que las instrucciones existan.
-   Que cada instrucción reciba la cantidad y el tipo de operandos
    esperados.
-   Que las etiquetas utilizadas estén definidas.
-   Que los destinos de los saltos sean válidos.

Las comprobaciones que dependan del estado de ejecución, como intentar
extraer un elemento de una pila vacía, también deberán estar protegidas
por la máquina virtual.

### 4.4 Generación de bytecode

El generador traducirá las instrucciones analizadas a su codificación
binaria.

Cada instrucción tendrá un opcode definido por WTZ y, cuando
corresponda, uno o más operandos codificados según el formato
establecido.

La resolución de etiquetas deberá convertir los destinos simbólicos de
los saltos en posiciones concretas del programa compilado.

## 5. Bytecode

El bytecode será el conjunto de instrucciones binarias que ejecutará la
VM. Su diseño será independiente de la sintaxis textual de WTZ y de los
tipos internos utilizados en Rust.

### 5.1 Opcodes

Cada operación tendrá un código numérico propio (*opcode*).

Ejemplo ilustrativo:

  Opcode   Operación
  -------- -----------
  `0x01`   `PUSH`
  `0x02`   `ADD`
  `0x03`   `SUB`
  `0x04`   `PRINT`
  `0xFF`   `HALT`

Estos valores son ejemplos para explicar el concepto; todavía no
constituyen una asignación definitiva.

### 5.2 Codificación de operandos

Las instrucciones podrán tener distintos formatos:

-   Instrucciones sin operandos, como `ADD`.
-   Instrucciones con un valor inmediato, como `PUSH 10`.
-   Instrucciones con una dirección o desplazamiento, como los saltos.

Antes de implementar el formato binario habrá que definir el tamaño de
los operandos, el orden de los bytes (*endianness*) y cómo se codifican
las direcciones.

### 5.3 Archivo de bytecode

El bytecode podrá almacenarse en un archivo independiente del código
fuente. Su extensión, cabecera, versión y organización interna quedan
pendientes de definición.

El cargador de la VM deberá comprobar que el archivo tenga una
estructura válida antes de ejecutarlo.

## 6. Máquina virtual

La VM será una máquina abstracta implementada en Rust. Ejecutará las
instrucciones del bytecode sin depender directamente de la arquitectura
física del procesador.

Se utilizará un modelo basado en pila (*stack-based*): las operaciones
tomarán sus operandos de la pila y depositarán allí sus resultados.

### 6.1 Componentes principales

-   **Stack:** almacena los valores sobre los que operan las
    instrucciones.
-   **Program Counter (PC):** identifica la posición de la siguiente
    instrucción que debe ejecutarse.
-   **Code:** contiene el bytecode cargado.
-   **Memory:** proporcionará almacenamiento direccionable cuando el
    lenguaje incorpore instrucciones de memoria.
-   **Execution state:** representa si la máquina está ejecutándose,
    finalizó o encontró un error.

La organización exacta de estos componentes se definirá durante la
implementación.

### 6.2 Ciclo de ejecución

La VM seguirá un ciclo equivalente a *fetch-decode-execute*:

1.  **Fetch:** obtiene la instrucción ubicada en la posición indicada
    por el PC.
2.  **Decode:** interpreta el opcode y extrae sus operandos.
3.  **Execute:** realiza la operación correspondiente y actualiza el
    estado de la máquina.
4.  Repite el ciclo hasta ejecutar `HALT` o producirse un error.

### 6.3 Errores de ejecución

La VM deberá detectar situaciones inválidas, entre ellas:

-   Opcode desconocido.
-   Bytecode truncado o mal formado.
-   Acceso fuera de los límites del programa.
-   Desbordamientos o divisiones inválidas, según las reglas aritméticas
    que se definan.
-   Operaciones sobre una pila sin suficientes elementos.
-   Accesos inválidos a memoria.

Los errores deberán informarse de forma controlada, sin provocar
comportamientos indefinidos.

## 7. Organización inicial del código

Una posible organización del proyecto en Rust:

``` text
WTZ/
├── docs/
│   └── Arquitectura.md
├── src/
│   ├── compiler/
│   │   ├── lexer.rs
│   │   ├── parser.rs
│   │   ├── semantic.rs
│   │   └── codegen.rs
│   ├── bytecode/
│   │   ├── opcode.rs
│   │   ├── encoder.rs
│   │   └── decoder.rs
│   ├── vm/
│   │   ├── vm.rs
│   │   ├── stack.rs
│   │   └── memory.rs
│   └── main.rs
├── tests/
├── examples/
├── Cargo.toml
└── README.md
```

Esta estructura es orientativa. Los módulos podrán reorganizarse cuando
se conozcan mejor sus responsabilidades y dependencias.

## 8. Decisiones pendientes

Antes de considerar estable la especificación, habrá que decidir:

-   Tipo y tamaño de los valores de la pila.
-   Representación de enteros y comportamiento ante desbordamientos.
-   Formato exacto de cada instrucción.
-   Asignación definitiva de opcodes.
-   Codificación y tamaño de los operandos.
-   Endianness del bytecode.
-   Formato del archivo compilado y su extensión.
-   Representación de las direcciones y de los destinos de salto.
-   Modelo de memoria.
-   Convenciones para errores de compilación y de ejecución.

Estas decisiones deberán documentarse para que el compilador y la VM
implementen exactamente la misma especificación.

## 9. Principios de diseño

-   Mantener separadas la compilación y la ejecución.
-   Definir un contrato claro entre el generador y el cargador de
    bytecode.
-   No acoplar el formato de bytecode a detalles internos de Rust.
-   Validar tanto los programas durante la compilación como los archivos
    al cargarlos.
-   Mantener pequeña la primera versión y agregar complejidad de forma
    incremental.
-   Priorizar la comprensión de los mecanismos internos sobre el uso de
    dependencias que los oculten.
