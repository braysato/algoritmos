# Parking Lot

## Descripción del Problema

Un estacionamiento lineal cobra una tarifa fija de R$ 10,00 por vehículo que logra estacionar. Cada vehículo ocupa un segmento continuo proporcional a su longitud. Al llegar, se busca la primera vacante (más cercana al inicio) cuyo tamaño sea suficiente; si se encuentra, el vehículo se coloca al inicio de esa vacante y se cobra la tarifa. Si no hay espacio, el vehículo se va sin pagar. Al salir, el espacio vuelve a quedar libre. Dada la secuencia cronológica de llegadas y salidas, se debe calcular la recaudación total.

## Entrada

Cada caso de prueba comienza con dos enteros `C` (1 ≤ C ≤ 1000) y `N` (1 ≤ N ≤ 10000), la longitud del estacionamiento y el número de eventos. Las siguientes `N` líneas describen un evento:

- `C P Q`: llegada de un vehículo con placa `P` (1000 ≤ P ≤ 9999) y longitud `Q` (1 ≤ Q ≤ C).
- `S P`: salida del vehículo con placa `P`.

Los eventos se dan en orden cronológico; nunca sale un vehículo que no está estacionado y jamás llega un vehículo con la misma placa de uno que esté estacionado en ese momento. El archivo termina con fin de archivo; cada caso comienza con el estacionamiento vacío.

## Salida

Para cada caso de prueba imprimir una línea con la recaudación total (en reales) considerando que cada vehículo estacionado paga exactamente R$ 10,00.

## Ejemplo de Entrada
```
10 7
C 1234 5
C 1111 4
C 2222 4
C 4321 3
S 1111
C 2002 6
C 4321 3
30 10
C 1000 10
C 1001 10
C 1002 10
S 1000
S 1002
C 1003 20
S 1001
C 1004 20
S 1004
C 1005 30
20 10
C 1234 20
C 5678 1
S 1234
C 1234 20
C 5678 1
S 1234
C 5678 1
C 1234 20
C 5555 1
S 5678
```

## Ejemplo de Salida
```
30
50
40
```

## Análisis y Enfoque de Solución

Mantener una representación lineal del estacionamiento permite simular los eventos. Para cada llegada se recorre el estacionamiento buscando la primera secuencia contigua de espacios libres cuya longitud sea mayor o igual a la del vehículo. Si se encuentra, se ocupa ese segmento y se registra la tarifa; caso contrario, se ignora el evento. Para las salidas basta con liberar el segmento previamente asociado a la placa.

### Complejidad

Con `C ≤ 1000`, la búsqueda secuencial de espacios y la liberación tienen costo O(C) por evento, lo que es suficiente incluso para `N = 10000`.
