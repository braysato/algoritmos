# 1141 - Growing Strings

## Problema

Gene y Gina tienen un tipo particular de granja. En lugar de cultivar animales y vegetales, como es usualmente el caso en granjas regulares, ellos cultivan strings. Un string es una secuencia de caracteres. Los strings tienen la particularidad de que, a medida que crecen, agregan caracteres a la izquierda y/o a la derecha de sí mismos, pero nunca pierden caracteres, ni insertan nuevos caracteres en el medio.

Gene y Gina tienen una colección de fotos de algunos strings en diferentes momentos durante su crecimiento. El problema es que la colección no está anotada, por lo que olvidaron a qué string pertenece cada foto. Quieren armar un muro para ilustrar los procedimientos de crecimiento de strings, pero necesitan tu ayuda para encontrar una secuencia apropiada de fotos.

Cada foto ilustra un string. La secuencia de fotos debe ser tal que si si viene inmediatamente antes de si+1 en la secuencia, entonces si+1 es un string que pudo haber crecido de si (es decir, si aparece como un substring consecutivo de si+1). Además, no quieren usar fotos repetidas, por lo que todos los strings en la secuencia deben ser diferentes.

Dado un conjunto de strings que representan todas las fotos disponibles, tu trabajo es calcular el tamaño de la secuencia más larga que pueden producir siguiendo las pautas anteriores.

**Input**
- Primera línea: entero N (1 ≤ N ≤ 10⁴) representando el número de strings
- Siguientes N líneas: strings diferentes no vacíos de como máximo 1000 letras minúsculas
- Suma de todas las longitudes de strings ≤ 10⁶
- Último caso de prueba seguido de una línea con 0

**Output**
Para cada caso de prueba imprimir un único entero representando el tamaño de la secuencia más larga de fotos.

**Ejemplo:**
```
Input:
6
plant
ant
cant
decant
deca
an

Output: 4
```
Secuencia: an -> ant -> cant -> decant

## Análisis

1. **Modelado del problema**: Encontrar la secuencia más larga de strings donde cada uno contiene al anterior como substring
   - Esto es equivalente a encontrar el camino más largo en un DAG implícito
   - Si ordenamos por longitud, solo necesitamos verificar strings más cortos como posibles predecesores
   
2. **Observaciones clave**:
   - Un string solo puede crecer de uno más corto, por lo tanto ordenar por longitud es crucial
   - Si un string A está contenido en B, entonces len(A) ≤ len(B)
   - Como cada string tiene máximo 1000 caracteres, si la diferencia de longitud > 1000, no puede ser substring
   
3. **Algoritmo optimizado**:
   - Ordenar strings por longitud ascendente
   - `dp[i]` = longitud de la secuencia más larga que termina en el string i
   - Para cada string i, verificar todos los strings j < i (más cortos)
   - Si string[i] contiene string[j], entonces `dp[i] = max(dp[i], dp[j] + 1)`
   - **Optimización crítica**: Si `len(string[i]) - len(string[j]) > 1000`, hacer break (no puede ser substring)
   - Retornar el máximo valor en dp

## Solución

**Estrategia:**
- Programación dinámica con ordenamiento por longitud
- Early stopping cuando la diferencia de longitud excede el máximo posible
- Ordenamiento directo de strings sin indirección para mejor performance

**Complejidad:**
- Ordenamiento: O(N log N)
- DP: O(N² × M) en el peor caso, donde M es la longitud promedio
- Con early stopping: mucho mejor en la práctica
- Espacial: O(N) para el array dp
