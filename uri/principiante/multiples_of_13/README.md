# 1132 - Multiples of 13

## Problema

Write a program that reads two integer numbers X and Y and calculate the sum of all number not divisible by 13 between them, including both.

**Input**
The input file contains 2 integer numbers X and Y without any order.

**Output**
Print the sum of all numbers between X and Y not divisible by 13, including them if it is the case.

**Ejemplo:**
- Input: 100, 200
- Output: 13954

## Análisis

1. **Entrada**: Dos números enteros X e Y (pueden estar en cualquier orden)
2. **Objetivo**: Sumar todos los números entre X e Y que NO sean divisibles por 13
3. **Consideraciones**:
   - Los números pueden venir en cualquier orden (X puede ser mayor que Y o viceversa)
   - Debemos incluir X e Y en el rango si no son divisibles por 13
   - Un número es divisible por 13 si `n % 13 == 0`

## Solución

**Algoritmo:**
1. Leer X e Y
2. Determinar el mínimo y máximo entre X e Y para establecer el rango
3. Iterar desde el mínimo hasta el máximo (inclusive)
4. Para cada número, verificar si NO es divisible por 13
5. Si no es divisible por 13, sumarlo al total
6. Imprimir la suma final

**Complejidad:**
- Temporal: O(|Y - X|) - iteramos sobre todos los números en el rango
- Espacial: O(1) - solo usamos variables auxiliares
