# Sudoku

## Descripción del Problema

El Sudoku se ha expandido a través del mundo, siendo el hobby más popular del planeta hoy en día. A pesar de esto, algunas personas completan la matriz de forma incorrecta, rompiendo las reglas. Tu tarea será escribir un programa que verifique si la matriz es una solución correcta al Sudoku o no.

La matriz del Sudoku es una matriz de enteros de 9 x 9. Para que una solución sea considerada correcta, cada fila y cada columna deben contener todos los números enteros entre el 1 y el 9. También, si la matriz se encuentra particionada en nueve sub-matrices de 3 x 3, cada una de éstas sub-matrices también debe contener todos los números del 1 al 9.

## Entrada

Se darán varias instancias. La primera línea contiene n > 0, el número de matrices en la entrada. Las siguientes líneas describen las n matrices. Cada matriz está descripta por 9 líneas. Estas líneas contienen 9 enteros cada una.

## Salida

Por cada instancia, su programa deberá imprimir una línea conteniendo "Instancia k", donde k es el número de instancia. En la segunda línea, su programa deberá imprimir "SIM" (si en portugués) si la matriz dada es una solución correcta al Sudoku, o "NAO" (no en portugués) en el caso contrario. Imprimir una línea vacía luego de cada instancia.

## Ejemplo de Entrada
```
2
1 3 2 5 7 9 4 6 8
4 9 8 2 6 1 3 7 5
7 5 6 3 8 4 2 1 9
6 4 3 1 5 8 7 9 2
5 2 1 7 9 3 8 4 6
9 8 7 4 2 6 5 3 1
2 1 4 9 3 5 6 8 7
3 6 5 8 1 7 9 2 4
8 7 9 6 4 2 1 5 3
1 3 2 5 7 9 4 6 8
4 9 8 2 6 1 3 7 5
7 5 6 3 8 4 2 1 9
6 4 3 1 5 8 7 9 2
5 2 1 7 9 3 8 4 6
9 8 7 4 2 6 5 3 1
2 1 4 9 3 5 6 8 7
3 6 5 8 1 7 9 2 4
8 7 9 6 4 2 1 3 5
```

## Ejemplo de Salida
```
Instancia 1
SIM

Instancia 2
NAO
```

## Análisis y Enfoque de Solución

Este es un problema de **validación** que requiere verificar tres condiciones para determinar si un Sudoku está correctamente resuelto.

### Reglas del Sudoku

Para que una matriz 9x9 sea una solución válida de Sudoku debe cumplir:

1. **Filas válidas**: Cada una de las 9 filas debe contener todos los números del 1 al 9 sin repetición
2. **Columnas válidas**: Cada una de las 9 columnas debe contener todos los números del 1 al 9 sin repetición
3. **Sub-matrices 3x3 válidas**: Cada una de las 9 sub-matrices de 3x3 debe contener todos los números del 1 al 9 sin repetición

### Estrategia de Solución

1. **Leer la matriz**: Almacenar la matriz 9x9 en una estructura de datos

2. **Validar filas**: Para cada fila, verificar que contiene exactamente los números 1-9
   - Usar un conjunto o arreglo booleano para detectar duplicados

3. **Validar columnas**: Para cada columna, verificar que contiene exactamente los números 1-9

4. **Validar sub-matrices 3x3**: 
   - Dividir la matriz en 9 regiones de 3x3
   - Para cada región, verificar que contiene exactamente los números 1-9
   - Las regiones empiezan en (0,0), (0,3), (0,6), (3,0), (3,3), (3,6), (6,0), (6,3), (6,6)

5. **Resultado**: Si todas las validaciones pasan, imprimir "SIM", caso contrario "NAO"

### Implementación

- Usar un arreglo booleano de tamaño 10 (índices 1-9) para verificar que cada número aparece exactamente una vez
- Iterar sobre filas, columnas y sub-matrices sistemáticamente
- Si cualquier validación falla, marcar como inválido

### Complejidad

- **Tiempo**: O(1) - siempre verificamos una matriz de 9x9 (81 celdas)
- **Espacio**: O(1) - usamos espacio constante para la validación
