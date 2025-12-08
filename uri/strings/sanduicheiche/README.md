# Sanduicheiche

## Descripción del Problema

La nutricionista Root Escrevemos repite el final de cada palabra cuando está nerviosa, duplicando un sufijo. Dada la palabra "engañada", se debe recuperar la palabra correcta eliminando el sufijo repetido.

## Entrada

Cada caso es una palabra de hasta 30 caracteres, donde las letras duplicadas son minúsculas. La entrada termina con fin de archivo.

## Salida

Para cada palabra, imprimir la versión corregida sin la parte repetida.

## Ejemplo de Entrada
```
sanduicheiche

barrilarril

ratoato

sol

coliseueu

queijoijo

astroastro

a
```

## Ejemplo de Salida
```
sanduiche

barril

rato

sol

coliseu

queijo

astro

a
```

## Análisis y Enfoque de Solución

El error consiste en duplicar un sufijo completo al final de la palabra. Para encontrarlo, basta revisar longitudes `len` desde 1 hasta `L/2` y verificar si los últimos `2*len` caracteres son dos copias consecutivas del mismo segmento. El primer `len` que cumpla la condición corresponde al sufijo repetido; el resultado es la palabra sin la segunda copia. Si no se detecta duplicación, se devuelve la palabra original.

### Complejidad

Con `L ≤ 30`, recorrer todas las longitudes posibles implica un costo O(L²), suficiente para los límites del problema.
