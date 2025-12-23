# Libertadores

## Descripción del Problema

En eliminatorias de ida y vuelta se determinan los clasificados por puntaje acumulado (3 puntos por victoria, 1 por empate). Si hay empate en puntos se aplican, en orden, diferencia de goles, goles marcados como visitante y finalmente penales. Dados los marcadores de ambos partidos, se debe determinar el ganador o indicar que se define por penales.

## Entrada

La primera línea contiene un entero `N` (1 ≤ N ≤ 100) con el número de llaves. Cada llave se describe con dos líneas, una por partido, en el formato `M x V`, donde `M` y `V` (1 ≤ M, V ≤ 100) son los goles del local y visitante respectivamente. En el primer encuentro el Equipo 1 juega de local y el Equipo 2 de visitante; en el segundo partido ocurre al revés.

## Salida

Para cada caso, imprimir una línea con `Time 1`, `Time 2` o `Penaltis`, según el equipo clasificado tras aplicar los criterios.

## Ejemplo de Entrada
```
4
1 x 1
2 x 1
2 x 0
2 x 1
1 x 1
2 x 2
3 x 1
3 x 1
```

## Ejemplo de Salida
```
Time 2
Time 1
Time 1
Penaltis
```

## Análisis y Enfoque de Solución

Se evalúan los criterios en cascada:

1. Calcular puntaje de cada equipo sumando los resultados de ambos partidos.
2. Si hay empate en puntos, comparar la diferencia de goles `(goles a favor − goles en contra)`.
3. En caso de igualdad, comparar los goles anotados como visitante (Equipo 1 en el segundo partido, Equipo 2 en el primero).
4. Si persiste el empate, la llave se decide por penales.

Todo puede resolverse con acumuladores simples tras leer los cuatro valores de cada llave.

### Complejidad

Cada caso se resuelve en tiempo y espacio constantes `O(1)`.
