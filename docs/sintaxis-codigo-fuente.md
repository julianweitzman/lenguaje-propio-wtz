# Sintaxis del código fuente

WTZ tendrá un lenguaje fuente de mayor nivel que se compilará a instrucciones
intermedias de una máquina de pila y después a bytecode binario. Su objetivo
es mejorar la legibilidad y permitir comprobar tipos antes de ejecutar.

Este documento es una propuesta inicial. No existen todavía compilador ni VM,
y la gramática definitiva queda pendiente.

## Reglas generales

- Cada programa deberá definir una única función `main` pública dentro de un namespace
  como punto de entrada.
- La inicialización será opcional en declaraciones de variables globales,
  locales y atributos. Leer un valor antes de inicializarlo será un error de
  compilación.
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
Una variable podrá ocultar otra del mismo nombre en un ámbito exterior.
Declarar dos variables con el mismo nombre dentro del mismo ámbito será un
error de compilación. Se admitirán declaraciones sin inicializador tanto
globales como locales; no habrá inicialización implícita a cero.

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

La asignación deberá preceder a cualquier lectura. El compilador deberá
comprobar que el valor esté inicializado en todos los caminos que lleguen a
una lectura; una asignación en una sola rama no será suficiente.

```c
i32 valor;
valor = 10;
println(valor);
```

En cambio, `i32 valor; println(valor);` será un error de compilación. La regla
también se aplicará a atributos y variables globales. Una operación que lea y
escriba, como `contador++`, requiere un valor inicializado previamente.
Los detalles del análisis entre funciones quedan pendientes, sin cambiar la
prohibición de leer valores no inicializados.

Los parámetros recibirán sus valores al llamar a la función y no necesitarán
un inicializador escrito en la declaración. Las restricciones de los
inicializadores globales siguen pendientes.

Por ejemplo, una variable local podrá ocultar una global:

```c
namespace aplicacion {
    i32 valor = 10;

    void ejemplo() {
        i32 valor = 20;
        println(valor);
    }
}
```

Dentro de `ejemplo`, el nombre sin prefijo `valor` designará la variable local.
La global conservará su valor. Los inicializadores globales permitidos y su
orden de evaluación todavía no están definidos.

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

### Condiciones

Se admitirán `if`, `else if` y `else`. Cada condición deberá producir un `bool`;
no habrá conversión automática de enteros a booleanos. Se evaluarán en orden y
se ejecutará únicamente el primer bloque cuya condición sea verdadera, o el
`else` si ninguna lo es. `else if` y `else` serán opcionales.

```c
if (edad < 18) {
    println("Menor");
} else if (edad == 18) {
    println("Tiene 18");
} else {
    println("Mayor");
}
```

Los paréntesis alrededor de las condiciones y las llaves de los cuerpos serán
obligatorios, incluso cuando el cuerpo contenga una sola sentencia.

### Bucles

Se admitirán `while` y `for`, con condiciones de tipo `bool` y cuerpos entre
llaves obligatorias.

```c
while (contador < 10) {
    contador++;
}

for (i32 i = 0; i < 10; i++) {
    println(i);
}
```

`while` comprobará la condición antes de cada vuelta; si es falsa desde el
principio, no ejecutará el cuerpo.

En `for (inicializacion; condicion; actualizacion)`, la inicialización se
ejecutará una sola vez. Antes de cada vuelta se comprobará la condición; tras
cada vuelta se ejecutará la actualización. Una variable declarada en la
inicialización solo será visible en la condición, actualización y cuerpo de
ese `for`, no después del bucle. La posibilidad de omitir partes de la cabecera
queda pendiente de definir.

El incremento posfijo `i++` usado en los ejemplos aumentará un `i32` en uno.
Sus reglas ante desbordamientos y el resto de variantes de incremento y
decremento se cerrarán junto con los operadores.

### Salidas y retornos

- `break;` terminará el bucle envolvente más cercano.
- `continue;` pasará a la siguiente vuelta del bucle envolvente más cercano.
  En `while` volverá a evaluar la condición; en `for` ejecutará primero la
  actualización y después comprobará la condición.
