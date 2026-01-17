# Ataque a Gasparini - ID 3343

## Problema

¡El gran rey de isla Paradis es advertido de que los titanes están organizando un ataque!

Los titanes vienen en 3 tamaños, titanes de metro pequeño, titanes de metro mediano y titanes de metro grande. Varios titanes de diferentes tamaños están organizando un ataque, y el rey necesita construir varios muros de x metros para detenerlos.

Afortunadamente el rey conoce la estrategia de los titanes, atacarán en secuencia, uno tras otro. Cuando un titán de tamaño k se encuentra con una pared, ocurre una de dos situaciones:

- Si el muro es más grande o del mismo tamaño que el titán, destruye k metros del muro y luego se cansa y muere, por lo que el muro se vuelve k metros más bajo.
- Si la pared es más pequeña que el Titán, salta sobre esa pared y pasa a la siguiente.

El rey te pregunta, el consejero del rey, cuál es el menor número de muros que se deben construir antes del ataque para detener el ataque de los titanes.

## Entrada

En la primera línea seguirán 2 números enteros, n y x, separados por un espacio, que representan la cantidad de titanes que componen el ataque y el tamaño de los muros a construir.

En la segunda línea sigue una cadena de texto de tamaño n, compuesta por los caracteres p, m y g, que indican el tamaño (pequeño, mediano y grande respectivamente) del i-ésimo titán. Titán Ti+1 ataques despues de titan Ti.

En la tercera línea siguen 3 números enteros, p, m y g, que representan el tamaño de un titán pequeño, mediano y grande respectivamente.

### Restricciones
- 1 ≤ n ≤ 3 × 10^5
- 1 ≤ X ≤ 10^5
- 1 ≤ p ≤ m ≤ g ≤ x

## Salida

Según la descripción del problema, imprime la cantidad mínima de paredes para detener el ataque de los titanes.

## Ejemplos

### Ejemplo 1
**Entrada:**
```
3 20
MPG
3 8 10
```
**Salida:**
```
2
```

### Ejemplo 2
**Entrada:**
```
8 20
MGGPGGGP
3 8 10
```
**Salida:**
```
4
```

### Ejemplo 3
**Entrada:**
```
4 6
GPMP
3 4 5
```
**Salida:**
```
3
```

## Análisis

### Estrategia Intentada
El problema requiere una simulación del proceso de ataque de los titanes:

1. **Múltiples muros**: Mantenemos una lista de todos los muros construidos con sus alturas actuales
2. **Búsqueda secuencial**: Cada titán busca el primer muro (de izquierda a derecha) que pueda detenerlo
3. **Decisión por titán**:
   - Si encuentra un muro con `altura >= tamaño_titán`: Ataca ese muro, lo reduce en `tamaño_titán` y muere
   - Si todos los muros son `< tamaño_titán`: Salta todos los muros existentes y construimos un nuevo muro de tamaño `x`, que el titán ataca inmediatamente

### Ejemplo detallado (Caso 3)
- n=4, x=6, titans="GPMP", p=3, m=4, g=5
- G(5): No hay muros → construye muro[6], ataca → muros=[1]
- P(3): muro[0]=1 < 3, salta → construye muro[6], ataca → muros=[1, 3]
- M(4): muro[0]=1 < 4, salta. muro[1]=3 < 4, salta → construye muro[6], ataca → muros=[1, 3, 2]
- P(3): muro[0]=1 < 3, salta. muro[1]=3 >= 3, ataca → muros=[1, 0, 2]
- **Resultado: 3 muros**

### Complejidad
- **Tiempo**: O(n × m) donde m es el número de muros (peor caso O(n²))
- **Espacio**: O(m) para almacenar los muros

### Casos a considerar
- Los titanes pequeños pueden usar muros que titanes grandes saltaron
- Cada muro permanece en su posición aunque sea reducido a 0
- La búsqueda siempre es de izquierda a derecha por todos los muros existentes

---

## ⚠️ PROBLEMA NO RESUELTO

Este problema **NO fue resuelto exitosamente**. Se intentaron múltiples enfoques pero ninguno pasó todos los casos de prueba:

### Intentos realizados:

1. **Simulación Simple (O(n))**: Solo considerar el último muro construido
   - **Resultado**: Wrong Answer - Devolvía menos muros de los necesarios

2. **Simulación Completa con Vector (O(n²))**: Buscar en todos los muros para cada titán
   - **Resultado**: Time Limit Exceeded

3. **Optimización con Multiset (O(n log n))**: Usar árbol balanceado para búsqueda eficiente
   - **Resultado**: Wrong Answer - Devolvía resultados incorrectos en algunos casos

4. **Optimización con Índice (O(n×m) optimizado)**: Mantener índice del primer muro activo
   - **Resultado**: Time Limit Exceeded

### Conclusión
No se logró encontrar una solución que combine correctitud y eficiencia suficiente para pasar todos los casos de prueba dentro del límite de tiempo.
