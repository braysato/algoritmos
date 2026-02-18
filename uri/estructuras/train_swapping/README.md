# Train Swapping - 1162

## Problema

En una vieja estación de tren, aún puedes encontrar uno de los últimos "intercambiadores de trenes". Un intercambiador de trenes es un empleado del ferrocarril, cuyo único trabajo es reorganizar los vagones de los trenes.

Una vez que los vagones están organizados en el orden óptimo, todo lo que el conductor tiene que hacer es dejar los vagones, uno por uno, en las estaciones para las cuales está destinada la carga.

El primer intercambiador de trenes había descubierto que el puente podía operarse con máximo dos vagones sobre él. Al rotar el puente 180 grados, los vagones intercambiaban lugar, permitiéndole reorganizar los vagones.

Ahora que casi todos los intercambiadores de trenes han desaparecido, la compañía ferroviaria quisiera automatizar su operación. Parte del programa a desarrollar es una rutina que decide para un tren dado el menor número de intercambios de dos vagones adyacentes necesarios para ordenar el tren. Tu tarea es crear esa rutina.

### Entrada

La entrada contiene en la primera línea el número de casos de prueba (N). Cada caso consiste de dos líneas. La primera línea contiene un entero L, determinando la longitud del tren (0 ≤ L ≤ 50). La segunda línea contiene una permutación de los números 1 hasta L, indicando el orden actual de los vagones. Los vagones deben ordenarse tal que el vagón 1 venga primero, luego el 2, etc., con el vagón L al final.

### Salida

Para cada caso de prueba imprimir la oración: 'Optimal train swapping takes S swaps.' donde S es un entero.

### Ejemplo de Entrada
```
3
3
1 3 2
4
4 3 2 1
2
2 1
```

### Ejemplo de Salida
```
Optimal train swapping takes 1 swaps.
Optimal train swapping takes 6 swaps.
Optimal train swapping takes 1 swaps.
```

## Análisis

1. **Inversiones**: El número mínimo de intercambios adyacentes para ordenar un arreglo es igual al número de inversiones.
2. **Inversión**: Un par (i, j) donde i < j pero arr[i] > arr[j].
3. **Solución**: Contar todas las inversiones con doble bucle O(n²), o simular Bubble Sort contando swaps.

## Complejidad

- Tiempo: O(N * L²) donde L es la longitud del tren
- Espacio: O(L) para almacenar el arreglo
