# Imagen - ID 1516

## Problema

Rafael encontró un nuevo hobby: dibujar haciendo uso de caracteres del teclado. A pesar de que este nuevo tipo de arte sea simple y limitado, la creatividad es suficiente para realizar dibujos muy interesantes.

Luego de hacer varios dibujos, Rafael se preguntó cómo serían si les cambiaría el tamaño, pero como tener que rehacer todos los dibujos sería muy cansador, el necesitará de tu ayuda.

Cuando se quiere cambiar el tamaño, una imagen de N líneas y M columnas tendrá A líneas y B columnas, y, dado que las nuevas dimensiones son más grandes que las dimensiones originales, algunos caracteres tendrán que repetirse.

Digamos que A es 3 veces más grande que N. En este caso, cada línea tendrá que repetirse 3 veces, tal que se cambie el tamaño de la imagen correctamente.

Dado un dibujo realizado por Rafael, deberás imprimir cómo sería el dibujo si cambiaria su tamaño con las nuevas dimensiones dadas.

### Entrada

La entrada consistirá de varios casos de prueba. Cada caso comienza con dos enteros N y M (1 ≤ N, M ≤ 50), representando respectivamente, la altura y el ancho del dibujo de Rafael.

Luego habrán N líneas, cada una conteniendo M caracteres, representando el dibujo hecho por Rafael. Luego seguirán dos enteros A y B (N < A ≤ 100, M < B ≤ 100, A es múltiplo de N, y B es múltiplo de M), representando respectivamente, la nueva altura y ancho que Rafael quiere que su dibujo tenga.

El último caso estará dado por N = M = 0, que no debe ser procesado.

### Salida

Para cada caso, deberá imprimir A líneas, conteniendo B caracteres cada una, representando el dibujo de Rafael con las nuevas dimensiones.

Luego de cada caso, imprima una línea en blanco.

### Ejemplo de entrada
```
3 3
###
#__
###
6 9
0 0
```

### Ejemplo de salida
```
#########
#########
###______
###______
#########
#########

```

## Análisis

### Comprensión del problema
- Se tiene una imagen ASCII de N×M caracteres
- Se debe redimensionar a A×B caracteres, donde A y B son múltiplos de N y M respectivamente
- Cada carácter debe replicarse proporcionalmente en ambas dimensiones

### Estrategia de solución

1. **Factores de escala:**
   - Factor vertical: `factorV = A / N`
   - Factor horizontal: `factorH = B / M`

2. **Proceso de escalado:**
   - Para cada línea del dibujo original:
     - Expandir horizontalmente: cada carácter se repite `factorH` veces
     - Repetir la línea expandida `factorV` veces verticalmente

3. **Estructura del programa:**
   - Leer N y M en un bucle
   - Si N = M = 0, terminar
   - Leer las N líneas del dibujo
   - Leer A y B
   - Aplicar el escalado y mostrar el resultado
   - Imprimir línea en blanco

### Complejidad
- Tiempo: O(A × B) por cada caso de prueba
- Espacio: O(N × M) para almacenar la imagen original

### Casos especiales
- Factores de escala mínimos (A = N+1, B = M+1 no son válidos, deben ser múltiplos)
- Imagen de 1×1 carácter
- Factores de escala grandes
