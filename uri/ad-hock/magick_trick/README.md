# 1360 - Magick Trick

## Problema

Un mago inventó un nuevo truco de cartas y lo presentó en la prestigiosa American Conference of Magicians (ACM). El truco requiere tres participantes: el mago, un espectador y un asistente.

### Descripción del Truco

1. El espectador baraja un mazo de 52 cartas y elige aleatoriamente 5 cartas
2. Las 5 cartas se dan al asistente (sin que el mago las vea)
3. El asistente mira las cartas y muestra 4 de las 5 cartas al mago, una por una
4. Después de ver las 4 cartas, el mago adivina la quinta carta oculta

### Ordenamiento de Cartas

Las cartas se ordenan primero por palo y luego por valor:

**Palos:** H < C < D < S (Hearts, Clubs, Diamonds, Spades)
**Valores:** 1 < 2 < 3 < 4 < 5 < 6 < 7 < 8 < 9 < T < J < Q < K

Donde T, J, Q, K representan Ten (10), Jack, Queen y King respectivamente.

### Estrategia del Asistente

**1. Encontrar un palo que aparezca al menos dos veces**
   - Si más de un palo aparece dos veces, elegir el de menor orden
   - Por ejemplo: JD, 8S, 7H, 8C, QH → Hearts (H) aparece dos veces

**2. Ocultar la carta correcta**
   - Ocultar la carta `x` con palo `s` que esté como máximo 6 posiciones adelante en el orden cíclico de otra carta `y` del mismo palo
   - Orden cíclico: 1 < 2 < ... < T < J < Q < K < 1 < 2 < ...
   - Si dos o más cartas satisfacen el criterio, elegir la de menor valor facial
   - En el ejemplo: se oculta QH

**3. Mostrar la carta `y` primero**
   - El mago ahora sabe el palo de la carta oculta
   - También sabe que el valor facial está como máximo 6 posiciones adelante de `y`

**4. Codificar un número entre 1 y 6 con las 3 cartas restantes**
   - Sean z1 < z2 < z3 las tres cartas en orden
   - El orden en que se muestran codifica un número:
     - z1, z2, z3 → 1
     - z1, z3, z2 → 2
     - z2, z1, z3 → 3
     - z2, z3, z1 → 4
     - z3, z1, z2 → 5
     - z3, z2, z1 → 6

### Entrada

La entrada contiene varios casos de prueba. La primera línea contiene un entero N especificando el número de casos (1 ≤ N ≤ 10000).

Cada caso de prueba está compuesto por una línea que contiene la descripción de las 4 cartas, separadas por espacio, en el orden en que fueron presentadas por el asistente.

### Salida

Para cada caso de prueba, producir una línea con la descripción de la carta oculta.

### Ejemplo de Entrada
```
2
7H 8S 8C JD
TC 2D 1S 5H
```

### Ejemplo de Salida
```
QH
1C
```

## Análisis del Problema

### Comprensión
- Debemos decodificar la información que el asistente codificó en las 4 cartas mostradas
- La primera carta indica el **palo** de la carta oculta
- Las otras 3 cartas codifican un **número entre 1 y 6** que indica cuántas posiciones adelante está la carta oculta

### Observaciones Clave

1. **Primera carta:** Su palo es el mismo que la carta oculta
2. **Posición cíclica:** Desde el valor de la primera carta, avanzar 1-6 posiciones en el orden cíclico
3. **Tres cartas restantes:** Su orden de presentación codifica el número (1-6)
4. **Orden cíclico:** K → 1 (después de King viene Ace)

### Pasos para Resolver

**1. Identificar el palo de la carta oculta:**
   - Es el mismo palo que la primera carta mostrada

**2. Determinar el offset (1-6):**
   - Ordenar las 3 últimas cartas (z1 < z2 < z3)
   - Comparar el orden real con el orden esperado para obtener el número

**3. Calcular el valor de la carta oculta:**
   - Valor inicial = valor de la primera carta
   - Valor oculto = (valor_inicial + offset) en orden cíclico
   - Si pasa de K, vuelve a 1

### Ejemplo
**Entrada:** 7H 8S 8C JD
- Primera carta: 7H → Carta oculta tiene palo H (Hearts)
- Tres cartas restantes: 8S, 8C, JD
- Ordenadas: 8C < 8S < JD (z1 = 8C, z2 = 8S, z3 = JD)
- Orden real: 8S, 8C, JD → corresponde a z2, z1, z3 → número **3**
- Valor oculto: 7 + 3 = 10 = T... pero contando cíclicamente: 7→8→9→T→J→**Q**
- Respuesta: **QH**
