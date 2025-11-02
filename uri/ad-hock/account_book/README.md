# Account Book

## Descripción del Problema

La FCC (Fundación para el Combate de la Corrupción) desmanteló un importante esquema de corrupción en Nlogonia. Durante la operación, varios libros de cuentas, con notas que documentaban las transacciones ilícitas llevadas a cabo por el esquema, fueron incautados por agentes de la FCC.

Cada página de los libros de cuentas contiene algunas transacciones (ingresos o gastos, en nilogos, la moneda local de Nlogônia, cuyo símbolo es N$) y el flujo de caja resultante de las transacciones en esa página. Por ejemplo, si una página registró las siguientes transacciones: un ingreso de N$ 7, un ingreso de N$ 2, un gasto de N$ 3, un ingreso de N$ 1 y un gasto de N$ 11, el flujo de caja en esa página sería 7 + 2 - 3 + 1 - 11 = -4.

Sin embargo, para obstruir el trabajo de la policía, los delincuentes no registraron en sus libros de cuentas el tipo de cada transacción (ingreso o gasto). En el ejemplo anterior, la página contendría solo los números 7, 2, 3, 1 y 11 (sin indicación de si eran transacciones de ingreso o gasto). El flujo de caja para cada página, sin embargo, siempre se registró normalmente, con la señal (en este caso, -4).

Para garantizar que los delincuentes sean condenados, los fiscales deben poder determinar con certeza si cada transacción es un ingreso o un gasto. En el ejemplo anterior, la transacción N$ 7 fue ciertamente un ingreso, y la transacción N$ 11 fue ciertamente un gasto. Pero no podemos decir nada sobre las transacciones N$ 2, N$ 3 y N$ 1. Las transacciones N$ 2 y N$ 1 podrían haber sido ingresos y en este caso la transacción N$ 3 habría sido un gasto; o N$ 1 y N$ 2 podrían haber sido gastos y en este caso la transacción N$ 3 sería un ingreso.

Muchos libros de cuentas tienen un número relativamente grande de páginas, con muchas transacciones, lo que dificulta que la policía procese toda la información. Por lo tanto, necesitan un programa que realice la tarea de manera eficiente.

## Entrada

La entrada consiste en varios casos de prueba. La primera línea de un caso de prueba contiene dos enteros N y F, que indican el número de transacciones en la página (2 ≤ N ≤ 40) y el flujo de caja para esta página (−16000 ≤ F ≤ 16000). Cada una de las siguientes N líneas contiene un entero Ti que indica el valor de la i-ésima transacción (1 ≤ Ti ≤ 1000).

El último caso de prueba es seguido por una línea que contiene solo dos ceros separados por un espacio.

## Salida

Para cada caso de prueba de entrada, su programa debe imprimir una sola línea con N caracteres. El i-ésimo carácter debe ser '+', si es posible determinar con certeza que la i-ésima transacción es un ingreso, '-', si es posible determinar con certeza que la i-ésima operación es un gasto, y '?', si es imposible determinar con certeza el tipo de transacción. Si el flujo de caja registrado en la página no se puede obtener de las transacciones registradas en la página, su programa debe imprimir una sola línea que contenga el carácter '*'.

## Ejemplo de Entrada
```
5 7
1
2
3
4
5
4 15
3
5
7
11
5 12
6
7
7
1
7
0 0
```

## Ejemplo de Salida
```
?+??+
*
+??-?
```

## Análisis y Enfoque de Solución

Este problema se resuelve utilizando **programación dinámica con subset sum** combinado con análisis de obligatoriedad de elementos.

### Modelo del Problema
- Cada transacción puede ser positiva (ingreso, +) o negativa (gasto, -)
- Necesitamos encontrar todas las formas posibles de asignar signos a las transacciones para que la suma sea igual a F

### Estrategia de Solución

1. **Detección de soluciones válidas**: Usar programación dinámica para determinar si existe al menos una forma de obtener el flujo F. Si no existe ninguna, retornar '*'.

2. **Determinación de certeza por transacción**: Para cada transacción i:
   - Intentar asignarla como ingreso (+) y verificar si el resto de transacciones pueden alcanzar el flujo restante
   - Intentar asignarla como gasto (-) y verificar si el resto de transacciones pueden alcanzar el flujo restante
   - Si solo una de las dos opciones es posible → podemos determinar con certeza el tipo de transacción ('+' o '-')
   - Si ambas opciones son posibles → es indeterminado ('?')

3. **Subset Sum con DP**: Para verificar si un conjunto de transacciones puede alcanzar un valor objetivo, usamos un enfoque donde cada elemento puede sumar o restar. Utilizamos un arreglo booleano que indica qué valores son alcanzables con un subconjunto de transacciones.