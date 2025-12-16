# Cuadrados Vacíos

## Descripción del Problema

Martín tiene un tablero de 1×N casillas y dispone de fichas de longitudes 1×1, 1×2, …, 1×N (una de cada). Ya hay colocada una ficha de longitud K, dejando E casillas libres a su izquierda. Nico quiere colocar algunas de las fichas restantes para cubrir la mayor cantidad posible de casillas libres. Se pide determinar cuántas casillas quedarán vacías cuando Nico actúe de forma óptima.

## Entrada

Una sola línea con tres enteros `N`, `K` y `E` (`1 ≤ N ≤ 1000`, `1 ≤ K ≤ N`, `0 ≤ E ≤ N − K`). El tablero tiene `N` casillas, la ficha ya colocada mide `K` casillas y deja `E` casillas libres a su izquierda.

## Salida

Un entero que indique el número de casillas que permanecerán vacías tras cubrir la máxima cantidad posible con las fichas restantes.

## Ejemplo de Entrada
```
6 2 2
```

## Ejemplo de Salida
```
3
```

## Análisis y Enfoque de Solución

La ficha fija divide al tablero en dos segmentos independientes: uno izquierdo de longitud `L = E` y otro derecho de longitud `R = N - K - E`. Quedan disponibles todas las fichas de longitudes `1..N`, salvo la de longitud `K`. Cada ficha puede usarse a lo sumo una vez y debe colocarse completamente dentro de alguno de los segmentos, sin solaparse con otras fichas.

El problema se reduce a asignar cada ficha al segmento izquierdo, al derecho o descartarla, maximizando la suma total de longitudes colocadas en ambos segmentos (con límites `L` y `R`). Esto puede resolverse con programación dinámica usando una tabla de `L + 1` estados, donde cada estado mantiene un bitset de longitudes alcanzables en el segmento derecho. Para cada ficha:

1. Se propaga el estado actual (opción de no usar la ficha).
2. Se intentan asignaciones al lado izquierdo actualizando las sumas posibles.
3. Se intentan asignaciones al lado derecho mediante desplazamientos del bitset.

Al finalizar, se busca la combinación `(l, r)` que maximiza `l + r`. Las casillas que permanecen vacías son `N - K - (l + r)`.

### Complejidad

Con `N ≤ 1000`, los segmentos tienen longitud máxima 1000. La DP recorre cada ficha actualizando `L + 1` estados con bitsets de tamaño `R + 1`, lo que implica un costo de `O(N · L · R / 64)` en la práctica, acotado y eficiente para los límites dados.
