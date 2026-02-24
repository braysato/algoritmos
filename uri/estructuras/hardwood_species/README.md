# Hardwood Species - 1260

## Problema

Los árboles de madera dura son el grupo botánico de árboles que tienen hojas anchas, producen una fruta o nuez, y generalmente quedan inactivos en invierno.

Los climas templados de América producen bosques con cientos de especies de madera dura -- árboles que comparten ciertas características biológicas. Aunque roble, maple y cerezo son todos tipos de árboles de madera dura, por ejemplo, son diferentes especies. Juntas, todas las especies de madera dura representan el 40 por ciento de los árboles en Estados Unidos.

Usando tecnología de imágenes satelitales, el Departamento de Recursos Naturales ha compilado un inventario de cada árbol en un día particular. Debes calcular la fracción total de la población de árboles representada por cada especie.

### Entrada

La primera línea es el número de casos de prueba, seguido de una línea en blanco. Cada caso de prueba consiste en una lista de especies de cada árbol observado por el satélite; un árbol por línea. Ningún nombre de especie excede 30 caracteres. No hay más de 10,000 especies y no más de 1,000,000 de árboles. Hay una línea en blanco entre cada caso de prueba consecutivo.

### Salida

Para cada caso de prueba imprimir el nombre de cada especie representada en la población, en orden alfabético, seguido del porcentaje de la población que representa, con 4 decimales. Imprimir una línea en blanco entre 2 conjuntos de datos consecutivos.

### Ejemplo de Entrada
```
2

Red Alder
Ash
Aspen
Basswood
Ash
Beech
Yellow Birch
Ash
Cherry
Cottonwood
Ash
Cypress
Red Elm
Gum
Hackberry
White Oak
Hickory
Pecan
Hard Maple
White Oak
Soft Maple
Red Oak
Red Oak
White Oak
Poplan
Sassafras
Sycamore
Black Walnut
Willow

Red Alder
Ash
```

### Ejemplo de Salida
```
Ash 13.7931
Aspen 3.4483
Basswood 3.4483
Beech 3.4483
Black Walnut 3.4483
Cherry 3.4483
Cottonwood 3.4483
Cypress 3.4483
Gum 3.4483
Hackberry 3.4483
Hard Maple 3.4483
Hickory 3.4483
Pecan 3.4483
Poplan 3.4483
Red Alder 3.4483
Red Elm 3.4483
Red Oak 6.8966
Sassafras 3.4483
Soft Maple 3.4483
Sycamore 3.4483
White Oak 10.3448
Willow 3.4483
Yellow Birch 3.4483

Ash 50.0000
Red Alder 50.0000
```

## Análisis

1. **Estructura de datos**: Usar un `map<string, int>` para contar frecuencias y mantener orden alfabético automáticamente.
2. **Lectura**: Leer líneas hasta encontrar línea vacía o EOF para cada caso.
3. **Cálculo**: porcentaje = (count * 100.0) / total
4. **Formato**: 4 decimales de precisión.

## Complejidad

- Tiempo: O(N * log(S)) donde N es número de árboles y S número de especies
- Espacio: O(S) para el mapa de especies
