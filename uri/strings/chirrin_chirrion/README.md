# Chirrin Chirrion

## Descripción del Problema

Tausfo recibió del demonio Mephistophetrist una herramienta llamada Chirrin Chirrion que permite invocar o expulsar objetos según se pronuncien las palabras clave. Si después del nombre de un objeto se dice "chirrin", el objeto aparece siempre que no lo posea ya. Si después del nombre se dice "chirrion", el objeto desaparece siempre que lo posea. Cualquier otra palabra no genera efectos. Mephistophetrist exige recuperar todo lo que Tausfo obtuvo con la herramienta, por lo que se debe determinar qué objetos conserva tras una secuencia de usos.

## Entrada

- Un entero `C` que indica el número de casos de prueba.
- Para cada caso:
  - Un entero `N` con la cantidad de usos registrados.
  - `N` líneas, cada una con dos palabras: el nombre del objeto y la palabra indicada (`chirrin` o `chirrion`).

## Salida

Para cada caso, se debe imprimir la palabra `TOTAL` en una línea y, a continuación, los objetos que Tausfo conserva, uno por línea, en orden alfabético. Si no conserva nada, solo se imprime `TOTAL`.

## Ejemplo de Entrada
```
2
4
sapo chirrion
bala charrin
vela chirrin
copo chirrin
3
galo chirrin
galo chirrion
raposa chirrin
```

## Ejemplo de Salida
```
TOTAL
copo
vela
TOTAL
raposa
```

## Análisis y Enfoque de Solución

Cada caso se resuelve manteniendo un conjunto de cadenas que representan los objetos en posesión de Tausfo. Para cada comando:

- Si la palabra es `chirrin` y el objeto no se encuentra en el conjunto, se inserta.
- Si la palabra es `chirrion` y el objeto pertenece al conjunto, se elimina.
- En cualquier otro caso la instrucción no tiene efecto.

Al finalizar los `N` comandos, se ordenan alfabéticamente los elementos del conjunto y se imprimen. La complejidad por caso es `O(N log N)` debido al costo de mantenimiento y ordenación del conjunto.
