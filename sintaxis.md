# Sintaxis de WTZ

Estado: diseño inicial, sin implementación disponible para ejecutar los ejemplos.

Las decisiones acordadas son una sintaxis basada en expresiones y comentarios
con `#`. Las demás reglas de este documento son propuestas pendientes de
confirmación.

## Relación con la arquitectura

El código fuente se escribe en archivos `.wtz`. El compilador traducirá las
expresiones a instrucciones para una máquina virtual basada en pila.

Esta propuesta evoluciona el diseño inspirado en Assembly descrito en
[la arquitectura inicial](docs/arquitectura.md). Los ejemplos de instrucciones
de ese documento representan el enfoque anterior; todavía está pendiente
actualizarlo para reflejar la sintaxis de expresiones.

## Reglas generales propuestas

- Una sentencia por línea, sin punto y coma final.
- Palabras reservadas en mayúsculas.
- Nombres de variables, parámetros y funciones en `snake_case`.
- Identificadores sensibles a mayúsculas y minúsculas.
- Bloques cerrados con `END`; la sangría facilita la lectura y no delimita bloques.
- Paréntesis para agrupar expresiones y pasar argumentos.

## Comentarios

`#` inicia un comentario que termina al finalizar la línea. Puede aparecer
en una línea propia o después de una sentencia. Dentro de una cadena,
`#` forma parte del texto.

```text
# Calcular el total
LET precio = 20
LET cantidad = 3
LET total = precio * cantidad  # Resultado: 60

PRINT total
PRINT "# es parte de esta cadena"
```

## Tipos de datos propuestos

| Tipo | Descripción | Literales |
| --- | --- | --- |
| `INT` | Entero con signo de 64 bits | `42`, `-7` |
| `FLOAT` | Número de punto flotante de 64 bits | `3.14`, `-0.5` |
| `BOOL` | Valor lógico | `TRUE`, `FALSE` |
| `STRING` | Texto Unicode entre comillas dobles | `"Hola"`, `"Hola\nWTZ"` |

Se propone tipado estático con inferencia: el compilador determina el tipo
al declarar una variable y exige conservarlo en las asignaciones posteriores.
También se permite escribir el tipo explícitamente.

```text
LET edad = 20
LET precio: FLOAT = 19.95
LET activo: BOOL = TRUE
LET nombre: STRING = "Julián"
```

Para cadenas se proponen los escapes `\n` (salto de línea), `\t`
(tabulación), `\"` (comilla doble) y `\\` (barra inversa).

Arreglos, estructuras y valores nulos quedan fuera de esta propuesta inicial.

## Variables

`LET` declara una variable con valor inicial. `SET` modifica una variable
ya declarada y conserva su tipo.

```text
LET contador: INT = 3
SET contador = contador - 1
```

Se propone rechazar el uso de variables sin declarar y las asignaciones de
un tipo incompatible:

```text
LET edad = 20
SET edad = "Hola"  # Error de compilación propuesto: se esperaba INT
```

## Expresiones y operadores

| Categoría | Operadores | Regla propuesta |
| --- | --- | --- |
| Aritmética | `+`, `-`, `*`, `/` | Operandos numéricos del mismo tipo |
| Resto | `%` | Operandos INT |
| Negación numérica | `-valor` | Operando INT o FLOAT |
| Orden | `<`, `<=`, `>`, `>=` | Operandos numéricos del mismo tipo; resultado BOOL |
| Igualdad | `==`, `!=` | Operandos del mismo tipo; resultado BOOL |

La multiplicación, división y resto tienen prioridad sobre suma y resta.
La negación numérica tiene prioridad sobre los operadores binarios.
Las comparaciones tienen menor prioridad que la aritmética. Los operadores
aritméticos de igual prioridad se agrupan de izquierda a derecha.
Se propone exigir condiciones separadas en lugar de comparaciones encadenadas.

```text
LET resultado = 10 + 20 * 2    # 50
LET agrupado = (10 + 20) * 2  # 60
LET es_mayor = resultado >= 18
```

Se proponen conversiones explícitas, sin conversión automática entre INT y FLOAT:

```text
LET cantidad = 3
LET precio = 10.5
LET total = FLOAT(cantidad) * precio
```

La división entre INT produce un INT truncado hacia cero: `7 / 2` da `3`.
La división entre FLOAT produce un FLOAT: `7.0 / 2.0` da `3.5`.
El desbordamiento de enteros y la división por cero deben producir errores
controlados. El detalle de las conversiones restantes y de los valores
especiales de punto flotante queda pendiente.

## Entrada y salida

`PRINT` recibe una expresión. Se propone que muestre su valor y añada un
salto de línea.

```text
PRINT "Hola, WTZ"
PRINT 10 + 20
```

La forma propuesta para leer un valor es `READ()` dentro de una expresión:

```text
LET edad: INT = READ()
PRINT edad
```

El tipo explícito del ejemplo expresa el valor esperado. Queda pendiente
definir el contrato de READ, la conversión de la entrada, el manejo de
entradas inválidas y si podrá usarse sin una anotación de tipo.

## Condiciones

`IF` exige una expresión BOOL. La rama `ELSE` es opcional.

```text
LET edad = 20

IF edad >= 18
    PRINT "Puede ingresar"
ELSE
    PRINT "No puede ingresar"
END
```

No se propone interpretar automáticamente números como condiciones lógicas.

## Ciclos

`WHILE` evalúa una condición BOOL antes de cada iteración.

```text
LET contador = 3

WHILE contador > 0
    PRINT contador
    SET contador = contador - 1
END
```

Este programa mostraría `3`, `2` y `1`, cada uno en una línea.

## Funciones

Se propone declarar funciones con `FUNC`, separar parámetros con comas,
retornar valores con `RETURN` y realizar llamadas con paréntesis.

```text
FUNC doble(numero)
    RETURN numero * 2
END

LET resultado = doble(21)
PRINT resultado
```

La sintaxis de funciones es ilustrativa. Antes de implementarla habrá que
definir los tipos de parámetros y retorno, su anotación o inferencia, el
alcance de las variables, las funciones sin retorno y las reglas de recursión.

## Ejemplo completo del núcleo propuesto

```text
# Mostrar el costo de una compra
LET precio: INT = 20
LET cantidad: INT = 3
LET total = precio * cantidad

PRINT "Total de la compra:"
PRINT total

IF total >= 50
    PRINT "Compra de al menos 50"
ELSE
    PRINT "Compra menor a 50"
END
```

## Decisiones pendientes y alcance inicial

Se recomienda implementar primero INT, BOOL, variables, expresiones y PRINT;
después incorporar condiciones, ciclos, FLOAT, STRING, entrada y funciones.
Esta secuencia es una propuesta de implementación, no una restricción definitiva.

Además de las cuestiones señaladas en cada sección, quedan pendientes:

- Confirmar las palabras reservadas, las reglas de tipos y los operadores.
- Definir el alcance de variables y las declaraciones duplicadas.
- Precisar operaciones sobre STRING y operadores lógicos.
- Definir el punto de entrada y la finalización normal del programa.
- Definir límites de literales, diagnósticos y comportamiento numérico completo.
- Actualizar la arquitectura y especificar cómo se codifican los valores y las
  operaciones en el bytecode, sin acoplarlo a la sintaxis textual.

Los ejemplos todavía no pueden validarse mediante un compilador de WTZ.
