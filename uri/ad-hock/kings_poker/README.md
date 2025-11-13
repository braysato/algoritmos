# Kings Poker

## Descripción del Problema

El Poker es uno de los juegos de cartas más jugados, y King's Poker es una de sus variaciones. El juego se juega con una baraja normal de 52 cartas. Cada carta tiene uno de 4 palos y uno de 13 rangos. Sin embargo, en King's Poker los palos de las cartas no son relevantes, mientras que los rangos son As (rango 1), 2, 3, 4, 5, 6, 7, 8, 9, 10, Jack (rango 11), Queen (rango 12) y King (rango 13). El nombre del juego proviene del hecho de que en King's Poker, el Rey es la carta de mayor rango. Pero esta no es la única diferencia entre el Poker regular y King's Poker. Los jugadores de King's Poker reciben una mano de solo tres cartas. Hay tres tipos de manos:

- **Un set (trío)**: formado por tres cartas del mismo rango.
- **Un par**: que contiene dos cartas del mismo rango, con la otra carta sin emparejar.
- **Sin par**: donde ninguna de las dos cartas tiene el mismo rango.

Las manos se clasifican usando las siguientes reglas:

1. Cualquier set derrota a cualquier par y a cualquier mano sin par.
2. Cualquier par derrota a cualquier mano sin par.
3. Un set formado con cartas de mayor rango derrota a cualquier set formado con cartas de menor rango.
4. Si las cartas emparejadas de dos pares tienen rangos diferentes, entonces el par con las cartas emparejadas de mayor rango derrota al par con las cartas emparejadas de menor rango.
5. Si las cartas emparejadas de dos pares tienen el mismo rango, entonces se comparan las cartas no emparejadas; el par con la carta no emparejada de mayor rango derrota al par con la carta no emparejada de menor rango, a menos que ambas cartas no emparejadas tengan el mismo rango, en cuyo caso hay un empate.

Una nueva empresa de software quiere ofrecer juegos de King's Poker en su sitio de juegos en línea y necesita un software que, dada una mano de King's Poker, determine el set o par con el rango más bajo que gane a la mano dada. ¿Puedes codificarlo?

## Entrada

Cada caso de prueba se describe usando una sola línea. La línea contiene tres enteros A, B y C que representan los rangos de las cartas repartidas en una mano (1 ≤ A, B, C ≤ 13).

El último caso de prueba es seguido por una línea que contiene tres ceros.

## Salida

Para cada caso de prueba, imprima una sola línea. Si existe un set o un par que gane a la mano dada, escriba la mano de menor rango que la gane. La mano ganadora debe escribirse especificando los rangos de sus cartas, en orden no decreciente. Si ningún set o par gana a la mano dada, escriba el carácter '*' (asterisco).

## Ejemplo de Entrada
```
1 1 1
1 1 12
1 1 13
1 13 1
10 13 10
1 2 2
13 13 13
13 12 13
12 12 12
3 1 4
1 5 9
0 0 0
```

## Ejemplo de Salida
```
2 2 2
1 1 13
1 2 2
1 2 2
1 11 11
2 2 3
*
1 1 1
13 13 13
1 1 2
1 1 2
```

## Análisis y Enfoque de Solución

Este es un problema de **clasificación y comparación de manos** de poker simplificado donde debemos encontrar la mano ganadora MÁS BAJA (set o par únicamente).

### Clasificación de Manos

Jerarquía de manos (de mayor a menor):
1. **Set (trío)**: Tres cartas del mismo rango
2. **Par**: Dos cartas del mismo rango, una diferente
3. **Sin par**: Tres cartas de rangos diferentes

### Reglas de Comparación

- **Set vs Set**: Gana el de mayor rango
- **Par vs Par**: 
  - Primero se compara el rango del par (mayor gana)
  - Si el rango del par es igual, se compara la carta suelta (mayor gana)
- Cualquier set gana a cualquier par
- Cualquier par gana a cualquier sin par

### Estrategia para Encontrar la Mano Ganadora Mínima

**Caso 1: Set de rango R**
- Solo otro set puede ganar a un set
- La mano ganadora es el siguiente set de rango R+1
- Si R = 13, no existe set mayor → output: `*`

**Caso 2: Par de rango R con carta suelta S**
- Tanto un set como un par mejor pueden ganar
- Estrategia: intentar mejorar el mismo par incrementando la carta suelta
  1. Calcular nextCard = S + 1
  2. Si nextCard == R (coincide con el par), usar nextCard = S + 2
  3. Si nextCard ≤ 13: retornar par `R R nextCard` ordenado
  4. Si nextCard > 13 y R < 13: retornar siguiente par `1 (R+1) (R+1)`
  5. Si nextCard > 13 y R = 13: retornar set `1 1 1` (única opción que gana)

**Importante**: Cuando mejoramos la carta suelta, debemos evitar que nextCard sea igual al rango del par, ya que eso crearía un set en lugar de un par.

**Caso 3: Sin Par**
- Cualquier par o set gana
- El par más bajo es `1 1 2` (par de ases con carta 2)
- Nota: `1 1 1` es un set, no un par

### Implementación

1. Ordenar las tres cartas de entrada
2. Clasificar la mano (set, par, o sin par)
3. Aplicar la lógica correspondiente:
   - **Set**: retornar (R+1, R+1, R+1) o `*`
   - **Par**: calcular nextCard evitando conflicto con el par, ordenar y retornar
   - **Sin par**: retornar `1 1 2`

### Complejidad

- **Tiempo**: O(1) - operaciones constantes (ordenar 3 elementos es O(1))
- **Espacio**: O(1)
