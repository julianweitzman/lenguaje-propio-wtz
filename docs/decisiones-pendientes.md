# Revisión de decisiones de WTZ

> **Respuestas incorporadas:** el usuario respondió las 42 cuestiones y aclaró
> algunos comentarios después. Las reglas actuales están en
> [sintaxis-codigo-fuente.md](sintaxis-codigo-fuente.md). Las selecciones originales
> y las aclaraciones están en [respuestas-diseno.json](respuestas-diseno.json).
> Las alternativas de este inventario son históricas, no preguntas aún sin responder.

## Detalles que todavía necesitan precisión

- Identidad de structs después de copiar, pasar por valor o retornar.
- Refinamiento de anulables tras llamadas que pueden mutarlos, con error runtime.
- Canal de error de salida: print/println son void y no retornan null.
- Prefijos de literales y posiciones de separadores para la gramática.

El análisis de inicialización parcial, alcanzabilidad y vida útil se implementará
conservando las reglas elegidas; no se presume un algoritmo completo ya diseñado.

## Alternativas originales del formulario


Revisión de `docs/sintaxis-codigo-fuente.md` en el commit `5f5cdd6` (2026-11-27).

El [formulario HTML](formulario-decisiones.html) contiene alternativas, recomendaciones y exportación de respuestas. Ninguna opción se considera aprobada por aparecer aquí; no hay respuestas preseleccionadas. Las decisiones existentes permanecen vigentes.

Las preguntas sobre seguridad de referencias y evaluación constante precisan interacciones aún ambiguas; una alternativa que cambie una regla vigente se identifica expresamente.

## Comparaciones y tipos

### eq_string: ¿Cómo se comparan dos strings?

Falta definir == y !=; el texto inmutable no determina por sí solo la igualdad.

`"Ana" == "Ana"`

- **Por contenido:** Recomendado: comparar los caracteres, no dónde se guardan.
- **Sin igualdad de strings:** Rechazar == y != entre strings en la primera versión.

### order_string: ¿Se pueden ordenar strings?

La aritmética y la igualdad no definen <, <=, > y >= para texto.

`"Ana" < "Beto"`

- **No por ahora:** Recomendado: reservar el orden a i32.
- **Orden ASCII:** Comparación lexicográfica por códigos ASCII.

### eq_struct: ¿Qué significa comparar structs?

La copia por campos ya está acordada; la igualdad de structs aún no.

`persona == otra`

- **No permitido:** Recomendado: comparar campos explícitamente; evita igualdad recursiva implícita.
- **Igualdad por campos:** Mismo tipo y comparación de cada campo; referencias por identidad. Todos los campos deben estar inicializados.

### eq_ref: ¿Cómo se comparan referencias?

Comparar la referencia y comparar el dato son operaciones diferentes.

`referencia == otra`

- **Identidad:** Recomendado: verdadero si apuntan al mismo dato y tienen tipos compatibles.
- **Sin igualdad de referencias:** Solo admitir comparación contra null para referencias anulables.

### bool_ops: ¿Qué comparaciones admite bool?

Ya hay lógica booleana y condiciones bool; falta restringir comparaciones.

`true == false; true < false;`

- **Solo igualdad:** Recomendado: == y !=; rechazar orden y aritmética.
- **Igualdad y orden:** false menor que true; seguir sin aritmética booleana.

## Anulables

### nullable_assign: ¿Se puede asignar T directamente a T??

No es una conversión entre tipos base distintos, sino introducir un valor presente.

`i32? numero = 10;`

- **Sí, envoltura automática:** Recomendado: permitir T a T?, nunca T? a T sin comprobar.
- **Exigir forma explícita:** Habrá que diseñar la sintaxis para un valor presente.

### nullable_eq: ¿Cómo funciona == entre dos anulables?

La comprobación contra null está acordada, pero no la comparación entre dos T?.

`i32? a = null; i32? b = null; a == b;`

- **Comparar ausencia o contenido:** Recomendado: dos null son iguales; un null y un valor no; dos presentes usan igualdad del tipo base.
- **Solo comparar con null:** Exigir comprobar ambos antes de comparar sus contenidos.

