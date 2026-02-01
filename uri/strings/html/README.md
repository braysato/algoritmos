# HTML - 1667

## Problema

Si alguna vez intentaste leer un documento HTML en una Macintosh (Mac), sabes lo difícil que es si no hay un navegador Netscape instalado. Ahora, ¿Quién puede olvidarse instalar un navegador HTML? Es muy fácil porque la mayoría de las veces no lo necesitas en una Mac dado que hay un Acrobat Reader que ya viene con la Mac. Pero si alguna vez necesitas uno, ¿qué hacés? Tu tarea es escribir un pequeño navegador-HTML. Debería mostrar sólo el contenido del archivo de entrada y reconocer sólo los comandos (tags) de HTML, los cuales son el salto de linea `<br>` y linea horizontal `<hr>`. Después deberías tratar todas las tabulaciones, espacios y saltos de línea como un espacio y mostrar el texto resultante con no más de 80 caracteres en una línea.

### Entrada

La entrada consta de un texto que deberías mostrar. Este texto consta de palabras y comandos de HTML separados por uno o más espacios, tabulaciones o saltos de línea.
Una palabra es una secuencia de letras, números y signos de puntuación. Por ejemplo, "abc,123" es una palabra, pero "abc, 123" son dos palabras denominadas "abc," y "123". Una palabra siempre es más corta que 81 caracteres y no posee ningún '<' o '>'. Todos los comandos de HTML son o `<br>` o `<hr>`.

### Salida

Deberías mostrar el texto resultante usando las siguientes reglas:

- Si leés una palabra en la entrada y la línea resultante no se vuelve mayor a 80 caracteres, imprímela, si no, imprímela en una nueva línea.
- Si lees un `<br>` en la entrada, empieza una nueva linea.
- Si lees un `<hr>` en la entrada, empieza una nueva línea a menos que ya estés en el comienzo de una línea, en ese caso, mostrar 80 caracteres '-' y empezar una nueva línea (nuevamente).

La última línea termina con un carácter de salto de linea.

### Ejemplos de Entrada
```
Hallo, dies ist eine
ziemlich lange Zeile, die in Html
aber nicht umgebrochen wird.
<br>
Zwei <br> <br>produzieren zwei Newlines.
Es gibt auch noch das tag <hr> was einen Trenner darstellt.
Zwei <hr> <hr> produzieren zwei Horizontal Rulers.
Achtung mehrere Leerzeichen irritieren
Html genauso wenig wie
mehrere Leerzeilen.
```

### Ejemplos de Salida
```
Hallo, dies ist eine ziemlich lange Zeile, die in Html aber nicht umgebrochen
wird.
Zwei
produzieren zwei Newlines. Es gibt auch noch das tag
--------------------------------------------------------------------------------
was einen Trenner darstellt. Zwei
--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
produzieren zwei Horizontal Rulers. Achtung mehrere Leerzeichen irritieren Html
genauso wenig wie mehrere Leerzeilen.
```

## Análisis

1. **Tokenización**: Leer todo el input y separar por espacios/tabs/newlines en tokens
2. **Procesamiento de tokens**:
   - Si es `<br>`: salto de línea
   - Si es `<hr>`: si no estamos al inicio, salto de línea, luego 80 guiones, luego salto de línea
   - Si es palabra: si cabe en la línea actual (considerando espacio separador), agregarla; si no, nueva línea y luego la palabra
3. **Control de posición**: Mantener un contador de caracteres en la línea actual

## Complejidad

- Tiempo: O(n) donde n es el tamaño del input
- Espacio: O(n) para almacenar los tokens

## Corrección: Presentation Error

**Error inicial**: El código no generaba newline con `<br>` si ya estábamos al inicio de línea (pos == 0).

**Solución**: `<br>` debe **siempre** generar un salto de línea, incluso si estamos al inicio (puede crear líneas vacías). En cambio, `<hr>` solo salta línea si no estamos al inicio, luego imprime 80 guiones.

Diferencia clave:
- `<br>`: Siempre imprime newline
- `<hr>`: Solo imprime newline si pos > 0, luego los 80 guiones
