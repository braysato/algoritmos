# Text Correction

## Descripción del Problema

En muchos idiomas no se permiten espacios en blanco justo antes de una coma o un punto. Dado un texto, se debe eliminar un único espacio que preceda inmediatamente a una coma o a un punto.

## Entrada

Varias líneas hasta fin de archivo. Cada línea contiene una cadena no vacía con caracteres alfanuméricos, espacios, comas o puntos.

## Salida

Para cada línea, imprimir la versión corregida del texto, sin espacios inmediatamente antes de comas o puntos (si hay múltiples espacios consecutivos, sólo se elimina uno por cada puntuación).

## Ejemplo de Entrada
```
Please , remove any blank space before a comma or a period .
ABc , 123  .
```

## Ejemplo de Salida
```
Please, remove any blank space before a comma or a period.
ABc, 123 .
```

## Análisis y Enfoque de Solución

Se procesa la línea carácter por carácter construyendo la salida en un búfer. Cada vez que se encuentra una coma o un punto, si el último carácter ya agregado es un espacio, se elimina antes de añadir la puntuación. Si hay más de un espacio consecutivo, sólo se suprime uno.

### Complejidad

La corrección se realiza en O(n) por línea, siendo n la longitud de la cadena.
