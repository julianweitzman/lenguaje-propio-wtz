# Sintaxis del código fuente

WTZ tendrá un lenguaje fuente de mayor nivel que se compilará a instrucciones
intermedias de una máquina de pila y después a bytecode binario. Su objetivo
es mejorar la legibilidad y permitir comprobar tipos antes de ejecutar.

Este documento es una propuesta inicial. No existen todavía compilador ni VM,
y la gramática definitiva queda pendiente.

## Reglas generales

- Cada programa deberá definir una única función `main` pública dentro de un namespace
  como punto de entrada.
- Toda variable, global o local, deberá inicializarse al declararse.
- No habrá declaraciones de constantes por ahora.
- Existirán variables globales y locales; sus reglas de alcance se definirán explícitamente.
- El código fuente completo, incluidos comentarios y strings, será ASCII.
- C será únicamente una referencia de estilo para la estructura del código.
  Ninguna característica ni regla semántica se incorporará por analogía con C.

La documentación puede estar escrita en español con tildes; la restricción
ASCII se aplica a los programas y a sus datos de texto.

## Estilo y declaraciones

Los archivos fuente conservarán inicialmente la extensión `.wtz`:

```c
namespace aplicacion {
    public i32 main() {
        string nombre = "Ana";
        i32 edad = 20;
        print(nombre);
        print(edad + 1);
        return 0;
    }
}
```

La sintaxis seguirá el estilo de C: declaraciones con el tipo antes del
nombre (`tipo nombre = expresion;`), sentencias terminadas en `;`, llamadas
con paréntesis y bloques delimitados por `{` y `}`. Las expresiones usan
operadores entre sus operandos, como `edad + 1`.

Las palabras reservadas se escribirán en minúsculas. Los nombres de instrucciones
en mayúsculas, como `PUSH_I32`, pertenecen al bytecode. Adoptar este estilo no
incorpora automáticamente punteros, conversiones ni gestión de memoria de C.

## Tipos iniciales propuestos

| Tipo | Valores | Operaciones iniciales |
| --- | --- | --- |
| `i32` | Enteros con signo de 32 bits: de -2147483648 a 2147483647. | Suma, resta y multiplicación. |
| `bool` | `true` y `false`. | Condiciones. |
| `string` | Texto entre comillas dobles, como `"Ana"`. | Literales, concatenación e impresión; contenido inmutable. |

El compilador comprobará que las expresiones sean compatibles con el tipo
declarado. Por ejemplo, `i32 edad = "Ana";` deberá rechazarse.
No se proponen conversiones implícitas entre texto y números.

El tamaño de un `i32` no determina el tamaño de un string ni de su referencia.
Los strings tendrán longitud variable; su representación se explica en la
[documentación del bytecode](sintaxis-bytecode.md).

## Variables globales y locales

Las variables globales se declararán fuera de las funciones. Para las locales se
propone limitarlas al bloque donde se declaren y a sus bloques interiores.
Las reglas de ocultamiento y nombres repetidos quedan pendientes. Una declaración sin inicializador será un error,
incluso para una variable global; no habrá inicialización implícita a cero.

```c
namespace aplicacion {
    i32 limite = 10;

    public i32 main() {
        i32 contador = 0;
        while (contador < limite) {
            contador = contador + 1;
        }
        return 0;
    }
}
```

Los parámetros recibirán sus valores al llamar a la función y no necesitarán
un inicializador escrito en la declaración. Las restricciones de los
inicializadores globales siguen pendientes.

## Concatenación de strings

El operador `+` concatenará dos strings y producirá un nuevo string sin
modificar el contenido de los originales:

```c
string nombre = "Ana";
string saludo = "Hola, " + nombre + "!";

print(saludo);
```

El resultado será `Hola, Ana!`; `nombre` conservará el valor `"Ana"`.

Para `+`, ambos operandos deberán ser strings o números compatibles. No habrá
conversión implícita entre números y texto. Para concatenar un entero `i32`,
se utilizará la conversión explícita `to_string`, que devolverá su representación
decimal como string:

```c
i32 edad = 20;
string mensaje = "Edad: " + to_string(edad);

print(mensaje);
```

El resultado será `Edad: 20`. La expresión `"Edad: " + edad` será un error
de tipos. La conversión de otros tipos y la interpolación quedan pendientes.
La traducción de estas operaciones a bytecode se diseñará más adelante.