### nullable_flow: ¿Hasta dónde se reconoce una comprobación de null?

El documento solo ejemplifica una rama if. Falta concretar el resto del flujo.

`if (x != null && x > 0) { ... }`

- **Ramas y cortocircuito:** Recomendado: if/else, while, &&, || y salida temprana; invalidar la prueba cuando cambia el valor o puede alterarlo una llamada.
- **Solo ramas if:** Comprobación más limitada: usar bloques explícitos. Invalidar igualmente ante cambios o llamadas que puedan mutar.

### nullable_ref: ¿Admitimos referencias a valores anulables?

T*? ya significa referencia que puede ser null. T?* sería una referencia a un dato anulable.

`i32?* dato; i32*? referencia;`

- **Sí, tipos diferentes:** Recomendado: ? y * se aplican de izquierda a derecha. No permitir ??; sí referencias a referencias.
- **Solo una capa:** Limitar la primera versión a T?, T* y T*? con T base o struct; excluir T?* y referencias a referencias.

### nullable_invalidation: ¿Qué pasa si una llamada puede cambiar un anulable comprobado?

Una referencia compartida puede cambiar un valor después de la comprobación.

`if (dato != null) { cambiar(&dato); print(dato); }`

- **Recomprobar tras posible cambio:** Recomendado: rechazar el uso como no nulo hasta realizar otra comprobación.
- **Usar copia local comprobada:** Exigir copiar a una variable no anulable antes de la llamada; no conservar refinamientos de datos referenciables.

## Referencias y vida útil

### ref_alias: ¿Puede haber varias referencias mutables al mismo dato?

Copiar un campo de referencia ya comparte el dato. Falta decidir si se acepta la coexistencia.

`i32* a = &valor; i32* b = &valor;`

- **Sí, acceso secuencial:** Recomendado: permitir alias; sin concurrencia en esta versión. Se mantiene la vida útil segura.
- **Solo una activa:** Restricción de exclusividad; copiar una referencia requerirá limitar los usos simultáneos. Cambia la utilidad de las copias ya acordadas.

### ref_rebind: ¿Una variable de referencia puede cambiar de destino?

Modificar el destino (*r) y cambiar a dónde apunta (r) son cosas distintas.

`r = &otro; *r = 20;`

- **Sí:** Recomendado: la referencia es reasignable si su declaración no es const; volver a comprobar vida útil.
- **No:** El destino se fija al inicializar la referencia; solo se modifica el dato apuntado.

### ref_escape: ¿Permitimos que una referencia sobreviva al bloque de su creación?

No puede sobrevivir al dato; falta precisar cómo comprobar retornos de structs que contienen referencias.

`struct Vista { i32* dato; }`

- **Análisis estático de origen:** Recomendado: permitir solo si el origen vive lo suficiente; rechazar casos no demostrables. Un struct retornado no puede contener referencias.
- **Referencias dentro de la misma función:** Más restrictivo: structs con referencias no se retornan y las referencias solo se guardan/usan en la función o llamadas que no las retengan.

### ref_const_read: ¿Se necesita referencia de solo lectura?

Ya se prohibió obtener referencias mutables a const; no existe una notación para solo lectura.

`const Persona p = Persona { ... };`

- **No en esta versión:** Recomendado: pasar constantes por copia; mantener la sintaxis actual.
- **Añadir referencia de lectura:** Reabrir el sistema de tipos para definir una notación distinta y su conversión desde referencias mutables.

### self_fields: ¿Cómo accedemos a campos con self* y T*?

El ejemplo actual usa self.edad aunque las referencias normales se desreferencian con *.

`self.edad; referencia.edad;`

- **Punto automático para ambos:** Recomendado: r.campo y r.metodo() desreferencian automáticamente si r es T*; no existe ->.
- **Automático solo en self:** self.campo es especial; para otras referencias exigir (*r).campo.
- **Siempre explícito:** Usar (*self).campo y (*r).campo; corregir el ejemplo actual.

## Inicialización y constantes

### partial_copy: ¿Se pueden copiar structs incompletos?

Se permiten campos omitidos, pero antes de copiar se exige inicializar lo que pueda leerse. Falta un límite preciso.

