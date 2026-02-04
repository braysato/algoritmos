# Sort by Length - 1244

## Problema

Crea un programa para ordenar un conjunto de strings por su tamaño. Tu programa debe recibir un conjunto de strings y devolver ese mismo conjunto ordenado por tamaño de palabras. Si el tamaño de los strings es igual, debe mantener el orden original del conjunto.

### Entrada

La primera línea de entrada tiene un único entero N que indica el número de conjuntos de strings. Cada conjunto puede contener entre 1 y 50 elementos inclusive, y cada uno de los strings del conjunto puede contener entre 1 y 50 caracteres inclusive.

### Salida

La salida debe contener el conjunto de strings de entrada ordenados por la longitud de los strings. Un espacio en blanco debe ser impreso entre dos palabras.

### Ejemplo de Entrada
```
4
Top Coder comp Wedn at midnight
one three five
I love Cpp
sj a sa df r e w f d s a v c x z sd fd
```

### Ejemplo de Salida
```
midnight Coder comp Wedn Top at
three five one
love Cpp I
sj sa df sd fd a r e w f d s a v c x z
```

## Análisis

1. **Ordenamiento estable**: Necesitamos ordenar de mayor a menor longitud, pero manteniendo el orden original para strings de igual longitud. Esto requiere un ordenamiento estable.
2. **Lectura**: Leer línea completa y separar por espacios.
3. **Ordenar**: Usar `stable_sort` en C++ o mantener índices originales para desempate.

## Complejidad

- Tiempo: O(N * M * log(M)) donde M es el número de palabras por línea
- Espacio: O(M) para almacenar las palabras
