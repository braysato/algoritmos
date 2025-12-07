# Jollo

## Descripción del Problema

Se reparten cinco cartas de una baraja de 52: tres para la princesa y dos para el príncipe. El sirviente puede elegir una tercera carta adicional para el príncipe. En cada ronda ambos revelan una carta y la más alta gana; las cartas usadas se descartan. El objetivo es encontrar la carta de menor valor que garantice que el príncipe no pierda la serie al mejor de tres sin importar el orden en que juegue sus cartas.

## Entrada

Cada caso es una línea con cinco enteros distintos `A B C X Y` (1 ≤ valores ≤ 52). Los tres primeros son cartas de la princesa y los dos últimos del príncipe. La entrada termina con `0 0 0 0 0`.

## Salida

Para cada caso se imprime un entero: la carta de menor valor que asegura que el príncipe no pierda sin importar cómo juegue. Si no existe, se imprime `-1`.

## Ejemplo de Entrada
```
28 51 29 50 52
50 26 19 10 27
10 20 30 24 26
46 48 49 47 50
0 0 0 0 0
```

## Ejemplo de Salida
```
30
-1
21
51
```

## Análisis y Enfoque de Solución

El príncipe necesita sumar al menos dos victorias en tres rondas aun si escoge sus cartas en el peor orden posible. Para comprobar si una carta candidata funciona, se examinan las seis permutaciones posibles del orden en que el príncipe podría jugar y se supone que la princesa responde óptimamente: usa la carta más baja que derrote la carta revelada, o su carta más baja cuando no puede ganarle. Si en alguna permutación el príncipe gana menos de dos rondas, la carta no sirve. Se prueban las cartas disponibles en orden creciente hasta hallar la primera que cumple la condición.

### Complejidad

Como máximo se evalúan 47 cartas candidatas, y para cada una se simulan 6 permutaciones de 3 rondas, por lo que el costo es constante.
