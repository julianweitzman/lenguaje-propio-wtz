# Sintaxis del código fuente

WTZ tendrá un lenguaje fuente de mayor nivel que se compilará a instrucciones
intermedias de una máquina de pila y después a bytecode binario. Su objetivo
es mejorar la legibilidad y permitir comprobar tipos antes de ejecutar.

Este documento recoge las decisiones de la primera versión. No existen todavía
compilador ni VM; la gramática formal y la implementación quedan pendientes.

## Reglas generales

- Cada programa deberá definir una única función `main` dentro de un namespace
  como punto de entrada.
- La inicialización será opcional en variables locales mutables y atributos.
  Toda declaración `const`, global o local, deberá incluir un inicializador.
  Leer un valor antes de inicializarlo será un error de compilación.
- Las declaraciones globales se escribirán con `const`. Las constantes serán
  inmutables y también podrán declararse dentro de funciones.
- Existirán variables globales y locales, con el alcance descrito en este documento.
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

Se permitirá asignar `T` directamente a `T?`, por ejemplo `i32? numero = 10;`.
Esto no permitirá asignar un entero a una referencia: `i32* referencia = 10;`
será un error de tipos; una referencia deberá recibir una referencia compatible.

La composición de tipos se limitará definitivamente a `T?`, `T*` y `T*?`, donde
`T` sea un tipo base o un struct. No se admitirán `T?*`, referencias a referencias
ni capas adicionales de modificadores.

Antes de usar un valor anulable como su tipo base deberá comprobarse que no es
`null`. El compilador reconocerá comprobaciones en `if`/`else`, `while`,
cortocircuito de `&&` y `||` y salidas tempranas. Una asignación que cambie el
valor invalidará la comprobación. Si una llamada lo cambia a través de una
referencia y se intenta usar como presente, se producirá un error en ejecución;
no se supondrá que una comprobación anterior garantiza su valor para siempre.
Una comprobación reconocida habilitará el uso del valor como su tipo base en
el camino correspondiente. Si una llamada puede mutarlo indirectamente, ese
uso seguirá admitiéndose, pero comprobará en ejecución que el valor leído siga
presente. Si es `null`, la ejecución terminará con un diagnóstico; no se
convertirá en un valor predeterminado ni se desreferenciará como si existiera.
Una asignación directa invalidará la prueba estática y exigirá otra comprobación.

Cada uso leerá y validará el valor una sola vez antes de consumirlo. Copiar un
valor ya comprobado a una variable no anulable conservará esa copia aunque
cambie el origen. El compilador solo podrá eliminar una comprobación dinámica
si demuestra que no cambia este comportamiento.

```c
i32? numero = 10;
```

Ejemplo de comprobación de entrada:

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
error de compilación. Las variables locales mutables podrán declararse sin
inicializador; las globales, al ser `const`, deberán inicializarse al declararse.
No habrá inicialización implícita a cero.

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
La comprobación se realizará por variable y por campo. Se permitirá copiar un
struct incompleto y se propagará qué campos están inicializados entre
asignaciones y funciones. Copiar un campo ausente no lo inicializará ni permitirá
leerlo. Esto reemplaza la exigencia anterior de tener el struct completo antes
de copiarlo o pasarlo a una función; habrá que diseñar los contratos de
inicialización de parámetros para conservar esta información.

El compilador analizará condiciones y cantidad de iteraciones cuando pueda
demostrarlas; si no demuestra la inicialización, rechazará la lectura. Por
ejemplo, inicializar `x` solo dentro de un bucle que puede ejecutar cero veces
no permitirá leer `x` después del bucle. No se asume que cualquier bucle vaya
a ejecutarse al menos una vez.

`const` declarará un valor inmutable tanto en el ámbito global como dentro de
funciones:

```c
const i32 limite = 10;
```

Toda constante deberá inicializarse en su declaración, tanto globalmente como
dentro de funciones. `const i32 limite;` será un error de compilación. No se
admitirá inicialización diferida ni reasignación. Tampoco se podrán modificar sus campos
ni obtener referencias mutables hacia ella o sus campos. Una constante no podrá
almacenar referencias mutables, tampoco dentro de sus campos. Una constante
de tipo struct deberá tener todos sus campos inicializados, también los de
structs anidados.

Los inicializadores globales admitirán operaciones puras sobre literales y otras
constantes globales: aritmética, lógica, comparaciones admitidas, concatenación
y literales de structs completos. Se rechazarán ciclos, referencias y llamadas
a funciones. Los errores de evaluación, incluida división por cero, se informarán
al iniciar el programa, antes de ejecutar `main`, en lugar de producir un error
de compilación. La evaluación anticipada no podrá ocultar ese comportamiento.
Los errores de sintaxis y tipos seguirán siendo errores de compilación.

