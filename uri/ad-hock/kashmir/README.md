# Kashmir

## Descripción del Problema

Tiago, Roberto, João y João Paulo son muy buenos amigos y viven en Kashmir. En esta tierra son muy aficionados a jugar con los números. Uno de los juegos es el siguiente, es una secuencia. Los términos primero y segundo son iguales a 1, y los siguientes términos son la suma de los 2 términos anteriores. De esta forma, el comienzo de la secuencia es 1, 1, 2, 3, 5,... A los amigos les gusta jugar de la siguiente manera: dado un número natural N, descubre el número N-ésimo de esa secuencia. El otro juego que les gusta es tomar 2 números naturales y encontrar el mayor divisor que estos números tienen en común.

En un hermoso día, decidieron unir los 2 juegos: dados 2 números naturales N y M, calcular el N-ésimo y el M-ésimo número de la secuencia y comprobar si el máximo común divisor entre los números de la secuencia también es un número de esa secuencia. y en qué posición de la secuencia se encuentra.

Ejemplo: el tercer número de la secuencia es 2 y el cuarto número de la secuencia es 3. El máximo común divisor entre 2 y 3 es 1, que está en la secuencia en las posiciones 1 y 2.

## Entrada

La entrada contiene 2 números enteros, N y M (1 ≤ N, M ≤ 10⁹) que representan las posiciones de la secuencia.

## Salida

La salida contiene solo un número entero que representa en qué posición de la secuencia se encuentra el máximo común divisor. Si hay más de una ocurrencia en la secuencia, imprima la posición más pequeña; y si el máximo común divisor no está en la secuencia, imprima -1.

## Ejemplo de Entrada
```
3 4
```

## Ejemplo de Salida
```
1
```

## Análisis y Enfoque de Solución

Este problema trata sobre la **secuencia de Fibonacci** y sus propiedades matemáticas.

### Observaciones Clave

1. **La secuencia descrita es Fibonacci**: F(1) = 1, F(2) = 1, F(n) = F(n-1) + F(n-2)

2. **Propiedad fundamental**: Existe una propiedad matemática que establece que:
   - **GCD(F(n), F(m)) = F(GCD(n, m))**
   
   Esto significa que el máximo común divisor de dos números de Fibonacci en las posiciones N y M es igual al número de Fibonacci en la posición GCD(N, M).

3. **Implicación directa**: 
   - No necesitamos calcular los valores de F(N) y F(M) (que pueden ser enormes para N, M ≤ 10⁹)
   - Solo necesitamos calcular GCD(N, M)
   - La respuesta siempre será GCD(N, M), nunca -1

### Estrategia de Solución

1. Leer N y M
2. Calcular GCD(N, M) usando el algoritmo de Euclides
3. Retornar GCD(N, M) como la posición en la secuencia

### Complejidad

- **Tiempo**: O(log(min(N, M))) - complejidad del algoritmo de Euclides
- **Espacio**: O(1)

### Justificación Matemática

La propiedad GCD(F(n), F(m)) = F(GCD(n, m)) se puede demostrar usando:
- La identidad de Fibonacci: F(m+n) = F(m)×F(n+1) + F(m-1)×F(n)
- Propiedades del máximo común divisor
- Inducción matemática

Por ejemplo:
- F(3) = 2, F(4) = 3
- GCD(2, 3) = 1
- GCD(3, 4) = 1
- F(1) = 1 ✓