- `return expresion;` terminará la función y devolverá un valor compatible
  con su tipo de retorno.
- `return;` terminará una función `void` sin devolver un valor.

Usar `break` o `continue` fuera de un bucle será un error de compilación.
También será un error devolver un valor desde una función `void` o usar
`return;` en una función que deba devolver un valor. Todos los caminos que
puedan finalizar una función con retorno deberán devolver un valor compatible.

`switch`, `do`/`while` y `goto` quedan fuera de la primera versión. Tampoco se
incluye el operador condicional ternario. La traducción de estas estructuras
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
| Incremento y decremento | Incremento posfijo `i++` definido para los bucles; otras variantes de `++` y `--` pendientes. |

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
definitiva, la recursión y el paso de argumentos quedan pendientes; no se
heredan de C. Las reglas de retorno se describen en el control de flujo.

Los fragmentos de sentencias de este documento deberán colocarse dentro de una
función para formar un programa. Los namespaces agruparán las declaraciones de variables globales y funciones;
se propone agregar definiciones de structs dentro de ellos.

## Modelo de datos pendiente

La asignación, la copia y el paso de argumentos se definirán cuando se diseñe
cómo maneja los datos el programa. Como orientación inicial se considera un
modelo parecido al de C#, con tipos de valor y tipos de referencia. Esta es
una propuesta de diseño, no la adopción de sus reglas.

Todavía no se decide qué tipos pertenecerán a cada categoría, cuándo se copiará
un valor, cuándo se compartirá una referencia ni cómo se administrará su memoria.
Estas decisiones incluyen el comportamiento de los structs y los parámetros de
funciones. Tampoco se incorporan por ahora modificadores como `ref` u `out`.

## Namespaces y visibilidad

El lenguaje usará namespaces para organizar nombres y reducir la necesidad de
importaciones. Se declararán con `namespace nombre { ... }`.

Los miembros de un namespace serán privados por defecto. Esta regla no se
aplica a los atributos de structs, que serán siempre públicos. Para exponerlos a
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
`use biblioteca::matematicas;`. Se podrán declarar con
`namespace biblioteca::matematicas { ... }`.
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

Si varios namespaces incorporados mediante `use` exponen el mismo nombre,
utilizar ese nombre sin prefijo será un error de compilación por ambigüedad.
Una ruta completa permitirá elegir el miembro:

```c
namespace aplicacion {
    use matematicas;
    use estadisticas;

    public i32 main() {
        println(matematicas::sumar(1, 2));
        return 0;
    }
}
```

El ejemplo supone que ambos namespaces exponen `sumar`; una llamada sin prefijo
`sumar(1, 2)` sería ambigua. No se establece un error por el mero hecho de declarar
ambos `use` si no se utiliza el nombre ambiguo.

Un namespace podrá repartirse entre varios archivos dentro de la raíz del
programa y sus subdirectorios. Las declaraciones con la misma ruta de namespace
pertenecerán al mismo namespace, independientemente del archivo.

Se admitirán namespaces anidados y su declaración mediante rutas:

```c
namespace biblioteca::matematicas {
    public i32 sumar(i32 a, i32 b) {
        return a + b;
    }
}
```

La ruta de acceso será `biblioteca::matematicas::sumar(1, 2)` y se podrá escribir
`use biblioteca::matematicas;` para acceder a sus miembros públicos sin prefijo.
La jerarquía de namespaces no tendrá que coincidir con los directorios.

Quedan pendientes el procedimiento de descubrimiento y carga de los archivos,
el alcance exacto de `use` y las reglas de acceso privado entre namespaces
padre e hijo.

## Structs

Los structs definirán tipos compuestos con campos nombrados y tipados. No será
obligatorio inicializar los campos al declarar el struct:

```c
struct Persona {
    string nombre;
    i32 edad;
}
```

Los atributos (campos) de un struct serán siempre públicos, sin necesidad de
escribir `public` en cada declaración. No habrá atributos privados. Esta regla
no hace público automáticamente al tipo struct ni define la visibilidad de sus
métodos; la visibilidad pública tampoco determina si un atributo es mutable.

