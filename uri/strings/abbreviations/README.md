# Abbreviations

## Descripción del Problema

Rafael quiere reducir la cantidad de caracteres en su publicación escogiendo, para cada letra inicial, una palabra del texto que comience con ella. Cada vez que la palabra seleccionada aparezca, se reemplaza por la abreviatura "c.", donde `c` es la letra inicial. Solo conviene abreviar cuando el número total de caracteres disminuye.

## Entrada

Cada caso de prueba es una línea con hasta 10⁴ caracteres, que contiene palabras en minúsculas y espacios. La secuencia termina con una línea que contiene únicamente un punto (`.`), la cual no debe procesarse.

## Salida

Para cada caso, imprimir la frase luego de aplicar las abreviaturas óptimas. A continuación, imprimir un entero `N` con la cantidad de letras abreviadas y, en las siguientes `N` líneas, las asociaciones en el formato `c. = palabra`, ordenadas por la letra.

## Ejemplo de Entrada
```
hoje eu visitei meus pais
tive que lavar meu cachorro pois ele estava fedendo
.
```

## Ejemplo de Salida
```
h. eu v. m. p.
4
h. = hoje
m. = meus
p. = pais
v. = visitei
t. q. l. m. c. p. ele e. f.
8
c. = cachorro
e. = estava
f. = fedendo
l. = lavar
m. = meu
p. = pois
q. = que
t. = tive
```

## Análisis y Enfoque de Solución

Cada palabra aporta un beneficio potencial igual a `(longitud - 2) × frecuencia`, ya que al abreviarla se reduce a dos caracteres. Para cada letra inicial se debe seleccionar la palabra con mayor beneficio positivo; en caso de empates, se escoge la que aparece primero en el texto. Luego, se recorre de nuevo la frase conservando los separadores originales y sustituyendo las palabras elegidas por su abreviatura.

### Complejidad

El procesamiento de cada caso implica dos recorridos sobre la línea (longitud `L ≤ 10⁴`), por lo que la complejidad es `O(L)`. Las estructuras auxiliares por letra manejan a lo sumo todas las palabras de la línea, manteniendo el algoritmo eficiente.
