# 1935 - Spiral

## Problema

Dado un tablero de dimensiones N × N, queremos colocar frijoles, un grano en cada cuadrado, siguiendo una espiral como se muestra en la figura. Comenzando desde la esquina superior izquierda, con coordenadas (1, 1), y luego yendo a la derecha tanto como sea posible, luego hacia abajo tanto como sea posible, luego a la izquierda tanto como sea posible y luego hacia arriba tanto como sea posible. Repetimos este patrón, derecha-abajo-izquierda-arriba, hasta que se coloquen B frijoles en el tablero.

El problema es: dados N y B, ¿en qué coordenadas se colocará el último grano de frijoles?

**Ejemplo:** Para N = 8 y B = 53, el último grano se colocó en las coordenadas (4, 6).

**Entrada:**
- Una línea con dos enteros N y B
- 1 ≤ N ≤ 2^30
- 1 ≤ B ≤ N^2

**Salida:**
- Una línea con dos enteros L y C representando las coordenadas del último grano de frijoles

**Ejemplos:**
```
Entrada: 8 53             → Salida: 4 6
Entrada: 1073741824 1152921504603393520 → Salida: 536871276 536869983
```

## Análisis del Problema

**Patrón de la espiral:**
1. Comenzamos en (1, 1) - esquina superior izquierda
2. Movemos en el patrón: Derecha → Abajo → Izquierda → Arriba
3. Cada "capa" de la espiral forma un marco rectangular

**Visualización para N = 8:**
```
1  2  3  4  5  6  7  8
24 25 26 27 28 29 30 9
23 40 41 42 43 44 31 10
22 39 52 53 54 45 32 11
21 38 51 56 55 46 33 12
20 37 50 49 48 47 34 13
19 36 35 34 33 32 31 14
18 17 16 15 14 13 12 15
```

**Observaciones clave:**
1. La espiral se forma en "capas" o "niveles"
2. Cada capa k (comenzando desde 0) ocupa el borde de un cuadrado
3. Los valores de N pueden ser muy grandes (hasta 2^30), no podemos simular
4. Necesitamos una fórmula matemática directa

**Estructura de capas:**
- Capa 0: borde externo del tablero N×N
- Capa 1: borde del tablero (N-2)×(N-2) interior
- Capa k: borde del tablero (N-2k)×(N-2k)

**Elementos en cada capa:**
- Capa k tiene un perímetro de: 4 × (N - 2k) - 4 elementos (excepto la última)
- Si solo queda un elemento central: 1 elemento

## Enfoque de Solución

**Estrategia:**
1. Determinar en qué capa está el frijol B
2. Determinar la posición dentro de esa capa
3. Calcular las coordenadas exactas basadas en el lado de la espiral

**Algoritmo:**
1. Para cada capa k, calcular cuántos frijoles caben en las capas anteriores
2. Encontrar la capa donde está B
3. Calcular el offset dentro de esa capa
4. Según el offset, determinar en qué lado de la espiral está (derecha, abajo, izquierda, arriba)
5. Calcular coordenadas finales

**Optimización:**
- Dado que N puede ser hasta 2^30, necesitamos trabajar con long long
- Calculamos directamente la capa sin iterar todas

## Complejidad

- **Tiempo:** O(min(N, √B)) para encontrar la capa correcta
- **Espacio:** O(1) - solo variables para cálculos

