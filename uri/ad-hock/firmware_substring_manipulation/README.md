# Firmware - Substring Manipulation

## Descripción del Problema

Dadas dos cadenas `s` y `part`, se elimina repetidamente la ocurrencia más a la izquierda de `part` en `s` hasta que no queden coincidencias. Se debe devolver la cadena resultante.

## Entrada

Cada caso de prueba contiene dos líneas:
- Primera línea: cadena `s`, con 1 ≤ |s| ≤ 1000.
- Segunda línea: cadena `part`, con 1 ≤ |part| ≤ 1000.

Ambas cadenas usan letras minúsculas del alfabeto inglés. Si falta la segunda línea, la salida correspondiente debe ser `null value`.

## Salida

Para cada caso de prueba se imprime la cadena resultante tras eliminar todas las ocurrencias de `part`. Si `part` no se suministra o el resultado final queda vacío, imprimir `null value`.

## Ejemplo de Entrada
```
axxxxyyyyb
xy
```

## Ejemplo de Salida
```
ab
```

## Análisis y Enfoque de Solución

Se requieren eliminaciones sucesivas desde la izquierda. Un enfoque iterativo con un búfer que actúa como pila permite simular estas eliminaciones en un solo barrido:

1. Recorrer `s` caracter por caracter agregándolos al búfer.
2. Cada vez que el búfer termina con `part`, quitar esos caracteres.
3. Al finalizar el recorrido, el búfer contiene el resultado deseado.

Este método garantiza que siempre se eliminen las coincidencias más a la izquierda porque el recorrido avanza de izquierda a derecha.

### Complejidad

- **Tiempo**: O(|s| · |part|) en el peor caso al verificar coincidencias con el sufijo.
- **Espacio**: O(|s|) para almacenar el búfer temporal.