`Persona a = Persona { edad: 20 }; Persona b = a;`

- **Solo structs completos:** Recomendado: copiar, pasar por valor, retornar o tomar referencia al struct exige todos sus campos inicializados. Sí se puede leer o referenciar un campo inicializado aislado.
- **Copiar estado parcial:** Propagar qué campos están inicializados entre asignaciones y funciones. Exige contratos adicionales y revisión de la prohibición de parámetros de salida.

### init_loops: ¿Qué análisis de inicialización aceptamos?

El compilador debe rechazar lecturas no garantizadas, aunque algunos casos complejos sí sean seguros.

`i32 x; while (condicion) { x = 1; } println(x);`

- **Conservador por flujo:** Recomendado: un while/for puede ejecutar cero veces; intersectar ramas; no demostrar propiedades aritméticas complejas.
- **Análisis más avanzado:** Intentar probar condiciones y cantidad de iteraciones; mayor complejidad del compilador, manteniendo rechazo si no se demuestra.

### const_partial: ¿Una constante struct puede tener campos omitidos?

const exige inicializador e impide modificar campos. Un inicializador parcial dejaría campos inaccesibles para siempre.

`const Persona p = Persona { edad: 20 };`

- **Exigir todos los campos:** Recomendado: constantes completas al declararse, también en tipos compuestos anidados.
- **Permitir campos ausentes:** Quedan permanentemente sin inicializar; leerlos siempre es error.

### const_eval_errors: ¿Qué ocurre con división por cero en una constante global?

Los inicializadores globales se evalúan al compilar, mientras que la división por cero normalmente falla al ejecutar.

`const i32 x = 10 / 0;`

- **Error de compilación:** Recomendado: fallos durante evaluación constante son diagnósticos de compilación; código normal conserva error de ejecución.
- **Error al iniciar:** Mover el diagnóstico a la carga/inicio; obliga a revisar que las globales se calculan al compilar.

### const_struct: ¿Se admiten structs y concatenación en constantes globales?

Faltan las operaciones que el evaluador constante debe soportar; llamadas a funciones ya están excluidas.

`const string s = "A" + "B";`

- **Sí, operaciones puras:** Recomendado: aritmética, lógica, comparación admitida, texto y literales de structs completos, sin referencias ni llamadas.
- **Solo literales sencillos:** Limitar globales a literales base y referencias a otras constantes; no concatenación ni structs globales.

## Nombres y archivos

### name_resolution: ¿Qué nombres tienen prioridad frente a use?

Ya hay error entre imports ambiguos y ocultamiento de variables exteriores. Falta combinarlos.

`use matematicas; i32 sumar(i32 a, i32 b) { ... }`

- **Local, namespace, use:** Recomendado: buscar primero ámbito local, luego namespace actual, después use; error si ese último nivel es ambiguo.
- **Toda colisión con use es error:** Rechazar el uso del nombre incluso si existe una declaración local o del namespace; usar ruta completa.

### path_resolution: ¿Desde dónde se resuelven las rutas con ::?

padre::hijo existe, pero no está definido si una ruta puede ser relativa al namespace actual.

`namespace a::b { c::funcion(); }`

- **Desde la raíz:** Recomendado: toda ruta con :: parte de la raíz; los nombres simples usan el contexto actual.
- **Contexto y después raíz:** Buscar relativo al namespace actual y luego desde raíz; definir error si existen ambas rutas.

### nested_decl: ¿Admitimos namespace dentro de otro bloque namespace?

La declaración namespace padre::hijo está aprobada; la forma anidada no está cerrada.

`namespace padre { namespace hijo { ... } }`

- **Sí, equivalente:** Recomendado: equivale a namespace padre::hijo. No hereda use de otros archivos.
- **Solo rutas explícitas:** Exigir namespace padre::hijo y no anidar bloques físicamente.

### name_spaces: ¿Tipos y funciones comparten espacio de nombres?

Sin sobrecarga, todavía puede aparecer una función con el mismo nombre que un struct o namespace.

`struct Persona { ... } i32 Persona() { ... }`