## Asignaciones y control de flujo

Se propone permitir reasignar una variable conservando su tipo:

```c
i32 edad = 20;
edad = edad + 1;
```

Las condiciones y los bucles tendrán paréntesis alrededor de la condición y
llaves para sus cuerpos:

```c
if (edad >= 18) {
    print("Mayor de edad");
} else {
    print("Menor de edad");
}

while (edad < 25) {
    edad = edad + 1;
}
```

Se propone que las condiciones produzcan un `bool`, sin convertir automáticamente
enteros a booleanos. Los ejemplos introducen las comparaciones `>=` y `<`; el
conjunto completo de operadores y su precedencia queda pendiente.
`if`/`else` y `while` constituyen la base propuesta. `for`, `do`/`while`,
`switch`, `break` y `continue` quedan pendientes de confirmación. La traducción
a bytecode se diseñará después.

## Operadores y notación pendientes

La siguiente tabla reúne candidatos de notación familiar. No constituye una
adopción de los operadores ni de las reglas de C: el conjunto exacto, los tipos
admitidos, la precedencia y la asociatividad deberán definirse para WTZ.

| Categoría | Operadores |
| --- | --- |
| Aritmética entera | `+`, `-`, `*`, `/`, `%`; signos unarios `+` y `-`. |
| Comparación e igualdad | `<`, `<=`, `>`, `>=`, `==`, `!=`. |
| Lógica | `&&`, `||`, `!`; `&&` y `||` tendrán cortocircuito. |
| Bits sobre enteros | `&`, `|`, `^`, `~`, `<<`, `>>`. |
| Asignación | `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=`. |
| Incremento y decremento | `++`, `--`, prefijos y posfijos. |

La concatenación de strings con `+` es una regla propia del lenguaje.
Los detalles de conversiones numéricas y errores aritméticos requieren una
especificación adicional; no se promete copiar comportamientos indefinidos de C.

También se propone la notación de comentarios `//` y `/* ... */`, y escapes
como `\t`, `\\` y `\"`, restringidos a ASCII. Su conjunto completo queda
pendiente. El escape `\n` ya está definido para insertar saltos de línea.

## Funciones

Las funciones declararán el tipo de retorno antes del nombre y el tipo de cada
parámetro antes de su identificador:

```c
i32 sumar(i32 a, i32 b) {
    return a + b;
}

void saludar(string nombre) {
    print(nombre);
}
```

`return expresion;` devolverá un valor compatible con el tipo de retorno.
`void` indicará que una función no devuelve un valor; no se propone como tipo
de variable. Las llamadas usarán argumentos separados por comas, por ejemplo
`sumar(10, 20)` o `saludar("Ana")`.

La existencia de `main` dentro de un namespace y declarada con `public` es obligatoria. Se propone la firma `i32 main()`, sin
parámetros, y `return 0;` para representar una terminación correcta. La firma
definitiva, las reglas de retorno, la recursión y el paso de argumentos quedan
pendientes; no se heredan de C.

Los fragmentos de sentencias de este documento deberán colocarse dentro de una
función para formar un programa. Los namespaces agruparán las declaraciones de variables globales y funciones;
se propone agregar definiciones de structs dentro de ellos. El uso de
`return;` en funciones `void` queda pendiente de cerrar junto con las reglas
de retorno.

## Namespaces y visibilidad

El lenguaje usará namespaces para organizar nombres y reducir la necesidad de
importaciones. Se declararán con `namespace nombre { ... }`.

Los miembros de un namespace serán privados por defecto. Para exponerlos a
otros namespaces se escribirá `public` antes de la declaración. Una ruta
completa respetará las mismas restricciones de visibilidad.

```c
namespace matematicas {
    public i32 sumar(i32 a, i32 b) {
        return a + b;
    }

    i32 auxiliar(i32 valor) {
        return valor + 1;
    }
}

namespace aplicacion {
    public i32 main() {
        println(matematicas::sumar(10, 20));
        return 0;
    }
}
```

`matematicas::sumar(...)` estará disponible sin `use`. La función `auxiliar`
será privada a `matematicas` y no podrá usarse desde `aplicacion`, tampoco
mediante una ruta completa.

### Uso de un namespace

