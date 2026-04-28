# Hall of Murderers - 1861

## Enunciado

Se recibe una lista de asesinatos. Cada línea contiene dos nombres:
- el nombre del asesino
- el nombre de la víctima

La lista no está en orden lexicográfico ni cronológico. Se pide contar cuántas personas mató cada asesino.

### Entrada
Cada línea de la entrada describe un asesinato con:
- `murderer victim`

Reglas:
- Cada nombre tiene entre 1 y 10 caracteres.
- El primer carácter es mayúscula y el resto minúsculas.
- Hay entre 1 y 10^5 líneas.
- La entrada termina en EOF.

### Salida
- La primera línea debe ser exactamente:
  `HALL OF MURDERERS`
- Luego, una línea por asesino con:
  `name count`
- El listado debe ir en orden lexicográfico.
- Si un asesino también fue asesinado, no debe aparecer en la salida.

### Ejemplo de Entrada
```
Arya Meryn
Meryn Syrio
Brienne Stannis
Ellaria Myrcella
Jaime Aerys
Brienne Jaime
```

### Ejemplo de Salida
```
HALL OF MURDERERS
Arya 1
Brienne 2
Ellaria 1
```

## Análisis

- Necesitamos dos estructuras:
1. Conteo de asesinatos por nombre.
2. Conjunto de personas asesinadas.

- Por cada línea `(killer, victim)`:
1. Incrementamos el conteo de `killer`.
2. Insertamos `victim` en el conjunto de asesinados.

- Al final, recorremos los asesinos en orden lexicográfico y solo imprimimos aquellos que **no** estén en el conjunto de asesinados.

## Complejidad

- Tiempo: O(N log N)
- Espacio: O(N)

Donde N es la cantidad de líneas de entrada.