Las constantes locales podrán usar expresiones evaluadas al ejecutar su
declaración, incluidas llamadas a funciones, respetando el tipo declarado.

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
se resolverán independientemente del orden de escritura. Los fallos de
evaluación se comunicarán al iniciar el programa.

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
de tipos. `to_string` también admitirá `bool` y `null`: producirá `"true"`,
`"false"` y `"null"`, respectivamente. La conversión de un anulable presente
requerirá antes su comprobación; no se fija una conversión genérica de `T?`.
La interpolación queda fuera de lo definido.
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
Además, una función con retorno deberá contener un retorno explícito alcanzable
que cumpla su firma; un bucle infinito sin retorno no sustituirá esa obligación.
Agregar un `return` inalcanzable después del bucle tampoco bastará. El compilador
deberá analizar el flujo, no solo buscar la palabra `return`.

En una función `void`, llegar al final del cuerpo equivaldrá a `return;`.

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

## Comparaciones

| Tipo | Igualdad `==` y `!=` | Orden `<`, `<=`, `>` y `>=` |
| --- | --- | --- |
| `i32` | Por valor. | Orden numérico con signo. |
| `bool` | Por valor. | No permitido; tampoco admite aritmética. |
| `string` | Por contenido ASCII completo. | No permitido. |
| Struct | Por identidad, no por igualdad de campos. | No permitido. |
| Referencia | Por identidad del dato apuntado y tipos compatibles. | No permitido. |

Cada instancia de struct tendrá una identidad lógica propia, independiente de
sus campos y de su dirección física. `persona == persona` será verdadero; dos
instancias distintas serán diferentes aunque sus campos coincidan.

Inicializar una variable mediante copia, pasar un argumento por valor o
materializar un resultado retornado producirá una instancia con identidad propia.
La igualdad observará las instancias de sus operandos, sin introducir copias
adicionales para comparar. Las optimizaciones no podrán cambiar ese resultado.

```c
Persona persona = Persona { nombre: "Ana", edad: 20 };
Persona otra = persona;
println(persona == otra);
println(persona == persona);
```

El ejemplo mostrará `false` y `true`. Asignar nuevos campos o copiar otro valor
sobre una instancia ya existente conservará la identidad de la instancia destino.
Una referencia o su copia seguirá identificando el mismo dato original; copiar
un struct no lo convertirá en una referencia compartida. Los campos de referencia
conservarán su semántica de compartir el dato apuntado.

Entre dos anulables compatibles, dos `null` serán iguales, un ausente y un
presente serán distintos, y dos presentes usarán la igualdad de su tipo base.
Comparar una referencia no comparará el contenido apuntado.

## Identificadores, comentarios y strings

Los identificadores usarán letras ASCII, dígitos y `_`, sin comenzar con dígito.
Se distinguirán mayúsculas de minúsculas y no se podrán usar palabras reservadas
como nombres. Se aceptarán espacios, tabulaciones y finales de línea LF o CRLF;
se rechazarán BOM, CR aislado y controles ajenos fuera de strings.

Los comentarios serán `//` hasta el final de la línea y `/* ... */`, sin
anidamiento. Los strings ocuparán una sola línea física y admitirán los escapes
`\n`, `\r`, `\t`, `\\`, `\"` y `\0`. Todos sus valores serán ASCII.
Los strings conservarán su longitud: `"A\0B"` tendrá tres caracteres; el cero
no será un terminador de texto.

Los literales enteros admitirán decimal, hexadecimal, octal, binario y separadores
`_`. El único tipo numérico seguirá siendo `i32`; no se introducen flotantes.
Los prefijos y las posiciones válidas del separador se precisarán en la gramática.
Los valores escritos se reducirán módulo 2^32 y se interpretarán con signo,
incluso para literales arbitrariamente largos. Esta regla de literales es distinta
del análisis estricto de rango de `parse_i32`.

No se permitirán comas finales en argumentos, parámetros ni campos de un literal
struct. Cada declaración de variable declarará un solo nombre. Las definiciones
de struct terminarán con `}`, sin punto y coma adicional; sus campos sí llevarán
`;`. Se permitirá cualquier expresión como sentencia terminada en `;`, descartando
su resultado.

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
namespace y sin modificador `public`. `return 0;` indicará terminación correcta y otros valores indicarán fallo.
El resultado se usará como estado de salida; su adaptación al rango del sistema
operativo deberá documentarse para cada plataforma.
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
original serán errores de compilación. Una referencia podrá sobrevivir al bloque
donde se obtuvo si el dato original vive lo suficiente; se rechazará lo que no
pueda demostrarse. No se permitirá retornar referencias ni structs que las
contengan.

Se permitirán varias referencias mutables al mismo dato. El usuario será
responsable de sus efectos; no se impondrá exclusividad de préstamos al estilo
de Rust. Esto no elimina las comprobaciones de vida útil acordadas. Una variable
de referencia mutable podrá cambiar de destino mediante otra referencia
compatible, comprobando de nuevo la vida útil.

`T*` no admitirá `null`; `T*?` será una referencia anulable que requerirá una
comprobación antes de desreferenciarse. No se podrán usar referencias para
modificar constantes ni sus campos.

Copiar un struct copiará sus campos. Un campo de referencia copiará la referencia
y compartirá el dato apuntado, sin hacer una copia profunda. Los strings conservarán
su contenido inmutable. No habrá referencias de solo lectura: las constantes
se pasarán por copia. El algoritmo de comprobación de vida útil y la estrategia
interna de administración de memoria quedan pendientes de implementación.

