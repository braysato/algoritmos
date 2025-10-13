# 1171 - Frecuencia de los Números

## Problema

En este problema su trabajo será leer algunos números enteros positivos e imprimir cuántas veces aparece cada número en la entrada. Deberá escribir cada valor diferente que aparezca en la entrada, ordenados ascendentemente.

### Entrada
La entrada consiste de un único caso de prueba. La primera línea contiene un entero N, que indica la cantidad de números enteros X (1 ≤ X ≤ 2000) que se deberán leer. Cada número no aparecerá más de 20 veces en la entrada.

### Salida
Imprima la salida como se muestra en el caso de ejemplo, indicando cuántas veces aparece cada número en la entrada, ascendentemente.

### Ejemplo de Entrada
```
7
8
10
8
260
4
10
10
```

### Ejemplo de Salida
```
4 aparece 1 vez(es)
8 aparece 2 vez(es)
10 aparece 3 vez(es)
260 aparece 1 vez(es)
```

## Análisis del Problema

### Comprensión
- Debemos leer N números enteros positivos
- Contar la frecuencia de aparición de cada número único
- Imprimir los números en orden ascendente junto con su frecuencia

### Observaciones Clave
1. **Rango de valores**: Los números X están en el rango [1, 2000]
2. **Frecuencia máxima**: Cada número aparece como máximo 20 veces
3. **Orden**: La salida debe estar ordenada ascendentemente por el valor del número
4. **Formato**: El formato de salida es específico: `{número} aparece {frecuencia} vez(es)`

## Enfoque de Solución

### Array de frecuencias
Dado que X ≤ 2000, utilizamos un array de tamaño 2001 donde:
- El índice representa el número
- El valor almacenado representa su frecuencia

**Pasos:**
1. Crear un array de 2001 posiciones inicializado en 0
2. Leer N números e incrementar el contador en la posición correspondiente
3. Recorrer el array del 1 al 2000 e imprimir los números con frecuencia > 0

**Ventajas:**
- Simple y directo
- Acceso O(1) para incrementar frecuencias
- Ya está ordenado por índice
- Usa memoria fija de 2001 enteros

## Complejidad

- **Tiempo**: O(N + M) donde M = 2000 (recorrer el array)
- **Espacio**: O(M) = O(2000) = O(1) espacio constante
