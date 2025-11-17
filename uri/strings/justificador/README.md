# Justificador

## Descripción del Problema

Tenemos algunas palabras y queremos justificarlas a la derecha, es decir, alinearlas a la derecha. Crea un programa que lea una palabra e imprima todo justificado a la derecha, en el mismo orden en que aparecen en la entrada.

## Entrada

La entrada contiene varios casos de prueba. La primera línea de un caso de prueba contendrá un número entero N (1 ≤ N ≤ 50) que indica el número de palabras siguientes. Cada palabra está compuesta de hasta 50 letras mayúsculas ('A'-'Z') y contendrá al menos una letra. El final de la entrada se indica con N = 0.

## Salida

Para cada caso de prueba, imprima las palabras rellenadas a la izquierda con caracteres de espacio para que tengan la misma longitud que la palabra más larga encontrada en ese texto. Imprima una línea vacía entre todos los casos de prueba. No debe haber ningún espacio impreso en la parte de atrás, y debe descartar cualquier espacio principal innecesario, para que al menos una línea en cada palabra de salida comienza con una letra.

## Ejemplo de Entrada
```
3
BOB
TOMMY
JIM
4
JOHN
JAKE
ALAN
BLUE
4
LONGEST
A
LONGER
SHORT
0
```

## Ejemplo de Salida
```
  BOB
TOMMY
  JIM

JOHN
JAKE
ALAN
BLUE

LONGEST
      A
 LONGER
  SHORT
```

## Análisis y Enfoque de Solución

Este es un problema de **formateo de texto** que requiere alinear palabras a la derecha.

### Observaciones Clave

1. **Justificación a la derecha**: Todas las palabras deben tener la misma longitud visual, agregando espacios a la izquierda de las palabras más cortas.

2. **Longitud de referencia**: La longitud de la palabra más larga determina el ancho de la columna.

3. **Espaciado entre casos**: Debe haber una línea vacía entre cada caso de prueba (excepto después del último).

4. **Sin espacios al final**: No se deben imprimir espacios al final de las líneas.

### Estrategia de Solución

1. **Leer todas las palabras** de un caso de prueba y almacenarlas en un vector/lista.

2. **Encontrar la longitud máxima** entre todas las palabras del caso actual.

3. **Imprimir cada palabra** con el padding necesario:
   - Calcular espacios necesarios: `maxLength - palabraActual.length()`
   - Imprimir espacios + palabra

4. **Separar casos de prueba** con una línea vacía (excepto después del último caso).

### Implementación

```
Para cada caso de prueba:
  1. Leer N palabras en un vector
  2. Encontrar maxLength = max(longitud de todas las palabras)
  3. Para cada palabra:
     - spaces = maxLength - palabra.length()
     - Imprimir (spaces espacios) + palabra
  4. Si no es el último caso, imprimir línea vacía
```

### Detalles Importantes

- No hay espacios al final de las líneas (solo espacios al inicio para justificar)
- La línea vacía entre casos se imprime ANTES del siguiente caso (no después del actual)
- El último caso NO debe tener línea vacía después

### Complejidad

- **Tiempo**: O(N × M) donde N es el número de palabras y M es la longitud promedio de las palabras
- **Espacio**: O(N × M) para almacenar todas las palabras
