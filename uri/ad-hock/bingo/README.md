# Bingo - Problema 1136 (Ad-Hoc)

## Enunciado del Problema

Albert, Charles y Mary inventaron una nueva versión del juego clásico de Bingo. En el Bingo tradicional, el juego es presidido por un no-jugador conocido como el locutor. Al comienzo del juego, cada jugador recibe una tarjeta que contiene una combinación única de números del 0 al N organizados en columnas y filas. El locutor tiene una bolsa que contiene N+1 pelotas, numeradas del 0 al N. En cada turno, el locutor selecciona aleatoriamente una pelota de la bolsa, anuncia el número de la pelota extraída a los jugadores, y pone la pelota a un lado para que no pueda ser seleccionada nuevamente. Cada jugador busca en su tarjeta el número llamado y lo marca si lo encuentra. El primer jugador que marque un patrón completo preestablecido en la tarjeta (por ejemplo, una línea horizontal completa) gana un premio.

En la versión de Albert-Charles-Mary, en cada turno, el locutor extrae una primera pelota, la devuelve a la bolsa, extrae una segunda pelota, la devuelve a la bolsa, y luego anuncia la diferencia absoluta entre los dos números de las pelotas. Para generar aún más emoción, antes de que comenzara el juego se eliminó un subconjunto posiblemente vacío de pelotas de la bolsa, de tal manera que al menos dos pelotas permanecen allí. Ellos quieren saber si cada número del 0 al N todavía puede ser anunciado con el nuevo método de extracción considerando las pelotas que quedaron en la bolsa.

## Entrada

Cada caso de prueba se da usando exactamente dos líneas. La primera línea contiene dos enteros N y B. El significado de N fue descrito arriba (1 ≤ N ≤ 90), mientras que B representa el número de pelotas que permanecieron en la bolsa (2 ≤ B ≤ N+1). La segunda línea contiene B enteros distintos bi, indicando las pelotas que permanecieron en la bolsa (0 ≤ bi ≤ N).

El último caso de prueba es seguido por una línea que contiene dos ceros.

## Salida

Para cada caso de prueba, imprima una sola línea que contenga una sola 'Y' mayúscula si es posible anunciar cada número del 0 al N, inclusive, o una sola 'N' mayúscula en caso contrario.

## Ejemplos

### Entrada
```
6 7
2 1 3 4 0 6 5
5 4
5 3 0 1
5 3
1 5 0
0 0
```

### Salida
```
Y
Y
N
```

## Análisis del Problema

### Comprensión del Problema
1. **Mecánica del juego**: En esta versión de Bingo, el locutor saca dos pelotas, las devuelve a la bolsa, y anuncia la **diferencia absoluta** entre los dos números.
2. **Objetivo**: Determinar si es posible generar todos los números del 0 al N usando las diferencias absolutas de las pelotas que quedaron en la bolsa.

### Enfoque de Solución
1. **Generación de diferencias**: Para cada par de pelotas (bi, bj) en la bolsa, calcular |bi - bj|.
2. **Verificación de cobertura**: Verificar si todas las diferencias posibles (0 a N) pueden ser generadas.

### Observaciones Clave
- El 0 siempre se puede generar tomando la misma pelota dos veces (|bi - bi| = 0).
- Necesitamos verificar si podemos generar todos los números del 1 al N.
- La diferencia máxima posible es la diferencia entre el mayor y menor número en la bolsa.

### Estrategia
1. Almacenar todas las pelotas disponibles en la bolsa.
2. Generar todas las posibles diferencias absolutas entre pares de pelotas.
3. Verificar si el conjunto de diferencias generadas incluye todos los números del 0 al N.

### Complejidad
- **Tiempo**: O(B²) para generar todas las diferencias + O(N) para verificar
- **Espacio**: O(N) para almacenar las diferencias posibles

### Casos de Prueba Explicados
1. **Caso 1**: N=6, pelotas=[2,1,3,4,0,6,5] → Se pueden generar todas las diferencias 0-6 → 'Y'
2. **Caso 2**: N=5, pelotas=[5,3,0,1] → Se pueden generar todas las diferencias 0-5 → 'Y'  
3. **Caso 3**: N=5, pelotas=[1,5,0] → No se pueden generar todas las diferencias 0-5 → 'N'

## Solución Implementada

### Algoritmo
1. **Lectura de datos**: Lee N y B, luego las B pelotas que quedaron en la bolsa
2. **Generación de diferencias**: Calcula todas las diferencias absolutas `|balls[i] - balls[j]|` para cada par de pelotas
3. **Almacenamiento eficiente**: Usa un `set<int>` para almacenar las diferencias únicas
4. **Verificación**: Comprueba si todos los números del 0 al N están en el conjunto de diferencias
5. **Salida**: Imprime 'Y' si es posible generar todos, 'N' en caso contrario

### Puntos Clave de la Implementación
- ✅ **Optimización de E/S**: `ios_base::sync_with_stdio(false)` y `cin.tie(nullptr)`
- ✅ **Diferencias con la misma pelota**: El bucle incluye `i == j`, generando diferencia 0
- ✅ **Búsqueda eficiente**: Uso de `set` para O(log n) en búsquedas
- ✅ **Manejo del caso terminal**: El bucle termina cuando N=0 y B=0

### Complejidad Final
- **Tiempo**: O(B² log B + N log B)
- **Espacio**: O(B²) en el peor caso para el set

### Validación de Casos de Ejemplo
- **Caso 1**: N=6, B=7, pelotas=[2,1,3,4,0,6,5] → 'Y' ✅
- **Caso 2**: N=5, B=4, pelotas=[5,3,0,1] → 'Y' ✅  
- **Caso 3**: N=5, B=3, pelotas=[1,5,0] → 'N' ✅