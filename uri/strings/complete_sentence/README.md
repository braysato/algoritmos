# Complete Sentence

## Descripción del Problema

Tu profesor de inglés le encanta traer cosas nuevas a la clase, y hoy no fue diferente. Hay una ciudad, según tu profesor, donde la gente se toma muy en serio la forma en que se hablan entre sí. En particular, cuando dos personas están hablando, piensan mucho en la oración que van a decir antes de decirla, para que puedan asegurar que su oración será una "frase completa", o tal vez una "frase casi completa".

Considerando nuestro alfabeto de 26 letras, una oración es "completa" si, y solo si, tiene todas las letras de nuestro alfabeto en ella. De manera similar, una oración es "casi completa" si, y solo si, no es "completa", pero tiene al menos la mitad de las letras de nuestro alfabeto en ella. Cuando una oración no es "completa" ni "casi completa", está "mal elaborada".

Tu profesor te dio una tarea muy difícil: dadas varias oraciones intercambiadas entre varias personas de la ciudad citada, di en cuál de las categorías dadas encaja cada oración.

## Entrada

La primera línea contiene un entero N, indicando el número de casos de prueba a seguir.

Cada caso de prueba contiene una línea, conteniendo letras minúsculas, espacios en blanco y/o comas. El número de caracteres de cada línea es al menos 3 y como máximo 1000, contando los espacios.

## Salida

Para cada caso de prueba, imprima una línea que contenga una de las siguientes oraciones: "frase completa", cuando la oración se considera completa; "frase quase completa", cuando la oración no se considera completa, pero se considera casi completa; o "frase mal elaborada", cuando la oración no es completa ni casi completa.

## Ejemplo de Entrada
```
2
ola, como voce esta hoje
hoje fui na feira, e comprei banana, melao e abacates
```

## Ejemplo de Salida
```
frase mal elaborada
frase quase completa
```

## Análisis y Enfoque de Solución

Este es un problema de **análisis de texto** que requiere contar letras únicas en una oración.

### Criterios de Clasificación

Dado un alfabeto de 26 letras:
- **Frase completa**: Contiene las 26 letras del alfabeto
- **Frase casi completa**: No es completa, pero contiene al menos 13 letras (la mitad de 26)
- **Frase mal elaborada**: Contiene menos de 13 letras diferentes

### Estrategia de Solución

1. Para cada caso de prueba:
   - Leer la línea completa
   - Contar cuántas letras diferentes del alfabeto aparecen en la línea
   - Clasificar según el conteo:
     - Si count == 26 → "frase completa"
     - Si count >= 13 → "frase quase completa"
     - Si count < 13 → "frase mal elaborada"

2. Para contar letras únicas:
   - Usar un conjunto (set) o arreglo booleano
   - Recorrer cada carácter de la línea
   - Si es una letra (a-z), marcarla como vista
   - Contar cuántas letras únicas fueron vistas

### Implementación

```
Para cada caso:
  1. Leer la línea
  2. Inicializar conjunto de letras vistas
  3. Para cada carácter en la línea:
     - Si es letra minúscula, agregar al conjunto
  4. Contar tamaño del conjunto
  5. Clasificar y mostrar resultado
```

### Ejemplo

**Caso 1:** "ola, como voce esta hoje"
- Letras únicas: o, l, a, c, m, v, e, s, t, h, j = 11 letras
- 11 < 13 → "frase mal elaborada"

**Caso 2:** "hoje fui na feira, e comprei banana, melao e abacates"
- Letras únicas: h, o, j, e, f, u, i, n, a, r, c, m, p, b, l, t, s = 17 letras
- 17 >= 13 (pero < 26) → "frase quase completa"

### Complejidad

- **Tiempo**: O(N × M) donde N es el número de casos y M es la longitud promedio de las frases
- **Espacio**: O(1) - el conjunto de letras tiene tamaño fijo (máximo 26)
