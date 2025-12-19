# Palindrome

## Descripción del Problema

Dada una cadena `S` de letras mayúsculas se desea formar una subsecuencia palíndroma que cubra la mayor cantidad posible de posiciones especiales predefinidas. Entre todas las subsecuencias palíndromas que cubran el máximo número de posiciones especiales, se debe devolver la de mayor longitud y reportar su tamaño.

## Entrada

Dos líneas:

1. Una cadena `S` (1 ≤ |S| ≤ 2000) formada por letras mayúsculas.
2. Una lista formada por un entero `N` (0 ≤ N ≤ |S|) seguido de `N` posiciones distintas entre 1 y |S| que indican los índices especiales.

## Salida

Un entero que representa la longitud de la subsecuencia palíndroma que cubre la máxima cantidad de posiciones especiales y, en caso de empate, es la más larga posible.

## Ejemplo de Entrada
```
BANANAS
0
```

## Ejemplo de Salida
```
5
```

## Análisis y Enfoque de Solución

Sea `special[i]` un indicador de si la posición `i` (0-indexada) es especial. Se define una programación dinámica sobre segmentos `S[i..j]` donde cada estado almacena dos valores: el número máximo de posiciones especiales incluidas en una subsecuencia palíndroma del segmento y, en caso de empate, la mayor longitud lograda.

Las transiciones son:

- Para segmentos de longitud 1, el valor es `(special[i], 1)`.
- Para segmentos mayores, se combinan las soluciones de los subsegmentos `S[i+1..j]` y `S[i..j-1]` y, si `S[i] == S[j]`, se considera también tomar ambos extremos junto con la mejor solución del segmento interno `S[i+1..j-1]`.
- La comparación entre estados prioriza el número de posiciones especiales y, en caso de empate, la longitud.

La respuesta final es el segundo componente del estado `dp[0][|S|-1]`.

### Complejidad

La tabla DP tiene tamaño `O(n²)` y cada estado se calcula en tiempo constante, por lo que el algoritmo opera en `O(n²)` con `n ≤ 2000`, perfectamente viable dentro de los límites.