Los structs tendrán métodos, definidos en bloques `impl` separados de la
declaración de campos, tomando como referencia la organización de Rust. Se
mantendrá la notación de funciones de WTZ, con el tipo de retorno antes del
nombre; no se adopta automáticamente la sintaxis ni el modelo de datos de Rust.

La forma general del bloque será:

```text
impl Persona {
    ... métodos de Persona ...
}
```

Los métodos usarán declaraciones con la forma `tipo_retorno nombre(parametros)`
y cuerpos entre llaves. La sintaxis del receptor de instancia todavía debe
definirse; no se adopta automáticamente el sistema de préstamos de Rust.

### Acceso y modificación de atributos

Se accederá a los atributos con un punto y se podrán modificar mediante
asignación de un valor del tipo correspondiente:

```c
Persona persona = Persona { nombre: "Ana", edad: 20 };
println(persona.nombre);
persona.edad = 21;
```

Asignar un atributo lo inicializará si todavía no tenía valor. Leerlo requerirá
que esté inicializado, conforme a la regla general de comprobación en compilación.

### Funciones de creación y convención `new`

`new` será únicamente un nombre usado por convención para una función asociada
que crea y devuelve un struct. No será obligatorio definirla ni usar ese nombre;
no tendrá un tratamiento especial ni se generará automáticamente.

Si se elige esa convención, la función se escribirá dentro de un bloque `impl`,
podrá recibir argumentos y se llamará con `Tipo::new(argumentos)`:

```c
impl Persona {
    public Persona new(string nombre, i32 edad) {
        return Persona { nombre: nombre, edad: edad };
    }
}
```

```c
Persona persona = Persona::new("Ana", 20);
```

El ejemplo muestra una función asociada al tipo; no necesita un receptor de
instancia. Se usa `public` para que `new` pueda invocarse desde otro namespace;
las reglas generales de visibilidad de métodos siguen pendientes. Se podrá
usar otro nombre, como `crear`, y llamarlo mediante `Persona::crear(...)`.

### Creación mediante campos nombrados

También se podrá construir un valor directamente con la forma
`Tipo { campo: valor }`, sin pasar por `new`:

```c
Persona persona = Persona { nombre: "Ana", edad: 20 };
```

La inicialización de todos los campos no será obligatoria. Los campos omitidos
quedarán sin inicializar y cualquier lectura de ellos será un error de compilación.
No se les asignarán valores predeterminados de forma implícita.

Esta alternativa sustituye la restricción anterior que solo permitía crear
structs mediante `new`. El tipo se muestra explícitamente en los ejemplos;
la posibilidad de escribir únicamente `{ campo: valor }` cuando se conoce el
tipo por contexto queda pendiente de precisar.

También quedan pendientes:

- Visibilidad de métodos.
- Sintaxis del receptor y llamadas a métodos de instancia.
- Asignación, copia y paso de structs a funciones, junto con el modelo de datos.
- Ubicación y cantidad de bloques `impl` para un mismo tipo.
- Reglas detalladas para campos repetidos, inexistentes y tipos incompatibles
  en la construcción por campos nombrados.

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
- Firma y selección de `main`, recursión y paso de argumentos.
- Alcance de `use`, acceso privado entre namespaces padre e hijo y carga de archivos.
- Modelo de tipos de valor y referencia, copia y administración de memoria.
- Aplicación detallada de visibilidad a tipos struct y sus métodos; los atributos son públicos.
- Omisión de partes de la cabecera de `for` y reglas completas de operadores.
- Análisis de inicialización entre funciones, receptores de métodos y bloques `impl`.
- Reglas detalladas de construcción mediante campos nombrados.
- Copia de structs, junto con el modelo de datos.
- Operaciones adicionales de strings y conversiones aparte de `to_string(i32)`.
- Fin de entrada, errores de lectura, entrada no ASCII y formato de valores en la salida.
- Traducción a bytecode, que se abordará después de la sintaxis fuente.

Consultar la [arquitectura](arquitectura.md) para las responsabilidades de cada etapa.
