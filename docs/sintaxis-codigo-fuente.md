# Sintaxis del código fuente

WTZ tendrá un lenguaje fuente de mayor nivel que se compilará a instrucciones
intermedias de una máquina de pila y después a bytecode binario. Su objetivo
es mejorar la legibilidad y permitir comprobar tipos antes de ejecutar.

Este documento recoge las decisiones de la primera versión. No existen todavía
compilador ni VM; la gramática formal y la implementación quedan pendientes.

## Reglas generales

- Cada programa deberá definir una única función `main` dentro de un namespace
  como punto de entrada.
- La inicialización será opcional en declaraciones de variables globales,
  locales y atributos. Leer un valor antes de inicializarlo será un error de
  compilación.
- Las declaraciones globales se escribirán con `const`. Las constantes serán
  inmutables y también podrán declararse dentro de funciones.
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
    i32 main() {
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

## Tipos iniciales

| Tipo | Valores | Operaciones iniciales |
| --- | --- | --- |
| `i32` | Enteros con signo de 32 bits: de -2147483648 a 2147483647. | Aritmética entera y comparaciones. |
| `bool` | `true` y `false`. | Condiciones. |
| `string` | Texto entre comillas dobles, como `"Ana"`. | Literales, concatenación e impresión; contenido inmutable. |

El compilador comprobará que las expresiones sean compatibles con el tipo
declarado. Por ejemplo, `i32 edad = "Ana";` deberá rechazarse.
No se proponen conversiones implícitas entre texto y números.

El tamaño de un `i32` no determina el tamaño de un string ni de su referencia.
Los strings tendrán longitud variable; su representación se explica en la
[documentación del bytecode](sintaxis-bytecode.md).

## Tipos anulables

`T?` representará un valor de tipo `T` o su ausencia, escrita como `null`.
Por ejemplo, `string` no admitirá `null`, pero `string?` sí. `null` será distinto
del string vacío `""` y de una variable sin inicializar: asignar `null` inicializa
una variable anulable, mientras que leer una variable no inicializada es un error.

Antes de usar un valor anulable como su tipo base deberá comprobarse que no es
`null`. El compilador reconocerá esa comprobación en la rama correspondiente:

```c
string? entrada = input();
if (entrada != null) {
    println(entrada);
} else {
    println("Fallo de lectura");
}
```

## Variables globales y locales

Las variables globales se declararán fuera de las funciones, con la forma
`const tipo nombre`, por ejemplo `const i32 limite = 10;`. Esta decisión
reemplaza la ausencia inicial de `const`. Las locales se
limitarán al bloque donde se declaren y a sus bloques interiores.
Una variable podrá ocultar otra del mismo nombre en un ámbito exterior.
Declarar dos variables con el mismo nombre dentro del mismo ámbito será un
error de compilación. Se admitirán declaraciones sin inicializador tanto
globales como locales; no habrá inicialización implícita a cero.

```c
namespace aplicacion {
    const i32 limite = 10;

    i32 main() {
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
La comprobación se realizará por variable y por campo. Antes de copiar un
valor o pasar una referencia, los datos que puedan leerse deberán estar
inicializados. No habrá parámetros exclusivos de salida en esta versión.

`const` declarará un valor inmutable tanto en el ámbito global como dentro de
funciones:

```c
const i32 limite = 10;
```

Una constante podrá declararse sin inicializador, pero admitirá exactamente una
asignación en cada camino de ejecución. Leerla antes de esa asignación o intentar
reasignarla será un error de compilación. Tampoco se podrán modificar sus campos
ni obtener referencias mutables hacia ella o sus campos. Una constante no podrá
almacenar referencias mutables, tampoco dentro de sus campos.

Los inicializadores globales solo admitirán literales y expresiones calculables
al compilar, incluidas otras constantes globales resolubles. Se rechazarán ciclos,
llamadas a funciones y referencias a variables locales. Si se omite el inicializador,
la primera asignación estará sujeta a estas mismas restricciones globales.

Los parámetros recibirán sus valores al llamar a la función y no necesitarán
un inicializador escrito en su declaración.

Por ejemplo, una variable local podrá ocultar una global:

```c
namespace aplicacion {
    const i32 valor = 10;

    void ejemplo() {
        i32 valor = 20;
        println(valor);
    }
}
```

Dentro de `ejemplo`, el nombre sin prefijo `valor` designará la variable local.
La global conservará su valor. Las dependencias entre inicializadores globales
se resolverán al compilar, independientemente del orden de escritura.

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

Se permitirá reasignar una variable conservando su tipo:

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
ese `for`, no después del bucle. Cualquiera de las tres partes podrá omitirse;
una condición omitida equivale a `true`. Por ejemplo, `for (;;) { break; }`.

El incremento posfijo `i++` usado en los ejemplos aumentará un `i32` en uno.
Se admitirán también decremento y variantes prefijas. El overflow será modular
como en el resto de la aritmética entera.

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

## Operadores, evaluación y conversiones

No habrá operaciones de bits: quedan excluidos AND, OR y XOR bit a bit,
complemento, desplazamientos y sus asignaciones compuestas. `&` se usará solo
para obtener referencias; `&&` y `||` serán operadores lógicos booleanos.

La siguiente tabla fija la precedencia de mayor a menor:

| Nivel | Operadores |
| --- | --- |
| 1 | Llamadas `()`, acceso a campos `.`, rutas `::`. |
| 2 | Incremento y decremento posfijos: `x++`, `x--`. |
| 3 | Unarios: `!`, `+`, `-`, desreferencia `*`, referencia `&`, `++x`, `--x`. |
| 4 | Multiplicación, división y módulo: `*`, `/`, `%`. |
| 5 | Suma, resta y concatenación: `+`, `-`. |
| 6 | Comparaciones: `<`, `<=`, `>`, `>=`. |
| 7 | Igualdad: `==`, `!=`. |
| 8 | AND lógico: `&&`. |
| 9 | OR lógico: `||`. |
| 10 | Asignaciones: `=`, `+=`, `-=`, `*=`, `/=`, `%=`. |

Los paréntesis cambiarán el agrupamiento. Las asignaciones y los unarios se
agruparán de derecha a izquierda; los restantes operadores binarios, de izquierda
a derecha. Así, `2 + 3 * 4` dará `14`, y `(2 + 3) * 4` dará `20`.
Los operandos y argumentos se evaluarán de izquierda a derecha. `&&` y `||`
tendrán cortocircuito: no evaluarán el segundo operando si el primero determina
el resultado. Las operaciones lógicas requerirán `bool`; la aritmética requerirá
`i32`, salvo la concatenación de dos strings con `+`.

Los operadores de incremento y decremento requerirán un destino mutable `i32`.
Los prefijos producirán el valor actualizado; los posfijos, el valor anterior.
Las asignaciones compuestas evaluarán el destino una sola vez.

No habrá conversiones implícitas entre tipos base distintos. Las conversiones
numéricas explícitas usarán `tipo(expresion)` cuando existan otros tipos numéricos;
por ahora el único tipo numérico será `i32`. El texto se obtendrá con `to_string`.

El overflow de `i32` será modular: se conservarán los 32 bits inferiores y se
interpretarán en complemento a dos. `2147483647 + 1` dará `-2147483648`.
La división entera truncará hacia cero; el resto conservará el signo del dividendo.
Dividir o calcular módulo por cero producirá un error controlado de ejecución.
`-2147483648 / -1` dará `-2147483648`, y su resto será cero.

## Identificadores, comentarios y strings

Los identificadores usarán letras ASCII, dígitos y `_`, sin comenzar con dígito.
Se distinguirán mayúsculas de minúsculas y no se podrán usar palabras reservadas
como nombres.

Los comentarios serán `//` hasta el final de la línea y `/* ... */`, sin
anidamiento. Los strings ocuparán una sola línea física y admitirán los escapes
`\n`, `\r`, `\t`, `\\`, `\"` y `\0`. Todos sus valores serán ASCII.

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

La firma del punto de entrada será `i32 main()`, sin parámetros, dentro de un
namespace y sin modificador `public`. `return 0;` indicará terminación correcta.
Se permitirá recursión y llamar a funciones declaradas después, sin declaraciones
anticipadas, incluida recursión mutua. No habrá sobrecarga, parámetros opcionales
ni cantidad variable de argumentos. Cada llamada deberá respetar la cantidad y
los tipos de parámetros. El paso de argumentos se describe a continuación.

Los fragmentos de sentencias de este documento deberán colocarse dentro de una
función para formar un programa. Los namespaces agruparán las declaraciones de variables globales y funciones;
también contendrán definiciones de structs.

## Paso de argumentos y modelo de datos

Un parámetro declarado como `tipo nombre` recibirá una copia. Cambiar ese
parámetro dentro de la función no modificará el valor del llamador:

```c
void cambiar(i32 numero) {
    numero = 20;
}
```

Un parámetro declarado como `tipo* nombre` permitirá modificar el dato original
del llamador. El asterisco expresa aquí el acceso al original en lugar de una
copia:

```c
void cambiar_original(i32* numero) {
    *numero = 20;
}
```

La llamada pasará una referencia explícita mediante `&variable`:

```c
i32 valor = 10;
cambiar_original(&valor);
```

`*numero` permitirá leer o modificar el dato original. También se podrán guardar
referencias en variables: `i32* referencia = &valor;`. No habrá aritmética de
punteros. Las referencias a temporales y los usos que puedan sobrevivir al dato
original serán errores de compilación. No se permitirá retornar referencias en
la primera versión.

`T*` no admitirá `null`; `T*?` será una referencia anulable que requerirá una
comprobación antes de desreferenciarse. No se podrán usar referencias para
modificar constantes ni sus campos.

Copiar un struct copiará sus campos. Un campo de referencia copiará la referencia
y compartirá el dato apuntado, sin hacer una copia profunda. Los strings conservarán
su contenido inmutable. El algoritmo de comprobación de vida útil y la estrategia
interna de administración de memoria quedan pendientes de implementación.

## Namespaces y visibilidad

El lenguaje usará namespaces para organizar nombres y reducir la necesidad de
importaciones. Se declararán con `namespace nombre { ... }`.

Todo será público: funciones, variables globales, tipos, atributos y métodos.
No habrá encapsulamiento ni se necesitarán modificadores `public` o `private`.
Esta decisión sustituye la visibilidad privada por defecto.

```c
namespace matematicas {
    i32 sumar(i32 a, i32 b) {
        return a + b;
    }

    i32 auxiliar(i32 valor) {
        return valor + 1;
    }
}

namespace aplicacion {
    i32 main() {
        println(matematicas::sumar(10, 20));
        return 0;
    }
}
```

Las funciones `sumar` y `auxiliar` serán accesibles desde otros namespaces
mediante sus rutas completas, sin `use`.

### Uso de un namespace

`use matematicas;` permitirá acceder sin prefijo a todos los miembros públicos
de ese namespace. Su efecto se limitará al archivo donde se escriba y no se
propagará a otros archivos del mismo namespace. Afectará todo el archivo,
independientemente de dónde se escriba, incluidos sus distintos bloques de
namespace. No incorporará automáticamente los namespaces hijos.

```c
namespace aplicacion {
    use matematicas;

    i32 main() {
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

`main` deberá pertenecer a un namespace y será pública sin modificador, como
todas las funciones. El programa deberá contener un único `main`.

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

    i32 main() {
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
    i32 sumar(i32 a, i32 b) {
        return a + b;
    }
}
```

La ruta de acceso será `biblioteca::matematicas::sumar(1, 2)` y se podrá escribir
`use biblioteca::matematicas;` para acceder a sus miembros públicos sin prefijo.
La jerarquía de namespaces no tendrá que coincidir con los directorios.

El compilador recibirá el archivo de `main` y reunirá recursivamente todos los
archivos `.wtz` de su directorio y subdirectorios, sin seguir enlaces simbólicos.
Rechazará nombres duplicados en el mismo ámbito y más de un `main`.
No habrá restricciones de visibilidad entre namespaces padre e hijo.

## Structs

Los structs definirán tipos compuestos con campos nombrados y tipados. No será
obligatorio inicializar los campos al declarar el struct:

```c
struct Persona {
    string nombre;
    i32 edad;
}
```

Tipos struct, atributos y métodos serán siempre públicos sin modificadores.

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
y cuerpos entre llaves. Los métodos de instancia usarán `self` como primer
parámetro explícito para recibir una copia o `self*` para modificar el original.
Ambas variantes se invocarán con punto, como `persona.cumplir_anios()`:

```c
impl Persona {
    void cumplir_anios(self*) {
        self.edad = self.edad + 1;
    }
}
```

Las funciones sin `self` se llamarán mediante el tipo. Se permitirán varios
bloques `impl` por tipo; repetir la definición de un método será un error de
compilación. Los bloques `impl` deberán estar en el namespace del struct, pero
podrán repartirse entre varios archivos. Un método con `self*` no podrá llamarse
sobre una constante.

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
    Persona new(string nombre, i32 edad) {
        return Persona { nombre: nombre, edad: edad };
    }
}
```

```c
Persona persona = Persona::new("Ana", 20);
```

El ejemplo muestra una función asociada al tipo, sin receptor de instancia.
Será pública como todas las funciones. También podrá llamarse `crear` u otro
nombre, con llamadas como `Persona::crear(...)`.

### Creación mediante campos nombrados

También se podrá construir un valor directamente con la forma
`Tipo { campo: valor }`, sin pasar por `new`:

```c
Persona persona = Persona { nombre: "Ana", edad: 20 };
```

La inicialización de todos los campos no será obligatoria. Los campos omitidos
quedarán sin inicializar y cualquier lectura de ellos será un error de compilación.
No se les asignarán valores predeterminados de forma implícita.

El orden de los campos no importará. Repetir un campo, nombrar uno inexistente
o asignarle un valor incompatible será un error de compilación. Se exigirá
el nombre del tipo: `Persona { ... }`, sin la forma abreviada `{ ... }`.

Los parámetros de structs seguirán la distinción entre copia y acceso al
original mediante `tipo*`, según las reglas del modelo de datos.

## Entrada y salida

La función incorporada `input()` leerá una línea de la entrada estándar y
no recibirá parámetros. En caso de éxito devolverá texto ASCII sin el terminador
de línea; una línea vacía devolverá `""`. En caso de fallo devolverá `null`.
Fin de entrada, fallos de lectura o caracteres fuera de ASCII devolverán `null`,
sin entregar texto parcial.

`print` y `println` aceptarán un argumento `i32`, `bool` o `string`. Los booleanos
se mostrarán como `true` o `false`. Los valores anulables requerirán una
comprobación previa.

La firma será `string? input()`: devolverá un string o `null`. Antes de usar
el resultado como texto habrá que comprobar que no sea `null`.

```c
namespace aplicacion {
    i32 main() {
        print("Nombre: ");
        string? nombre = input();
        if (nombre != null) {
            println("Hola, " + nombre);
        } else {
            println("Fallo de lectura");
        }
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

## Límites de la primera versión

No habrá arrays, genéricos, herencia, excepciones, administración manual de
memoria ni aritmética de punteros. Tampoco operaciones de bits, ternario,
`switch`, `do`/`while` ni `goto`.

## Trabajo pendiente

- Escribir la gramática formal a partir de estas reglas.
- Preparar programas completos que combinen las características acordadas.
- Precisar reglas de tipos para comparaciones de strings, structs y referencias.
- Diseñar el análisis de inicialización, constantes, anulables y vida útil.
- Definir la administración interna de memoria sin exponer memoria manual.
- Abordar la traducción a bytecode después de cerrar el lenguaje fuente.

Consultar la [arquitectura](arquitectura.md) para las responsabilidades de cada etapa.
