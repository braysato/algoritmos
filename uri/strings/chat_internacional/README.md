# Chat Internacional - 1581

## Problema

Rafael ganó recientemente una beca y está estudiando fuera de Brasil, donde conoció mucha gente de muchas nacionalidades diferentes. La lengua nativa de éste país es el inglés, y toda la gente que Rafael conoció habla inglés como su lengua primaria o secundaria.

Como aprender una segunda lengua es una tarea difícil y agotadora, algunas personas prefieren hablar en su lengua nativa cuando sea posible. Una excepción a ésta regla es cuando hay dos personas en el grupo que no tienen la misma lengua nativa. En éste tipo de situaciones, la lengua hablada por ellos es el inglés.

Por ejemplo, si hay un grupo con sólo brasileros, la lengua hablada será el portugués, pero si hay un español entre ellos, la lengua hablada será el inglés.

Rafael a veces se confunde sobre qué lengua debería ser hablada en cada grupo de personas, y entonces él pide tu ayuda.

## Entrada

La primera línea contendrá un entero N, que representa el número de casos de prueba.

Cada caso de prueba empieza con un entero K (2 ≤ K ≤ 100), que representa el número de personas en el grupo. Seguidamente, habrá K líneas donde cada una contiene una cadena de caracteres S, que representa la lengua nativa de las K personas.

Cada cadena tendrá entre 1 y 20 caracteres, con letras sólo en minúscula (a-z).

## Salida

Imprimir una línea, que contenga una cadena S, que represente la lengua más apropiada para la situación.

## Ejemplo de Entrada

```
2
3
portugues
chines
portugues
2
espanhol
espanhol
```

## Ejemplo de Salida

```
ingles
espanhol
```

## Análisis

- Si todas las personas del grupo tienen la misma lengua nativa, se habla esa lengua.
- Si hay al menos dos personas con lenguas nativas diferentes, se habla "ingles".

### Estrategia

1. Leer la primera lengua como referencia.
2. Comparar las demás lenguas con la primera.
3. Si alguna es diferente, el resultado es "ingles".
4. Si todas son iguales, el resultado es la lengua común.
