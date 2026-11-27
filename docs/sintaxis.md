# Sintaxis básica de WTZ

Este documento propone una sintaxis inicial para WTZ, un lenguaje educativo
basado en pila. El compilador y la máquina virtual todavía no están
implementados; las reglas siguientes son una base de diseño y podrán cambiar.
El contexto general se describe en [Arquitectura.md](Arquitectura.md).

## Estructura de un programa

Los programas se escribirán en archivos con extensión `.wtz`. Se propone
escribir una instrucción por línea, con su nombre en mayúsculas y, cuando
corresponda, un operando separado por espacios o tabulaciones:

```text
INSTRUCCION operando
```

Las instrucciones sin operando se escribirán únicamente con su nombre. Los
espacios al inicio y al final de una línea y las líneas vacías se ignorarán.
No se utilizarán punto y coma ni llaves para delimitar instrucciones o bloques.

## Valores y pila

Para los ejemplos iniciales se utilizarán enteros decimales, como `10`, `0` y
`-3`. El rango y el tamaño de estos valores quedan pendientes de definición.

La pila almacena los valores en orden de llegada: el último valor agregado
queda en la cima y será el primero en retirarse. Las operaciones aritméticas
obtendrán sus operandos de la pila, por lo que se escribirá `ADD` en lugar de
una expresión como `10 + 20`.

## Instrucciones básicas propuestas

| Instrucción | Operando escrito | Comportamiento propuesto |
| --- | --- | --- |
| `PUSH valor` | Un entero decimal | Agrega el valor a la cima de la pila. |
| `POP` | Ninguno | Retira el valor de la cima. |
| `DUP` | Ninguno | Duplica el valor de la cima. |
| `SWAP` | Ninguno | Intercambia los dos valores superiores. |
| `ADD` | Ninguno | Retira dos valores y agrega su suma. |
| `SUB` | Ninguno | Retira dos valores y agrega su resta. |
| `MUL` | Ninguno | Retira dos valores y agrega su producto. |
| `PRINT` | Ninguno | Muestra el valor de la cima. Queda por decidir si lo retira. |
| `HALT` | Ninguno | Finaliza la ejecución. |

Para las operaciones binarias, se propone retirar primero `b` y luego `a`,
y calcular `a operación b`. Por ejemplo, `PUSH 10`, `PUSH 3`, `SUB` dejaría
`7` en la cima. La VM deberá informar un error si una instrucción requiere
más valores de los disponibles.

## Ejemplo básico

```asm
PUSH 10
PUSH 20
ADD
PRINT
HALT
```

El programa agregaría `10` y `20` a la pila. `ADD` retiraría ambos valores y
agregaría `30`; `PRINT` mostraría ese resultado y `HALT` finalizaría la
ejecución. El ejemplo es ilustrativo y todavía no puede ejecutarse en el
proyecto.

## Errores que deberá detectar el compilador

- Instrucciones desconocidas, como `SUMAR`.
- Operandos ausentes, como `PUSH` sin un valor.
- Operandos adicionales, como `ADD 10`.
- Literales incompatibles con el tipo esperado, como `PUSH texto`.

Los errores que dependan del estado de la pila deberán estar protegidos
también durante la ejecución, según la arquitectura prevista.

## Aspectos pendientes

Antes de cerrar la sintaxis habrá que definir:

- Comentarios y el símbolo que los introduce.
- Etiquetas y operandos de `JMP`, `JZ` y `JNZ`.
- Reglas de división y módulo para `DIV` y `MOD`.
- Sintaxis y comportamiento de `CALL`, `RET`, `LOAD`, `STORE` y `READ`.
- Rango de enteros, desbordamientos y formato de salida de `PRINT`.
- Si `HALT` será obligatorio y qué ocurrirá al llegar al final del archivo.

Estas instrucciones adicionales están previstas en la arquitectura, pero
su sintaxis todavía no se considera definida.
