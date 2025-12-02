# Tiempo de Juego con Minutos

## Descripción del Problema

Leer la hora y minuto de inicio y de final de un juego, que puede terminar al día siguiente. Calcular la duración total en horas y minutos.

## Entrada

Cuatro enteros: hora inicial, minuto inicial, hora final, minuto final (0 ≤ horas ≤ 23, 0 ≤ minutos ≤ 59).

## Salida

Imprimir la duración en el formato:
```
O JOGO DUROU X HORA(S) E Y MINUTO(S)
```

## Ejemplo de Entrada
```
7 8 9 10
```

## Ejemplo de Salida
```
O JOGO DUROU 2 HORA(S) E 2 MINUTO(S)
```

## Análisis y Enfoque de Solución

Convertimos ambos horarios a minutos desde las 00:00. Si el instante final es menor o igual que el inicial, sumamos 1440 minutos (24 horas) al final para simular que terminó al día siguiente. La duración es la diferencia de minutos y la expresamos en horas y minutos con división y módulo.

### Complejidad

Constante O(1).
