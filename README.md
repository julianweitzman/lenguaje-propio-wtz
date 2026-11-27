# WTZ

**Un lenguaje de programación basado en pila, con compilador y máquina
virtual propios.**

WTZ es un proyecto educativo cuyo objetivo es explorar cómo funcionan los lenguajes de programación, los compiladores, el bytecode y las máquinas virtuales mediante la implementación de sus componentes fundamentales.

El proyecto consiste en diseñar un lenguaje de sintaxis sencilla, inspirado en Assembly, desarrollar un compilador que traduzca sus programas a bytecode propio y construir una máquina virtual capaz de ejecutar ese bytecode.

La implementación se realizará principalmente en **Rust**.

## Características principales

-   Lenguaje de instrucciones simples y explícitas.
-   Archivos de código fuente con extensión **`.wtz`**.
-   Compilador propio.
-   Bytecode con un conjunto de opcodes definido por el proyecto.
-   Máquina virtual basada en pila (*stack-based virtual machine*).
-   Implementación de los componentes fundamentales sin delegar el
    funcionamiento central en bibliotecas externas.

## Ejemplo

Un programa WTZ podría verse así:

``` asm
PUSH 10
PUSH 20
ADD
PRINT
HALT
```

La ejecución coloca los valores `10` y `20` en la pila, los suma,
muestra el resultado y finaliza.

## Arquitectura general

El flujo principal del programa será:

``` text
Código fuente (.wtz)
        |
        v
      Lexer
        |
        v
      Parser
        |
        v
 Validación semántica
        |
        v
 Generador de bytecode
        |
        v
     Bytecode
        |
        v
 Máquina virtual (VM)
        |
        v
     Ejecución
```

El compilador traducirá las instrucciones del lenguaje directamente a
bytecode. No se plantea una representación intermedia (IR) independiente
en la primera versión, ya que el lenguaje estará formado inicialmente
por instrucciones elementales que se corresponden directamente con
operaciones de la máquina virtual.

La representación estructurada de las instrucciones que utilice
internamente el compilador no constituye, por sí sola, una IR
independiente.

Para más detalles, consultar
[`docs/Arquitectura.md`](docs/Arquitectura.md).

## Objetivos de aprendizaje

WTZ servirá para estudiar y poner en práctica:

-   Diseño de lenguajes y definición de gramáticas.
-   Análisis léxico y sintáctico.
-   Validación semántica.
-   Compilación y generación de bytecode.
-   Codificación y decodificación binaria.
-   Diseño de conjuntos de instrucciones (*instruction sets*).
-   Arquitectura de máquinas virtuales.
-   Pila, memoria y contador de programa.
-   Flujo de control, saltos y llamadas a funciones.
-   Representación de datos y administración de memoria.

## Tecnologías

-   **Lenguaje de implementación:** Rust.
-   **Gestión del proyecto:** Cargo.
-   **Control de versiones:** Git.
-   **Pruebas:** sistema de tests de Rust.

Se priorizará comprender e implementar los mecanismos centrales del
compilador y de la VM, evitando dependencias que oculten esos
mecanismos.

## Estructura inicial

La estructura podrá evolucionar durante el desarrollo. Como punto de
partida:

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
│   ├── vm/
│   │   ├── vm.rs
│   │   ├── stack.rs
│   │   ├── memory.rs
│   │   └── instruction.rs
│   ├── bytecode/
│   │   ├── opcode.rs
│   │   ├── encoder.rs
│   │   └── decoder.rs
│   └── main.rs
├── tests/
├── examples/
├── Cargo.toml
└── README.md
```

## Hoja de ruta

Las etapas son orientativas; se irán ajustando a medida que se definan e
implementen los componentes.

-   [ ] **1. Diseño inicial:** definir las instrucciones, sus operandos
    y el comportamiento de la máquina virtual.
-   [ ] **2. VM básica:** implementar la pila, el contador de programa y
    el ciclo de ejecución.
-   [ ] **3. Lenguaje:** definir la sintaxis de WTZ e implementar el
    lexer y el parser.
-   [ ] **4. Compilador:** validar instrucciones y generar bytecode.
-   [ ] **5. Formato de bytecode:** definir la codificación binaria, la
    carga y la validación de programas compilados.
-   [ ] **6. Control de flujo:** implementar saltos, etiquetas y
    llamadas.
-   [ ] **7. Memoria y funciones:** ampliar el modelo de memoria y
    agregar funciones y variables locales.
-   [ ] **8. Herramientas:** desarrollar un desensamblador, un depurador
    y posibles optimizaciones.

## Filosofía

WTZ no busca competir con lenguajes existentes ni convertirse
inicialmente en una herramienta de producción. Es un proyecto de
aprendizaje orientado a comprender cómo se diseña un lenguaje, cómo se
traduce a una representación ejecutable y cómo una máquina virtual
interpreta instrucciones.

Se priorizarán la claridad de la arquitectura, la comprensión de cada
componente y la evolución incremental del proyecto.

**Estado:** planificación y diseño inicial.
