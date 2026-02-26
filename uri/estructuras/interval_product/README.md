# Interval Product - 1301

## Problema

Es normal sentirse preocupado y tenso el día antes de un concurso de programación. Para relajarte, saliste a tomar algo con amigos en un pub cercano. Para mantener tu mente aguda para el día siguiente, decidiste jugar el siguiente juego. Para empezar, tus amigos te darán una secuencia de N enteros X1, X2,..., XN. Luego, habrá K rondas; en cada ronda, tus amigos emitirán un comando, que puede ser:

- un comando de **cambio**, cuando tus amigos quieren cambiar uno de los valores en la secuencia; o
- un comando de **producto**, cuando tus amigos te dan dos valores I, J y te preguntan si el producto XI × XI+1 × ... × XJ-1 × XJ es positivo, negativo o cero.

### Entrada

Cada caso de prueba se describe usando varias líneas. La primera línea contiene dos enteros N y K, indicando respectivamente el número de elementos en la secuencia y el número de rondas del juego (1 ≤ N, K ≤ 10^5). La segunda línea contiene N enteros Xi que representan los valores iniciales de la secuencia (-100 ≤ Xi ≤ 100). Cada una de las siguientes K líneas describe un comando y comienza con una letra mayúscula que es 'C' o 'P'. Si la letra es 'C', la línea describe un comando de cambio, y la letra es seguida por dos enteros I y V indicando que XI debe recibir el valor V. Si la letra es 'P', la línea describe un comando de producto, y la letra es seguida por dos enteros I y J indicando que el producto desde XI hasta XJ, inclusive, debe ser calculado.

### Salida

Para cada caso de prueba imprimir una línea con una cadena representando el resultado de todos los comandos de producto en el caso de prueba. El i-ésimo carácter de la cadena representa el resultado del i-ésimo comando de producto. Si el resultado es positivo el carácter debe ser '+'; si es negativo debe ser '-'; si es cero debe ser '0'.

### Ejemplo de Entrada
```
4 6
-2 6 0 -1
C 1 10
P 1 4
C 3 7
P 2 2
C 4 -5
P 1 4
5 9
1 5 -2 4 3
P 1 2
P 1 5
C 4 -5
P 1 5
P 4 5
C 3 0
P 1 5
C 4 -5
C 4 -5
```

### Ejemplo de Salida
```
0+-
+-+-0
```

## Análisis

1. **Optimización**: No necesitamos el producto real (puede desbordarse), solo el signo.
2. **Representación**: Guardamos solo el signo de cada número: -1 (negativo), 0 (cero), 1 (positivo).
3. **Segment Tree**: Usamos un árbol de segmentos donde cada nodo almacena el producto de signos de su rango.
4. **Operaciones**: 
   - Update: O(log N)
   - Query: O(log N)

## Complejidad

- Tiempo: O((N + K) * log N)
- Espacio: O(N) para el Segment Tree
