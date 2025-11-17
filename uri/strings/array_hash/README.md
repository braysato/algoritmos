# Array Hash

## Descripción del Problema

Se te darán muchas líneas de entrada con cadenas. El valor de cada carácter en la entrada se calcula de la siguiente manera:

**Valor = (Posición en el Alfabeto) + (Índice del Elemento) + (Posición en el Elemento)**

Todas las posiciones están basadas en 0. 'A' tiene posición alfabética 0, 'B' tiene posición alfabética 1, ... El hash retornado es la suma de todos los valores de caracteres en la entrada.

Por ejemplo, si la entrada es:
```
CBA
DDD
```

Entonces cada valor de carácter se calcularía de la siguiente manera:
```
2 = 2 + 0 + 0 : 'C' en elemento 0 posición 0
2 = 1 + 0 + 1 : 'B' en elemento 0 posición 1
2 = 0 + 0 + 2 : 'A' en elemento 0 posición 2
4 = 3 + 1 + 0 : 'D' en elemento 1 posición 0
5 = 3 + 1 + 1 : 'D' en elemento 1 posición 1
6 = 3 + 1 + 2 : 'D' en elemento 1 posición 2
```

El hash final sería 2+2+2+4+5+6 = 21.

## Entrada

La entrada contiene muchos casos de prueba. La primera línea de entrada contiene un entero N que indica la cantidad de casos de prueba. Cada caso de prueba comienza con un entero L (1 ≤ L ≤ 100) que indica la cantidad de líneas siguientes. Cada una de estas L líneas contendrá una cadena con hasta 50 letras mayúsculas ('A' - 'Z').

## Salida

Para cada caso de prueba, imprima el hash calculado según la explicación anterior.

## Ejemplo de Entrada
```
5
2
CBA
DDD
1
Z
6
A
B
C
D
E
F
6
ABCDEFGHIJKLMNOPQRSTUVWXYZ
ABCDEFGHIJKLMNOPQRSTUVWXYZ
ABCDEFGHIJKLMNOPQRSTUVWXYZ
ABCDEFGHIJKLMNOPQRSTUVWXYZ
ABCDEFGHIJKLMNOPQRSTUVWXYZ
ABCDEFGHIJKLMNOPQRSTUVWXYZ
1
ZZZZZZZZZZ
```

## Ejemplo de Salida
```
21
25
30
4290
295
```

## Análisis y Enfoque de Solución

Este es un problema de **cálculo de hash** basado en posiciones de caracteres.

### Fórmula del Hash

Para cada carácter en la posición `j` de la línea `i`:
```
valor = (char - 'A') + i + j
```

Donde:
- `(char - 'A')`: Posición alfabética (0-based)
- `i`: Índice de la línea (0-based)
- `j`: Posición del carácter en la línea (0-based)

El hash total es la suma de todos los valores calculados.

### Estrategia de Solución

1. Para cada caso de prueba:
   - Inicializar hash = 0
   - Leer L líneas
   - Para cada línea i (desde 0):
     - Para cada carácter en posición j (desde 0):
       - Calcular: valor = (carácter - 'A') + i + j
       - Sumar al hash
   - Imprimir el hash total

### Ejemplo de Cálculo

Para `CBA` (línea 0) y `DDD` (línea 1):

**Línea 0 (CBA):**
- C (pos 0): (2) + 0 + 0 = 2
- B (pos 1): (1) + 0 + 1 = 2
- A (pos 2): (0) + 0 + 2 = 2

**Línea 1 (DDD):**
- D (pos 0): (3) + 1 + 0 = 4
- D (pos 1): (3) + 1 + 1 = 5
- D (pos 2): (3) + 1 + 2 = 6

**Total:** 2 + 2 + 2 + 4 + 5 + 6 = 21

### Complejidad

- **Tiempo**: O(N × L × M) donde N es el número de casos de prueba, L es el número de líneas por caso, y M es la longitud promedio de las cadenas
- **Espacio**: O(1) - solo necesitamos variables para acumular el hash
