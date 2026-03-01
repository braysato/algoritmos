# Elementary, my Dear Watson! - 1382

## Problema

Watson, Crick y Wilkins recibieron el Premio Nobel de Medicina de 1962 especialmente por su trabajo que resultó en el descubrimiento de la estructura de las moléculas de ADN y su importancia en la transmisión de información entre generaciones de seres vivos.

Muchos biólogos creen en el principio de parsimonia, que dice que el número de mutaciones debe ser el mínimo posible, ya que la Naturaleza busca, de alguna manera, el camino "más barato" para la modificación deseada.

Tu tarea en este problema es asistir a los investigadores en determinar si dos secuencias de ADN pueden tener un ancestro común. Considera dos secuencias dadas (podemos imaginarlas como secuencias de enteros). Tu objetivo es determinar el menor número de intercambios de elementos de una secuencia (los elementos no necesitan estar en posiciones adyacentes) que transforma una secuencia en la otra. Nota que podemos considerar una secuencia como fija (ej., en orden ascendente), así buscamos el número mínimo de intercambios que ordena la secuencia dada.

### Entrada

La entrada está compuesta de varios casos de prueba. La primera línea contiene un entero T indicando el número de casos. La primera línea de cada instancia tiene un entero N (1 ≤ N ≤ 10000) indicando el número de enteros en la secuencia. La segunda línea contiene una permutación de enteros 1, 2, ..., N separados por un espacio.

### Salida

Para cada instancia imprimir una línea conteniendo el número mínimo de intercambios que ordena la secuencia dada.

### Ejemplo de Entrada
```
2
5
2 3 4 5 1
5
2 1 4 5 3
```

### Ejemplo de Salida
```
4
3
```

## Análisis

1. **Ciclos en permutaciones**: Una permutación se puede descomponer en ciclos disjuntos.
2. **Swaps por ciclo**: Para ordenar un ciclo de longitud k, necesitamos exactamente k-1 swaps.
3. **Fórmula**: Número mínimo de swaps = N - (número de ciclos)

**Ejemplo [2, 3, 4, 5, 1]**:
- Ciclo único: 1→2→3→4→5→1 (longitud 5)
- Swaps = 5 - 1 = 4 ✓

**Ejemplo [2, 1, 4, 5, 3]**:
- Ciclo 1: 1→2→1 (longitud 2)
- Ciclo 2: 3→4→5→3 (longitud 3)
- Swaps = 5 - 2 = 3 ✓

## Complejidad

- Tiempo: O(N) para encontrar todos los ciclos
- Espacio: O(N) para el arreglo de visitados