- **Un único espacio por ámbito:** Recomendado: rechazar colisiones entre tipos, funciones, globales y namespaces hermanos.
- **Separar tipos y valores:** Permitir nombres iguales para tipos y valores; el parser deberá distinguirlos por contexto.

### builtin_names: ¿Se pueden ocultar input, print, println y to_string?

Son funciones incorporadas, pero aún no hay namespace o regla de resolución para ellas.

`void print(i32 numero) { ... }`

- **Reservar nombres incorporados:** Recomendado: error al redeclararlos; sin namespace estándar adicional.
- **Permitir ocultamiento:** Tratar incorporadas como último nivel de búsqueda y definir cómo acceder a la versión original.

## Structs y funciones

### struct_recursion: ¿Qué structs recursivos son válidos?

Un campo del mismo tipo por valor no tiene tamaño finito; una referencia sí.

`struct Nodo { Nodo siguiente; }`

- **Solo mediante referencias:** Recomendado: rechazar ciclos por valor directos e indirectos; admitir Nodo*? y verificar vida útil.
- **Sin structs recursivos aún:** Rechazar también campos de referencia que formen ciclos de tipos en esta versión.

### self_temporary: ¿Puede llamarse un método self* sobre un temporal?

Las referencias a temporales están prohibidas, pero los receptores tienen sintaxis automática.

`Persona::new("Ana", 20).cumplir_anios();`

- **No:** Recomendado: self* exige variable o campo mutable con vida útil suficiente; self por copia sí puede usar temporales.
- **Sí, solo durante llamada:** Excepción temporal para el receptor, sin permitir que escape; necesita reglas especiales de préstamos.

### void_end: ¿Las funciones void necesitan return al final?

return; es válido y los retornos de funciones con valor están definidos; falta el final implícito de void.

`void saludar() { println("Hola"); }`

- **Retorno implícito:** Recomendado: llegar al final de void equivale a return;.
- **Siempre explícito:** Exigir return; en todos los caminos que puedan finalizar la función.

### infinite_return: ¿Un bucle infinito satisface una función con retorno?

Se exige retornar en todos los caminos que finalizan; un bucle infinito no finaliza.

`i32 esperar() { for (;;) {} }`

- **Sí, si es evidente:** Recomendado: aceptar bucles infinitos sintácticamente demostrables sin break alcanzable; no buscar pruebas complejas.
- **Exigir return escrito:** Rechazar una función no void sin retorno explícito aunque no pueda terminar.

### main_exit: ¿Qué significa el resultado de main?

return 0 indica éxito, pero falta cómo se comunica otro i32 al entorno.

`return 7;`

- **Estado de salida:** Recomendado: cero éxito; otros valores fallo. Documentar adaptación al rango del sistema operativo.
- **Resultado interno:** Conservar i32 como resultado de VM y mostrarlo únicamente cuando se solicite; definir por separado el estado de salida.

## Gramática y literales

### literals_range: ¿Cómo se tratan literales fuera del rango i32?

Overflow modular de operaciones no implica necesariamente truncar un literal escrito.

`i32 x = 2147483648; i32 min = -2147483648;`

- **Error salvo mínimo negativo:** Recomendado: rango estricto; aceptar -2147483648 con regla del signo unario. Las operaciones válidas siguen envolviendo.
- **Reducir módulo 2^32:** Todo literal decimal se reduce a 32 bits y se interpreta con signo; aceptar números arbitrariamente largos.

### literal_formats: ¿Qué formatos numéricos se permiten?

Los ejemplos solo usan decimal; no se acordaron prefijos ni separadores.

`1_000; 0xFF; 0b10;`

- **Solo decimal:** Recomendado: dígitos decimales, sin prefijos ni separadores en la primera versión.
- **Decimal con separador:** Admitir _ entre dígitos decimales; no hexadecimal ni binario.

### trailing_comma: ¿Admitimos coma final?

Afecta llamadas, parámetros y literales de structs.

`sumar(1, 2,); Persona { edad: 20, };`

- **Sí en listas:** Recomendado: permitir una coma final en argumentos, parámetros y campos, no elementos vacíos internos.
- **No:** Exigir que cada coma separe dos elementos.

### struct_semicolon: ¿La definición de struct termina con punto y coma?

