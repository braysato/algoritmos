# Fibo's Sequence - 1701

## Enunciado

Fibo es un gran fan de los números, especialmente de los grandes. Le encantan las secuencias que crecen rápido, su favorita es la secuencia de Fibonacci. Un día decidió crear una nueva secuencia usando su favorita. Su nueva secuencia se construye multiplicando números de Fibonacci consecutivos.

Dados dos índices iniciales A y B de Fibonacci:
- El primer elemento es F(A) * F(B)
- El segundo es F(A+1) * F(B+1)
- Y así sucesivamente...

Dado N, calcular la suma de los primeros N números de la nueva secuencia.

PS: F(0) = 0 y F(1) = 1

### Entrada
Varios casos de prueba. Cada caso tiene tres enteros A, B y N (1 ≤ A, B, N ≤ 10⁹). La entrada termina con tres ceros.

### Salida
Para cada caso, imprimir la suma de los N primeros elementos de la secuencia de Fibo, MOD 1000000007.

### Ejemplo de Entrada
```
1 1 5
3 4 5
0 0 0
```

### Ejemplo de Salida
```
40
438
```

## Análisis

Necesitamos: $S = \sum_{i=0}^{N-1} F(A+i) \cdot F(B+i) \mod 10^9+7$

Con A, B, N hasta 10⁹, no podemos iterar. Usamos **exponenciación de matrices**.

### Idea clave: Estado de 5 variables

Definimos en el paso i:
- p = F(A+i) · F(B+i)
- q = F(A+i+1) · F(B+i)
- r = F(A+i) · F(B+i+1)
- s = F(A+i+1) · F(B+i+1)
- S = suma acumulada

Transición al paso i+1:
- p' = s
- q' = r + s
- r' = q + s
- s' = p + q + r + s
- S' = S + s

Esto es lineal en (p, q, r, s, S) → se puede representar como multiplicación de matriz 5×5.

### Algoritmo
1. Calcular F(A), F(A+1), F(B), F(B+1) con exponenciación de matriz 2×2
2. Construir vector inicial (p₀, q₀, r₀, s₀, S₀)
3. Aplicar la matriz de transición 5×5 elevada a (N-1)
4. El resultado es la componente S

## Complejidad

- Tiempo: O(5³ · log N + 2³ · log(max(A,B))) = O(125 log N)
- Espacio: O(1)
