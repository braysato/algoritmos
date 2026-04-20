# Marte Necesita Matemáticos - 1804

## Enunciado

En el LAGOS 2015 en Beberibe, Ceará, los organizadores contrataron buggies para llevar a los investigadores a puntos turísticos. Los buggies salieron del hotel formando una fila, uno detrás del otro. Alienígenas marcianos comenzaron a abducir algunos buggies con todas las personas a bordo. Cada conductor quiere saber cuántas personas quedan en los buggies delante de él.

### Entrada
Un entero N (1 ≤ N ≤ 10⁵) representa el número de buggies. La segunda línea tiene N enteros pi (1 ≤ pi ≤ 5) representando personas en el buggy i. Las líneas siguientes contienen:
- `a i`: abducción del buggy i (aún no abducido)
- `? i`: el conductor del buggy i (no abducido) quiere saber cuántas personas quedan detrás de su buggy

### Salida
Para cada `? i`, imprimir cuántas personas quedan detrás del buggy i.

### Ejemplo de Entrada
```
10
1 2 3 4 5 4 3 2 1 2
a 9
? 10
a 2
a 5
? 6
a 6
? 10
```

### Ejemplo de Salida
```
24
8
13
```

## Análisis

- "Detrás del buggy i" = buggies con índice < i que no han sido abducidos.
- Verificación: `? 10` tras abducir 9: suma de buggies 1-8 = 1+2+3+4+5+4+3+2 = 24 ✓
- Operaciones: actualización puntual (abducir) y consulta de suma prefijo → **Fenwick Tree (BIT)**.
- `a i`: restar p[i] de la posición i
- `? i`: consultar suma del prefijo [1, i-1]

## Complejidad

- Tiempo: O((N + Q) · log N)
- Espacio: O(N)
