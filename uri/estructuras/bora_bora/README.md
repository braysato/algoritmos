# Bora Bora - 1439

## Problema

Bora Bora es un juego de cartas simple para niños. Dos o más jugadores pueden jugar usando una baraja estándar. Las cartas tienen los rangos usuales: As, 2-10, Jack, Queen y King. Cada carta tiene uno de cuatro palos: Clubs, Diamonds, Hearts y Spades.

Los jugadores se sientan en círculo y juegan por turnos. La dirección puede ser horaria o antihoraria dependiendo de las cartas jugadas. Al inicio, la dirección es horaria.

Se reparten M cartas a cada jugador. El resto del mazo es el stock. La primera carta del stock se coloca boca arriba formando el discard pile.

El objetivo es descartar todas las cartas. Una carta solo puede descartarse si tiene el mismo rango O el mismo palo que la carta superior del discard pile. Si no tienes carta válida, robas una del stock; si esa carta se puede descartar, la descartas. Siempre descartas la carta de mayor valor posible (primero por rango, luego por palo: C < D < H < S).

**Cartas especiales:**
- **Queen (12)**: Revierte la dirección de juego
- **Seven (7)**: El siguiente jugador roba 2 cartas y pierde su turno
- **Ace (1)**: El siguiente jugador roba 1 carta y pierde su turno
- **Jack (11)**: El siguiente jugador pierde su turno

### Entrada

La primera línea contiene P, M y N (jugadores, cartas por jugador, cartas totales). Las siguientes N líneas describen cada carta con rango (1-13) y palo (C/D/H/S). Las primeras P×M cartas se reparten, la siguiente va al discard pile, el resto es el stock.

### Salida

Para cada caso, imprimir el número del jugador ganador.

### Ejemplo de Entrada
```
2 2 10
1 D
7 D
1 S
3 C
13 D
1 S
5 H
12 D
7 S
2 C
0 0 0
```

### Ejemplo de Salida
```
1
```

## Análisis

1. **Simulación**: Simular el juego siguiendo todas las reglas.
2. **Selección de carta**: Encontrar la carta de mayor valor que coincida en rango o palo.
3. **Efectos especiales**: Manejar Queen, Seven, Ace, Jack.
4. **Dirección**: Mantener track de la dirección de juego.

## Complejidad

- Tiempo: O(N × M) en el peor caso
- Espacio: O(N) para almacenar las cartas
