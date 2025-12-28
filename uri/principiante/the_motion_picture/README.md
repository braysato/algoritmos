# The Motion Picture

## Descripción del Problema

Durante el 3er UFFS Junior Programming Contest, la abuela Zazá descubrió que el precio de las entradas de cine aumentó y quiere protestar mostrando el porcentaje exacto del incremento. Dados el precio antiguo y el nuevo precio (ambos con dos decimales), se debe calcular el porcentaje de aumento para completar el cartel:

```
¡QUÉ ABSURDO! ¡EL PRECIO DE LAS ENTRADAS DE CINE SUBIÓ ... %!
```

## Entrada

La única línea de entrada contiene dos valores reales `A` y `B`, con `0.00 < A ≤ B ≤ 1000.00`, que representan respectivamente el precio anterior y el nuevo precio del boleto.

## Salida

Se debe imprimir una única línea con el porcentaje de aumento, en el formato `xx.xx%`, siempre con dos cifras decimales.

## Análisis y Enfoque de Solución

La variación porcentual se calcula mediante la expresión:

- `incremento = ((B - A) / A) * 100`

Basta con leer la pareja `(A, B)` y aplicar la fórmula. El resultado se imprime con dos decimales y el símbolo `%`. La solución es `O(1)` en tiempo y memoria.
