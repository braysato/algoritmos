# GCD Grid - 1511

## Enunciado

Dado un grid infinito lleno de ceros y 'Q' consultas de los siguientes tipos:

**SET x y d:** Establece la posición (x,y) del grid al entero d.

**QUERY x y d:** Retorna el gcd (Máximo Común Divisor) de todas las posiciones del grid que están a lo más 'd' posiciones de distancia (Distancia Manhattan) de la posición (x,y).

### Entrada
La entrada contiene muchos casos de prueba y termina con EOF. La primera línea de cada entrada contiene un entero Q (1 <= Q <= 10^5) que representa la cantidad de consultas a realizar.

Cada una de las siguientes Q líneas contiene una de las consultas:
- SET x y d
- QUERY x y d

Restricciones:
- 0 <= |x|, |y| <= 500
- 0 <= d <= 10^6

### Salida
Imprimir todas las consultas para todos los casos de prueba, una por línea. Para cada QUERY, imprimir el GCD de todas las posiciones dentro de la distancia Manhattan d.

### Ejemplo de Entrada
```
10
QUERY 0 0 10
SET 0 0 25
QUERY 0 0 10
SET 0 5 15
QUERY 0 0 6
SET 0 -7 5
QUERY 0 0 7
SET 0 -8 4
QUERY 0 0 8
QUERY 0 -9 1
```

### Ejemplo de Salida
```
0
25
5
5
1
4
```

## Análisis

- El grid es infinito pero las coordenadas están acotadas: |x|, |y| <= 500.
- El grid empieza lleno de ceros, y gcd(a, 0) = a, por lo que los ceros no afectan el GCD.
- Para cada QUERY solo importan las posiciones que fueron SET con valor distinto de cero.
- La distancia Manhattan máxima entre dos puntos del grid es 2000 (de (-500,-500) a (500,500)).

## Soluciones Intentadas

### Intento 1: Map + Vector de posiciones (TLE)
```cpp
map<pair<int,int>, long long> grid;
vector<pair<int,int>> positions;
// Para cada QUERY, iterar sobre todas las posiciones
```
**Por qué falló:** 
- `map` tiene acceso O(log n) en lugar de O(1)
- `cout/cin` son lentos comparados con scanf/printf
- Complejidad O(Q²) en el peor caso

### Intento 2: Array 2D + GCD Global con cache (TLE)
```cpp
vector<vector<long long>> grid(1001, vector<long long>(1001, 0));
// Cache del GCD global para d >= 2000
```
**Por qué falló:**
- `vector<vector<>>` es más lento que array estático
- Aún usa cin/cout
- El overhead de la inicialización del vector en cada caso

### Intento 3: Transformación Manhattan + Binary Search (TLE)
```cpp
// Transformación: u = x+y, v = x-y
// Ordenar por u y usar binary search
sort(positions.begin(), positions.end());
auto lo = lower_bound(...);
auto hi = upper_bound(...);
```
**Por qué falló:**
- Overhead de tuplas y structured bindings
- El sorting frecuente añade overhead
- Aún usa cin/cout

### Intento 4: SOLUCIÓN EXITOSA
```cpp
long long grid[1001][1001];  // Array estático
scanf/printf;                 // I/O rápido
for (... && result != 1)      // Early termination
if (d >= 2000) { cache }      // GCD global cacheado
```
**Por qué funcionó:**
1. **Array estático** en lugar de vector → acceso más rápido, sin overhead de asignación
2. **scanf/printf** en lugar de cin/cout → significativamente más rápido
3. **Early termination** cuando GCD=1 → no se puede reducir más, sale del loop
4. **GCD global cacheado** para d >= 2000 → evita recalcular cuando cubre todo el grid
5. **Vectores separados** para x,y → mejor cache locality que vector de pares
6. **GCD inline** → evita overhead de llamada a función

## Complejidad Final

- Tiempo: O(Q * S) donde S es el número de posiciones seteadas, pero con early termination y cache
- Espacio: O(1001²) para el grid estático