`use matematicas;` permitirá acceder sin prefijo a todos los miembros públicos
de ese namespace. No expondrá sus miembros privados.

```c
namespace aplicacion {
    use matematicas;

    public i32 main() {
        println(sumar(10, 20));
        return 0;
    }
}
```

Este ejemplo usa el namespace `matematicas` definido anteriormente. Las rutas
completas seguirán disponibles aunque se utilice `use`.

Las rutas a namespaces anidados podrán expresarse con `::`, como
`use biblioteca::matematicas;`; sus reglas de declaración quedan pendientes.
No se fija todavía una forma de importar un único miembro.

`main` deberá pertenecer a un namespace y declararse con `public`. No tendrá
una excepción a la visibilidad privada por defecto. La selección del namespace
de entrada sigue pendiente.

### Ubicación de los archivos

El directorio que contiene el archivo donde se declara `main` será la raíz del
programa. Los archivos fuente `.wtz` deberán estar en ese directorio (`./`) o
en sus subdirectorios (`./dir*/`), tomando siempre esa raíz como referencia,
no el directorio desde el que se ejecute el compilador.

```text
programa/
├── principal.wtz       (contiene main)
├── utilidades.wtz
└── calculo/
    └── operaciones.wtz
```

Los archivos fuera de esa raíz no pertenecerán al conjunto de fuentes del
programa. La ubicación física no define todavía el nombre del namespace.

Quedan pendientes el procedimiento de descubrimiento y carga de los archivos,
si un namespace puede repartirse entre varios archivos, los namespaces anidados
y el alcance exacto y los conflictos de nombres de `use`.

## Structs

Se propone incorporar tipos compuestos con campos nombrados y tipados:

```c
struct Persona {
    string nombre;
    i32 edad;
}
```

Este ejemplo define la forma propuesta del tipo, no crea una variable. La
sintaxis todavía no constituye una especificación completa de structs.

Antes de incorporarlos hay que definir:

- Cómo crear e inicializar un valor del struct.
- Cómo acceder a sus campos y si se pueden modificar.
- Cómo se comportan la asignación, la copia y el paso de structs a funciones.
- Cómo se aplica la inicialización obligatoria a todos sus campos.

No se asumen las reglas de structs de C ni se fija su representación en memoria.

## Entrada y salida

La función incorporada `string input();` leerá una línea de la entrada estándar
y devolverá un string ASCII sin el terminador de línea. No recibirá parámetros.
El tratamiento de fin de entrada, errores de lectura y caracteres no ASCII
queda pendiente.

`print(valor)` mostrará el valor sin agregar un salto de línea.
`println(valor)` mostrará el valor y agregará un salto de línea al final.
También se podrá insertar un salto mediante el escape `\n` dentro de un string.

```c
namespace aplicacion {
    public i32 main() {
        print("Nombre: ");
        string nombre = input();
        println("Hola, " + nombre);
        print("Fin\n");
        return 0;
    }
}
```

En este ejemplo, `print` deja el cursor después de la pregunta, `input` obtiene
el nombre y `println` termina el saludo con un salto de línea. `print("Fin\n")`
produce el mismo texto y salto final que `println("Fin")`.

## Relación con el bytecode

Por ahora se diseñará la sintaxis fuente y sus reglas sin fijar cómo se traducen
a instrucciones de pila. La documentación de bytecode contiene propuestas que
se revisarán más adelante y no limitan las características del lenguaje fuente.

## Decisiones pendientes

- Gramática formal y restricciones de identificadores e inicializadores globales.
- Conversiones numéricas, división, desbordamientos y desplazamientos inválidos.
- Firma y selección de `main`, retornos, recursión y paso de argumentos.
- Alcance y conflictos de `use`, namespaces anidados y organización de módulos y archivos.
- Aplicación detallada de visibilidad a structs y sus campos.
- Inclusión de otras estructuras de control y reglas completas de operadores.
- Creación, acceso a campos, mutabilidad, copia e inicialización de structs.
- Operaciones adicionales de strings y conversiones aparte de `to_string(i32)`.
- Fin de entrada, errores de lectura, entrada no ASCII y formato de valores en la salida.
- Traducción a bytecode, que se abordará después de la sintaxis fuente.

Consultar la [arquitectura](arquitectura.md) para las responsabilidades de cada etapa.
