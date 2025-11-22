# Array Selection

## Descripción del Problema

En este problema, tu tarea es leer un arreglo A[100]. Al final, imprimir todas las posiciones del arreglo que almacenan un número menor o igual a 10 y el número almacenado en esa posición.

## Entrada

La entrada contiene 100 números. Cada número puede ser entero, de punto flotante, positivo o negativo.

## Salida

Para cada número del arreglo que sea igual a 10 o menor, imprima "A[i] = x", donde i es la posición del arreglo y x es el número almacenado en la posición, con un dígito después del punto decimal.

## Ejemplo de Entrada
```
0
-5
63
-8.5
...
```

## Ejemplo de Salida
```
A[0] = 0.0
A[1] = -5.0
A[3] = -8.5
...
```

## Análisis y Enfoque de Solución

Este es un problema simple de **filtrado de arreglos** que requiere leer números y mostrar solo aquellos que cumplan una condición.

### Condición de Selección

Imprimir solo los elementos del arreglo donde el valor sea **menor o igual a 10**.

### Estrategia de Solución

1. Leer 100 números y almacenarlos en un arreglo (o procesarlos uno por uno)
2. Para cada posición i (de 0 a 99):
   - Si el valor en la posición i es <= 10
   - Imprimir "A[i] = valor" con un decimal

### Optimización

No es necesario almacenar todo el arreglo. Podemos procesar cada número mientras lo leemos:
```
Para i desde 0 hasta 99:
  Leer número en posición i
  Si número <= 10:
    Imprimir "A[i] = número" (con 1 decimal)
```

### Formato de Salida

- Usar formato con un decimal: `.1f` en C/C++ o `:.1` en Rust
- Ejemplo: `A[0] = 0.0`, `A[1] = -5.0`

### Complejidad

- **Tiempo**: O(N) donde N = 100 (constante)
- **Espacio**: O(1) si procesamos en línea, O(N) si almacenamos el arreglo
