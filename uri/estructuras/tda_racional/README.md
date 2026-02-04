# TDA Racional - 1022

## Problema

Te invitaron a hacer un pequeño trabajo para tu Profesor de Matemática. El trabajo consiste en leer una expresión matemática en formato de dos números racionales (numerador / denominador) y mostrar el resultado de la operación. Cada operando o operador está separado por un espacio en blanco. La secuencia de entrada (cada línea) debe respetar el siguiente formato: número, (caracter '/'), número, caracter operación ('/', '*', '+', '-'), número, (caracter '/'), número. La respuesta debe ser mostrada seguida por un operador '=' y la respuesta simplificada. Si la respuesta no puede ser simplificada, debe ser repetida después de un operador '='.

Considerando N1 y D1 como numerador y denominador de la primera fracción, siga las indicaciones para hacer cada una de estas 4 operaciones:

- Suma: (N1*D2 + N2*D1) / (D1*D2)
- Resta: (N1*D2 - N2*D1) / (D1*D2)
- Multiplicación: (N1*N2) / (D1*D2)
- División: (N1/D1) / (N2/D2), esto es (N1*D2)/(N2*D1)

### Entrada

La entrada contiene varios casos de prueba. El primer valor es un entero N (1 ≤ N ≤ 1*10^4), indicando la cantidad de casos de prueba que deben ser leídos. Cada caso de prueba contiene un valor racional X (1 ≤ X ≤ 1000), una operación (-, +, * or /) y otro valor racional Y (1 ≤ Y ≤ 1000).

### Salida

La salida debe ser un número racional, seguido por un signo "=" y otro número racional, que es la simplificación del primer valor. En caso que el primer valor no pueda ser simplificado, se debe repetir el mismo valor después del signo '='.

### Ejemplo de Entrada
```
4
1 / 2 + 3 / 4
1 / 2 - 3 / 4
2 / 3 * 6 / 6
1 / 2 / 3 / 4
```

### Ejemplo de Salida
```
10/8 = 5/4
-2/8 = -1/4
12/18 = 2/3
4/6 = 2/3
```

## Análisis

1. **Operaciones**: Aplicar las fórmulas dadas para suma, resta, multiplicación y división de fracciones.
2. **Simplificación**: Calcular el MCD (máximo común divisor) del numerador y denominador, luego dividir ambos por el MCD.
3. **Signos**: Manejar correctamente los signos negativos. El signo negativo debe ir en el numerador.

## Complejidad

- Tiempo: O(N * log(max_valor)) por el cálculo del MCD
- Espacio: O(1)
