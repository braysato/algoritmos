# Grandma Day - 1591

## Problema

La abuela está resolviendo una sopa de letras. Quieres ayudarla y construirás un algoritmo que, dado el puzzle y las palabras a buscar, imprima cuántas veces aparece cada palabra.

En este puzzle, las palabras estarán solo en horizontal o vertical. Las palabras no se envuelven. Las palabras pueden superponerse, es decir, la misma letra en el puzzle puede ser usada por más de una palabra. ¡Las palabras de una sola letra se cuentan solo una vez, ver el primer caso de prueba!

### Entrada

La primera línea contiene un entero T (T <= 100) indicando el número de casos de prueba.

La primera línea de cada caso tendrá dos números, L (1 ≤ L ≤ 10* o 1 ≤ L ≤ 50**) y C (1 ≤ C ≤ 10* o 1 ≤ C ≤ 50**), el número de filas y columnas del puzzle respectivamente. Las siguientes L líneas tendrán C letras minúsculas del alfabeto inglés indicando la configuración del puzzle. La siguiente línea tendrá un entero P (1 ≤ P ≤ 50) indicando cuántas palabras hay que buscar. Las siguientes P líneas contienen las palabras a buscar.

*para alrededor del 90% de los casos;
**para los otros casos.

### Salida

Para cada caso, imprimir para cada palabra en una sola línea el número de veces que aparece en el puzzle, en el mismo orden de la entrada. Si la palabra no se encuentra, imprimir 0.

### Ejemplo de Entrada
```
3
3 3
asa
bao
oab
6
a
asa
bao
boa
aob
ab
5 5
abcde
fghij
klmno
pqrst
uvwxy
6
agm
cdef
imq
ye
au
gfji
4 3
aaa
aaa
aaa
aaa
3
a
aa
aaa
```

### Ejemplo de Salida
```
4
1
1
0
1
2
0
0
0
0
0
0
12
17
10
```

## Análisis

1. **Direcciones de búsqueda**: Las palabras pueden estar en 4 direcciones:
   - Horizontal izquierda a derecha
   - Horizontal derecha a izquierda
   - Vertical arriba a abajo
   - Vertical abajo a arriba

2. **Caso especial**: Palabras de una sola letra se cuentan solo una vez (no en las 4 direcciones).

3. **Estrategia**: Para cada palabra, recorrer todas las posiciones posibles en cada dirección y contar coincidencias.

## Complejidad

- Tiempo: O(T * P * L * C * max_len) donde max_len es la longitud máxima de palabra
- Espacio: O(L * C) para almacenar el puzzle

## Corrección: Wrong Answer

**Error inicial**: Se buscaba en 4 direcciones (horizontal izq→der, horizontal der→izq, vertical arriba→abajo, vertical abajo→arriba). Esto causaba que palíndromos como "asa" se contaran dos veces.

**Segundo intento fallido**: Buscar en 2 direcciones + buscar el reverso de la palabra. Esto era incorrecto porque buscar el reverso de "bao" ("oab") no es lo mismo que buscar "bao" escrito al revés en el grid.

**Solución correcta**: Solo buscar en 2 direcciones:
- Horizontal de izquierda a derecha
- Vertical de arriba hacia abajo

No se busca en las direcciones inversas ni el reverso de las palabras. Las palabras de una sola letra se cuentan solo una vez.
