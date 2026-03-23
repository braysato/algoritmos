# Height - 1566

## Enunciado

El gobierno brasileño decidió crear el "beneficio de altura". Para ello, se te instruyó a encuestar la altura de la población de varias ciudades y ordenar esta población en orden ascendente de altura. Las ciudades no tienen más de 3 millones de personas y, según el IBGE, ninguna persona tiene más de 230 cm.

### Entrada
La entrada contiene muchos casos de prueba. La primera línea contiene un entero NC (NC < 100) que indica la cantidad de casos de prueba (ciudades). Para cada caso, la primera línea contiene un entero N (1 < N ≤ 3000000) indicando la cantidad de personas. La siguiente línea contiene la altura de cada ciudadano en centímetros h (20 ≤ h ≤ 230), separadas por espacios.

### Salida
Para cada caso, imprimir una línea con todas las alturas en orden ascendente, separadas por espacios.

**Obs:** El archivo de entrada es muy grande, se necesitan métodos de I/O rápidos.

### Ejemplo de Entrada
```
6
10
65 31 37 37 72 76 61 35 57 37
12
45 186 185 55 51 51 22 78 64 26 49 21
10
20 93 203 67 64 225 112 81 58 180
8
169 189 220 228 68 32 214 180
6
133 55 67 166 112 41
4
39 38 120 55 
```

### Ejemplo de Salida
```
31 35 37 37 37 57 61 65 72 76
21 22 26 45 49 51 51 55 64 78 185 186
20 58 64 67 81 93 112 180 203 225
32 68 169 180 189 214 220 228
41 55 67 112 133 166
38 39 55 120 
```

## Análisis

- N hasta 3,000,000 pero alturas solo van de 20 a 230 → rango de 211 valores.
- Esto es un caso perfecto para **Counting Sort** con complejidad O(N + K) donde K = 211.
- Un sort comparativo O(N log N) podría ser demasiado lento para N = 3M.
- Se requiere I/O rápido (scanf/printf en C++, BufReader/BufWriter en Rust).

## Complejidad

- Tiempo: O(N + K) por caso, donde K = 211
- Espacio: O(K) = O(211) para el array de conteo
