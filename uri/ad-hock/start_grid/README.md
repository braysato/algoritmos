# Start Grid

## Descripción del Problema

Nlogonia celebrará la sensacional final mundial del campeonato de pilotos de Fórmula 17. Los competidores se alinean en la salida y compiten en la carrera. Tendrás acceso a las parrillas de salida y llegada. La pregunta es determinar el número mínimo de adelantamientos que se realizaron durante la competencia.

## Entrada

La entrada contiene varios casos de prueba. Cada caso de prueba usa tres líneas. La primera línea de un caso de prueba contiene un entero N (2 ≤ N ≤ 24) que indica el número de competidores. Cada competidor se identifica con un número del 1 al N. La segunda línea contiene los N competidores en orden de la parrilla de salida. La tercera línea de cada caso tiene los mismos competidores, pero ahora en el orden en que terminan la carrera.

## Salida

Para cada caso de prueba, imprime una línea que contenga un solo entero, el número mínimo de adelantamientos necesarios para pasar de la parrilla de salida a la parrilla de llegada.

## Ejemplo de Entrada
```
5
1 2 3 4 5
3 1 2 5 4
5
3 1 2 5 4
1 2 3 4 5
5
3 1 2 5 4
5 3 2 1 4
```

## Ejemplo de Salida
```
3
3
4
```

## Análisis y Enfoque de Solución

Este problema trata de contar **inversiones** entre dos permutaciones. Un adelantamiento ocurre cuando un competidor que estaba detrás de otro en la parrilla de salida termina adelante en la llegada.

### Observaciones Clave

1. **Un adelantamiento es una inversión**: Si el competidor A estaba detrás de B al inicio y A termina adelante de B al final, hubo al menos un adelantamiento
2. **Contamos pares invertidos**: Para cada par de competidores, si su orden relativo cambió, contamos un adelantamiento
3. **Mapeo de posiciones**: Necesitamos saber la posición inicial de cada competidor para comparar con su posición final

### Estrategia de Solución

1. **Crear un mapeo** de cada competidor a su posición en la parrilla de salida
2. **Para cada par de competidores** en la parrilla de llegada:
   - Si el competidor que aparece primero en la llegada tenía una posición inicial mayor (estaba más atrás) que el que aparece después
   - Entonces hubo un adelantamiento
3. **Contar todas las inversiones**

### Algoritmo Detallado

```
1. Leer N
2. Leer parrilla de salida y crear mapa: competidor -> posición_inicial
3. Leer parrilla de llegada
4. contador = 0
5. Para cada i de 0 a N-1:
   Para cada j de i+1 a N-1:
     Si posición_inicial[llegada[i]] > posición_inicial[llegada[j]]:
       contador++
6. Imprimir contador
```

### Ejemplo

Salida: `1 2 3 4 5`
Llegada: `3 1 2 5 4`

Posiciones iniciales:
- 1 → 0, 2 → 1, 3 → 2, 4 → 3, 5 → 4

Comparaciones en orden de llegada:
- 3 (pos 2) vs 1 (pos 0): 2 > 0 ✓ (inversión) 
- 3 (pos 2) vs 2 (pos 1): 2 > 1 ✓ (inversión)
- 3 (pos 2) vs 5 (pos 4): 2 < 4 (no inversión)
- 3 (pos 2) vs 4 (pos 3): 2 < 3 (no inversión)
- 1 (pos 0) vs 2 (pos 1): 0 < 1 (no inversión)
- 1 (pos 0) vs 5 (pos 4): 0 < 4 (no inversión)
- 1 (pos 0) vs 4 (pos 3): 0 < 3 (no inversión)
- 2 (pos 1) vs 5 (pos 4): 1 < 4 (no inversión)
- 2 (pos 1) vs 4 (pos 3): 1 < 3 (no inversión)
- 5 (pos 4) vs 4 (pos 3): 4 > 3 ✓ (inversión)

Total: 3 adelantamientos

### Complejidad

- **Tiempo**: O(N²) por contar todos los pares
- **Espacio**: O(N) para el mapeo de posiciones
