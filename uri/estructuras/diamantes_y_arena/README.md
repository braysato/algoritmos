# Diamantes y Arena - 1069

## Problema

John está trabajando en una mina de diamante, tratando de extraer la mayor cantidad de diamantes "<>". Él debe evitar todas las partículas de arena encontrada "." en este proceso y luego el diamante se puede extraer y nuevos diamantes se pueden formar. Si tiene como entrada `<... << .. >> ....> .... >>>` tres diamantes se han formado. El primero se toma de `<..>` resultando `<... <> ....> .... >>>`. El segundo diamante es entonces retirado, dejando `<.......> .... >>>`. El tercer diamante es entonces retirado, dejando al final `..... >>>` con la posibilidad de extraer nuevos diamantes.

### Entrada

Leer un entero N que es el número de casos de prueba. Entonces siguen las N líneas, cada una hasta 1000 caracteres, incluyendo "<", ">" y "."

### Salida

Debes imprimir la cantidad de diamantes que se han podido extraer en cada caso de prueba.

### Ejemplo de Entrada
```
2
<..><.<..>>
<<<..<......<<<<....>
```

### Ejemplo de Salida
```
3
1
```

## Análisis

1. **Estructura de pila**: Cada vez que encontramos un `<`, lo agregamos a la pila.
2. **Emparejamiento**: Cuando encontramos un `>`, si hay un `<` en la pila, sacamos uno y contamos un diamante.
3. **Arena**: Los `.` se ignoran completamente.
4. **Greedy**: Siempre emparejamos con el `<` más cercano (último en la pila).

## Complejidad

- Tiempo: O(N * L) donde L es la longitud de cada línea
- Espacio: O(L) para la pila en el peor caso
