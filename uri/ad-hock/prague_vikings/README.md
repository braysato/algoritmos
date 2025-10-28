# 1816 - Prague Vikings

## Problema

Se han descubierto rastros de una antigua civilización vikinga alrededor de Praga, y se encontró una gran cantidad de material impreso cerca del sitio arqueológico. Como era de esperar, la lectura de este material resultó ser ardua y desafiante, ya que esta civilización usaba un esquema de codificación de texto para evitar que su conocimiento fuera usado por sus rivales.

Recientemente, investigadores checos anunciaron con gran emoción a la prensa la comprensión del mecanismo de codificación usado por esos vikingos. Según los investigadores, el alfabeto vikingo estaba compuesto por las letras de A a Z (incluyendo las letras K, W y Y).

La codificación se realizaba de la siguiente manera. Inicialmente, se construía una lista donde la letra A aparecía en la primera posición, la letra B en la segunda, y así sucesivamente, con las letras siguiendo el mismo orden de nuestro alfabeto. Luego, el texto a codificar se recorría de izquierda a derecha y, para cada letra l encontrada, se imprimía el número de su posición en la lista y l se movía al inicio de la lista.

**Ejemplo:** La codificación vikinga para el mensaje `A B B B A A B B B B A C C A B B A A A B C` fue dada por la secuencia `1 2 1 1 2 1 2 1 1 1 2 3 1 2 3 1 2 1 1 2 3`

**Entrada:**
- Múltiples instancias
- Cada instancia: primera línea tiene m (0 ≤ m ≤ 10000) - número de enteros codificados
- Siguiente línea: m enteros (1 ≤ valor ≤ 26)
- m = 0 indica el fin de la entrada

**Salida:**
- Para cada instancia: `Instancia h` donde h comienza en 1
- Siguiente línea: texto decodificado
- Línea en blanco después de cada instancia

## Análisis del Problema

**Mecánica de codificación:**
1. Comenzamos con una lista ordenada: [A, B, C, D, ..., Z]
2. Para cada letra en el texto original:
   - Encontramos su posición en la lista (1-indexed)
   - Imprimimos esa posición
   - Movemos la letra al inicio de la lista

**Para decodificar:**
1. Comenzamos con la lista ordenada: [A, B, C, D, ..., Z]
2. Para cada número en la secuencia codificada:
   - El número indica la posición de la letra en la lista actual
   - Extraemos esa letra
   - Movemos la letra al inicio de la lista

**Ejemplo paso a paso:**
```
Lista inicial: [A, B, C, D, E, ...]
Número: 1 → letra A (pos 1), mover A al inicio → [A, B, C, D, E, ...] (no cambia)
Número: 2 → letra B (pos 2), mover B al inicio → [B, A, C, D, E, ...]
Número: 1 → letra B (pos 1), mover B al inicio → [B, A, C, D, E, ...] (no cambia)
Número: 1 → letra B (pos 1), mover B al inicio → [B, A, C, D, E, ...] (no cambia)
Número: 2 → letra A (pos 2), mover A al inicio → [A, B, C, D, E, ...]
...
```

## Enfoque de Solución

1. Mantener una lista dinámica del alfabeto
2. Para cada número en la secuencia codificada:
   - Acceder al carácter en la posición indicada (1-indexed)
   - Agregar ese carácter al resultado
   - Eliminar el carácter de su posición actual
   - Insertar el carácter al inicio de la lista
3. Imprimir el texto decodificado

**Estructura de datos:** Vector/lista para mantener el alfabeto dinámico

## Complejidad

- **Tiempo:** O(m × 26) donde m es el número de caracteres codificados
  - Por cada carácter: acceso O(1), eliminación O(26), inserción O(26)
- **Espacio:** O(26) para la lista del alfabeto