El ejemplo actual no lo usa; conviene cerrar la gramática sin adoptar C automáticamente.

`struct Persona { i32 edad; }`

- **Sin punto y coma final:** Recomendado: conservar los ejemplos; ; solo en campos y sentencias.
- **Con punto y coma final:** Usar struct Persona { i32 edad; }; y ajustar ejemplos.

### decl_lists: ¿Una declaración puede crear varias variables?

C permite listas, pero no forman parte de lo aprobado.

`i32 a = 1, b = 2;`

- **Una por declaración:** Recomendado: i32 a = 1; i32 b = 2;.
- **Listas con mismo tipo:** Permitir comas; const debe inicializar cada elemento.

### expr_statement: ¿Se permite cualquier expresión como sentencia?

Una suma descartada y una llamada útil pueden tener la misma forma gramatical.

`10 + 20;`

- **Solo efectos explícitos:** Recomendado: llamadas, asignaciones, incrementos y decrementos.
- **Cualquier expresión:** Permitir descartar el resultado de una expresión mediante ;.

### source_line_end: ¿Qué finales de línea y espacios se aceptan?

ASCII está acordado, pero no los caracteres de control admitidos en el código.

`LF / CRLF / tabulaciones`

- **LF y CRLF:** Recomendado: espacios, tabulaciones y saltos LF o CRLF; rechazar BOM, CR aislado y controles ajenos fuera de strings.
- **También CR aislado:** Normalizar LF, CRLF y CR; mismas restricciones sobre controles.

## Texto y diagnóstico

### string_zero: ¿Qué significa el escape \0 dentro de un string?

Está aprobado, pero falta decidir si el string guarda longitud o termina en cero.

`"A\0B"`

- **Contenido con longitud:** Recomendado: contiene tres caracteres; \0 no termina el string.
- **Terminador de texto:** El cero termina el string; implica cambiar concatenación, igualdad y salida.

### bool_to_string: ¿to_string admite bool además de i32?

Solo se documentó to_string(i32); print ya acepta bool.

`to_string(true)`

- **Solo i32 por ahora:** Recomendado: mantener la API mínima acordada.
- **i32 y bool:** Devolver true o false; excepción incorporada a la regla de no sobrecarga, igual que print.

### parse_input: ¿Necesitamos convertir input a i32?

Leer devuelve texto. Sin conversión no se pueden hacer cálculos con datos introducidos.

`i32? numero = parse_i32(input_comprobado);`

- **Añadir parse_i32(string):** Recomendado: devuelve i32?; decimal con signo opcional, sin espacios; null ante formato inválido o fuera de rango, sin overflow modular de lectura.
- **Posponer:** No añadir conversión de entrada numérica en esta versión.

### output_failure: ¿Cómo se informa un fallo de print o println?

Solo input tiene fallo definido; todavía no hay excepciones ni recuperación general.

`print("Hola");`

- **Terminar con error controlado:** Recomendado: detener ejecución y emitir diagnóstico cuando el entorno permita informarlo.
- **Devolver bool:** true éxito, false fallo; revisar la firma de salida y permitir ignorar o comprobar su resultado.

### diagnostics: ¿Qué información mínima tienen los errores?

Es una decisión observable, distinta de elegir el algoritmo del compilador.

`archivo.wtz:12:8: ...`

- **Ubicación y causa:** Recomendado: archivo, línea y columna base 1 en compilación; función/ubicación fuente si está disponible al ejecutar. No fijar aún el texto literal.
- **Solo mensaje inicial:** Posponer ubicaciones de ejecución; mantener archivo y línea en compilación.

## Trabajo que no necesita otra elección del lenguaje

- Escribir la gramática y comprobarla con programas completos una vez cerradas las respuestas.
- Implementar los análisis de tipos, inicialización, anulabilidad y vida útil según las reglas elegidas.
- Elegir estructuras internas y administración automática de memoria sin cambiar el comportamiento acordado.
- Diseñar lexer y parser en su etapa; trasladar a bytecode más adelante.

El formulario sigue disponible como registro de alternativas. Las respuestas recibidas ya se incorporaron; una nueva exportación no cambia automáticamente las reglas vigentes.