## Namespaces y visibilidad

El lenguaje usará namespaces para organizar nombres y reducir la necesidad de
importaciones. Se declararán con `namespace nombre { ... }`.

La búsqueda de nombres simples priorizará el ámbito local, después el namespace
actual y después los namespaces incorporados con `use`. Si hay varias
coincidencias en ese último nivel, será un error de compilación. Las rutas con
`::` partirán siempre de la raíz.

Habrá un único espacio de nombres por ámbito: se rechazarán colisiones entre
tipos, funciones, globales y namespaces hermanos. `input`, `print`, `println`,
`to_string` y `parse_i32` serán nombres incorporados reservados; no se podrán
redefinir ni ocultar.

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
No se fija una forma de importar un único miembro. También se admitirá
`namespace padre { namespace hijo { ... } }`, equivalente a la ruta
`namespace padre::hijo { ... }`.

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
Se rechazarán ciclos de tipos por valor, directos o indirectos. La recursión de
tipos solo podrá pasar por referencias, por ejemplo un atributo `Nodo*? siguiente`;
seguirán vigentes las reglas de vida útil.

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
        (*self).edad = (*self).edad + 1;
    }
}
```

Las funciones sin `self` se llamarán mediante el tipo. Se permitirán varios
bloques `impl` por tipo; repetir la definición de un método será un error de
compilación. Los bloques `impl` deberán estar en el namespace del struct, pero
podrán repartirse entre varios archivos. Un método con `self*` no podrá llamarse
sobre una constante ni un temporal: exigirá una variable o campo mutable con
vida útil suficiente. Los métodos por copia `self` podrán usar temporales.

El acceso mediante referencias siempre será explícito: `(*self).campo` y
`(*referencia).campo`. No habrá desreferencia automática para campos ni un
operador `->`.

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

## Entrada, salida y diagnóstico

La función incorporada `input()` leerá una línea de la entrada estándar y
no recibirá parámetros. En caso de éxito devolverá texto ASCII sin el terminador
de línea; una línea vacía devolverá `""`. En caso de fallo devolverá `null`.
Fin de entrada, fallos de lectura o caracteres fuera de ASCII devolverán `null`,
sin entregar texto parcial.

`print` y `println` aceptarán un argumento `i32`, `bool` o `string`. Los booleanos
se mostrarán como `true` o `false`. Los valores anulables requerirán una
comprobación previa. `print` y `println` serán funciones `void`: no devolverán
un valor, tampoco `null`. Esta última aclaración sustituye el comentario anterior
que proponía retornar `null` en fallo.

Si falla la escritura, se detendrá el programa con un error de ejecución
controlado y un estado de salida distinto de cero. Se intentará informar la
causa por la salida de errores estándar; si ese canal también falla, se
conservará el estado de fallo sin intentar imprimir errores indefinidamente.
No se devolverá un valor ni se lanzará una excepción recuperable en el lenguaje.
La salida que ya se haya escrito antes del fallo no se revertirá.

`to_string` aceptará `i32`, `bool` y `null`. Las funciones incorporadas con varios
tipos admitidos serán una excepción a la ausencia de sobrecarga definida por
el usuario.

`i32? parse_i32(string texto)` leerá decimal con signo opcional, sin espacios.
Devolverá `null` ante formato inválido o valor fuera del rango de `i32`; no
aplicará reducción modular. La sintaxis interna para producir el valor presente
respetará la asignación permitida de `T` a `T?`.

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

Los diagnósticos de compilación incluirán archivo, línea y columna contando
desde uno, además de la causa. Los de ejecución incluirán función y ubicación
fuente cuando esté disponible. No se fija todavía el texto literal de los mensajes.

## Relación con el bytecode

Por ahora se diseñará la sintaxis fuente y sus reglas sin fijar cómo se traducen
a instrucciones de pila. La documentación de bytecode contiene propuestas que
se revisarán más adelante y no limitan las características del lenguaje fuente.

## Límites de la primera versión

No habrá arrays, genéricos, herencia, excepciones, administración manual de
memoria ni aritmética de punteros. Tampoco operaciones de bits, ternario,
`switch`, `do`/`while` ni `goto`.

## Trabajo pendiente

Las [respuestas del formulario](respuestas-diseno.json) se incorporaron a este
documento junto con las aclaraciones posteriores del usuario. El
[formulario original](formulario-decisiones.html) y su
[inventario](decisiones-pendientes.md) conservan el contexto de las alternativas;
no son reglas vigentes ni nuevas solicitudes de aprobación.

- Formalizar formatos de literales, palabras reservadas y gramática completa.
- Diseñar análisis de inicialización parcial y contratos entre funciones,
  alcanzabilidad, vida útil y administración interna de memoria.
- Preparar programas completos y sus resultados esperados.
- Abordar la traducción a bytecode después de cerrar el lenguaje fuente.

Consultar la [arquitectura](arquitectura.md) para las responsabilidades de cada etapa.
