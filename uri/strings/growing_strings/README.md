# 1141 - Growing Strings

## Problema

Gene and Gina have a particular kind of farm. Instead of growing animals and vegetables, as it is usually the case in regular farms, they grow strings. A string is a sequence of characters. Strings have the particularity that, as they grow, they add characters to the left and/or to the right of themselves, but they never lose characters, nor insert new characters in the middle.

Gene and Gina have a collection of photos of some strings at different times during their growth. The problem is that the collection is not annotated, so they forgot to which string each photo belongs to. They want to put together a wall to illustrate strings growing procedures, but they need your help to find an appropriate sequence of photos.

Each photo illustrates a string. The sequence of photos must be such that if si comes immediately before si+1 in the sequence, then si+1 is a string that may have grown from si (i.e., si appears as a consecutive substring of si+1). Also, they do not want to use repeated pictures, so all strings in the sequence must be different.

Given a set of strings representing all available photos, your job is to calculate the size of the largest sequence they can produce following the guidelines above.

**Input**
- First line: integer N (1 ≤ N ≤ 10⁴) representing the number of strings
- Next N lines: different non-empty strings of at most 1000 lowercase letters
- Sum of all string lengths ≤ 10⁶
- Last test case followed by a line with 0

**Output**
For each test case print a single integer representing the size of the largest sequence of photos.

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
