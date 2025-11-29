# Etaoin Shrdlu

## Descripción del Problema

La frecuencia relativa de caracteres y pares de caracteres (dígrafos) es clave en criptografía. Para cada caso de prueba se entregan varias líneas que, concatenadas sin los saltos de línea, forman un texto. Se debe determinar los cinco dígrafos más frecuentes, reportando su frecuencia absoluta y relativa.

## Entrada

Cada caso de prueba inicia con un entero `n` (1 ≤ n ≤ 64). Le siguen `n` líneas con hasta 80 caracteres imprimibles ASCII cada una. El texto a analizar se obtiene concatenando estas líneas sin incluir los finales de línea. La entrada finaliza con `n = 0`.

## Salida

Para cada caso de prueba se deben imprimir cinco líneas. Cada línea contiene el dígrafo, su frecuencia absoluta y su frecuencia relativa (frecuencia absoluta dividida por el número total de dígrafos del texto), redondeada a 6 decimales. En caso de empates en la frecuencia absoluta, los dígrafos se ordenan lexicográficamente (orden ASCII). Después de los cinco dígrafos se imprime una línea en blanco.

## Ejemplo de Entrada
```
2
Take a look at this!!
!!siht ta kool a ekaT
5
P=NP
 Authors: A. Cookie, N. D. Fortune, L. Shalom
 Abstract: We give a PTAS algorithm for MaxSAT and apply the PCP-Theorem [3]
 Let F be a set of clauses. The following PTAS algorithm gives an optimal
 assignment for F:
0
```

## Ejemplo de Salida
```
	a 3 0.073171
!! 3 0.073171
a  3 0.073171
 t 2 0.048780
oo 2 0.048780

 a 8 0.037209
or 7 0.032558
.  5 0.023256
e  5 0.023256
al 4 0.018605
```

## Análisis y Enfoque de Solución

Un dígrafo corresponde a dos caracteres consecutivos del texto concatenado. El número total de dígrafos es `L - 1`, donde `L` es la longitud del texto.

### Observaciones Clave

- El texto se procesa como una cadena continua sin saltos de línea.
- Existen como máximo 95 caracteres ASCII imprimibles, por lo que un conteo directo de dígrafos es factible.
- Los dígrafos se deben ordenar primero por frecuencia absoluta (descendente) y, ante empate, por orden lexicográfico ascendente.

### Estrategia de Solución

1. Leer `n` y las `n` líneas, concatenándolas en una sola cadena `text`.
2. Recorrer `text` con una ventana de tamaño 2, acumulando las frecuencias de cada dígrafo en un mapa.
3. Calcular el total de dígrafos `total = max(0, L - 1)` para obtener la frecuencia relativa.
4. Trasladar el mapa a un vector y ordenarlo por frecuencia descendente y dígrafo ascendente.
5. Imprimir los cinco primeros elementos con la frecuencia absoluta y la relativa redondeada a 6 decimales.
6. Imprimir una línea en blanco y continuar con el siguiente caso.

### Complejidad

- **Tiempo**: O(L + U log U), donde `L` es la longitud del texto y `U` el número de dígrafos distintos (a lo sumo 95²).
- **Espacio**: O(U) para almacenar las frecuencias de los dígrafos.
