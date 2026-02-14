# Hash Tables - 1256

## Problema

Las tablas hash se utilizan para almacenar elementos basándose en el valor absoluto de sus claves y técnicas de manejo de colisiones. Para calcular una dirección donde debe almacenarse una clave determinada, se usa una función llamada función hash que transforma la clave en una de las direcciones disponibles en la tabla.

Supongamos que una aplicación utiliza una tabla hash con 13 direcciones base (índices 0 a 12) y usa la función de dispersión h(x) = x mod 13, donde x es la clave cuya dirección base se calcularía.

Si x = 49, la función hash devolverá 10, indicando la ubicación (dirección) donde esta clave debe almacenarse. Si necesitamos insertar la clave 88 en la misma aplicación, el cálculo devuelve el mismo valor 10, ocurrirá una colisión. El tratamiento de colisiones se usa para resolver conflictos cuando más de una clave se mapea a la misma dirección. Este tratamiento puede considerar recálculo de dirección o encadenamiento externo.

El profesor te pidió escribir un programa que calcule la dirección para muchas claves en algunas tablas, con funciones de dispersión y tratamiento de colisión por encadenamiento externo.

### Entrada

La entrada contiene muchos casos de prueba. La primera línea contiene un entero N indicando el número de casos. Cada caso se compone de dos líneas. La primera contiene un entero M (1 ≤ M ≤ 100) que indica el número de direcciones base en la tabla (usualmente un número primo) seguido de un espacio y un entero C (1 ≤ C ≤ 200) que indica la cantidad de claves a almacenar. La segunda contiene cada una de las C claves (con valor entre 1 y 200), separadas por un espacio.

### Salida

La salida debe imprimirse como los siguientes ejemplos, donde la cantidad de líneas de cada caso está determinada por el valor de M. Una línea en blanco debe separar cada conjunto de salida.

### Ejemplo de Entrada
```
2
13 9
44 45 49 70 27 73 92 97 95
7 8
35 12 2 17 19 51 88 86
```

### Ejemplo de Salida
```
0 -> \
1 -> 27 -> 92 -> \
2 -> \
3 -> \
4 -> 95 -> \
5 -> 44 -> 70 -> \
6 -> 45 -> 97 -> \
7 -> \
8 -> 73 -> \
9 -> \
10 -> 49 -> \
11 -> \
12 -> \

0 -> 35 -> \
1 -> \
2 -> 2 -> 51 -> 86 -> \
3 -> 17 -> \
4 -> 88 -> \
5 -> 12 -> 19 -> \
6 -> \
```

## Análisis

1. **Tabla Hash**: Crear un vector de M listas (encadenamiento externo).
2. **Función Hash**: h(x) = x mod M.
3. **Inserción**: Para cada clave, calcular su índice y agregarla al final de la lista correspondiente.
4. **Salida**: Imprimir cada índice con sus claves encadenadas, terminando con "\".

## Complejidad

- Tiempo: O(N * (M + C)) para procesar e imprimir
- Espacio: O(M + C) para la tabla hash
