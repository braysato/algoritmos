# Guilds - 1527

## Enunciado

Rafael está jugando un nuevo y emocionante juego RPG, y acaba de notar la existencia de algo llamado Guild (Gremio). Un Guild es un grupo de jugadores que se juntan con un objetivo común dentro del juego, aprovechando el trabajo en equipo.

El juego tiene un sistema GVG (Guild versus Guild) muy disputado.

El sistema GVG funciona así: la batalla ocurre entre dos guilds, y gana el guild que tiene la mayor cantidad de puntos. El número de puntos de un guild es la suma de los puntos de los jugadores del guild. Cada jugador tiene puntos que corresponden a su nivel actual.

Inicialmente, todos los jugadores son parte de un guild que contiene solo al jugador mismo. La unión entre dos guilds hace que todos los jugadores de ambos guilds sean parte de un único guild, y el otro deja de existir.

Dada una lista de acciones (uniones entre guilds y batallas entre guilds), indicar el número de veces que el guild en el que está Rafael fue victorioso en una batalla.

### Entrada
Varios casos de prueba. Cada caso comienza con dos enteros N y M (1 ≤ N ≤ 10⁵, 1 ≤ M ≤ 5 * 10⁵), representando el número de jugadores y el número de acciones.

Luego N enteros Pi (1 ≤ Pi ≤ 100), donde el i-ésimo número representa los puntos del i-ésimo jugador. Rafael siempre es el jugador 1.

Luego M líneas con tres enteros Q, A y B (1 ≤ Q ≤ 2, 1 ≤ A, B ≤ N):
- Si Q = 1: el guild del jugador A y el guild del jugador B se unen
- Si Q = 2: el guild del jugador A y el guild del jugador B batallan

El último caso es cuando N = M = 0, que no debe procesarse.

### Salida
Para cada caso, imprimir un entero representando el número de batallas que el guild de Rafael ganó. Los empates no cuentan como victorias.

### Ejemplo de Entrada
```
5 5
5 3 4 3 2
1 1 2
1 3 4
2 2 4
1 4 5
2 1 3
0 0 
```

### Ejemplo de Salida
```
1
```

## Análisis

- Este es un problema clásico de **Union-Find (Disjoint Set Union)** con peso.
- Cada guild tiene una suma de puntos que debe mantenerse actualizada.
- Al unir dos guilds, se suman los puntos.
- Rafael es el jugador 1, así que debemos verificar si su guild participa en batallas.
- Solo contamos victorias (no empates) del guild de Rafael.

## Complejidad

- Tiempo: O(M * α(N)) donde α es la inversa de Ackermann (prácticamente constante)
- Espacio: O(N)
