# 1483 - Animal Game

## Problema

En un país no muy lejano, las personas son adictas a un juego de apuestas simple basado en números llamado "Animal Game" (Juego de Animales). Los números se dividen en 25 grupos dependiendo del valor de los dos últimos dígitos (decenas y unidades). Cada grupo está asociado con un animal:

- Primer grupo (burro): números 01, 02, 03, 04
- Segundo grupo (águila): números 05, 06, 07, 08
- ... y así sucesivamente hasta el último grupo: números 97, 98, 99, 00

### Reglas del Juego

El jugador decide el valor de la apuesta **V** y un número **N** (0 ≤ N ≤ 1000000). Cada día se sortea un número **M** (0 ≤ M ≤ 1000000).

**El premio se calcula de la siguiente forma:**

1. Si **M** y **N** tienen los mismos **4 últimos dígitos** (miles, centenas, decenas, unidades) → Premio: **V × 3000**
   - Ejemplo: N = 99301 y M = 19301

2. Si **M** y **N** tienen los mismos **3 últimos dígitos** (centenas, decenas, unidades) → Premio: **V × 500**
   - Ejemplo: N = 38944 y M = 83944

3. Si **M** y **N** tienen los mismos **2 últimos dígitos** (decenas, unidades) → Premio: **V × 50**
   - Ejemplo: N = 111 y M = 552211

4. Si **M** y **N** tienen los **2 últimos dígitos en el mismo grupo** (mismo animal) → Premio: **V × 16**
   - Ejemplo: N = 82197 y M = 337600

5. Si ninguna de las anteriores ocurre → Premio: **0.00**

**Notas importantes:**
- El premio dado es el **máximo posible** según las reglas
- **No es posible acumular premios**, solo se aplica un criterio
- Si N o M tienen menos de 4 dígitos, se asume que se agregan ceros al frente (ej: 17 → 0017)

### Entrada

La entrada contiene varios casos de prueba. Cada caso consiste en una línea con:
- Un número real **V**: monto apostado con dos decimales (0.01 ≤ V ≤ 1000.00)
- Un entero **N**: número elegido para la apuesta (0 ≤ N ≤ 1000000)
- Un entero **M**: número sorteado (0 ≤ M ≤ 1000000)

El fin de la entrada se indica con una línea conteniendo V = M = N = 0.

### Salida

Para cada caso de prueba, imprimir una línea con un número real con dos decimales, representando el premio correspondiente a la apuesta dada.

### Ejemplo de Entrada
```
32.20 32 213929
10.50 32 213032
2000.00 340000 0
520.00 874675 928567
10.00 1111 578311
0 0 0
```

### Ejemplo de Salida
```
515.20
5250.00
6000000.00
0.00
500.00
```

## Análisis del Problema

### Comprensión
- Debemos comparar los últimos dígitos de dos números N y M
- Calcular el premio basado en cuántos dígitos coinciden (4, 3, 2 o mismo grupo)
- Aplicar el multiplicador correspondiente al valor de la apuesta

### Observaciones Clave
1. **Prioridad**: Se debe aplicar el premio **máximo** (verificar en orden: 4 dígitos → 3 dígitos → 2 dígitos → mismo grupo)
2. **Grupos de animales**: Cada grupo tiene 4 números consecutivos (01-04, 05-08, ..., 97-00)
3. **Formato de salida**: Siempre con dos decimales
4. **Números con menos de 4 dígitos**: Se completan con ceros a la izquierda

### Cálculo de Grupos
Para determinar si dos números están en el mismo grupo (mismo animal):
- Extraer los 2 últimos dígitos de N y M
- Calcular el grupo: `grupo = (ultimos_2_digitos - 1) / 4`
- Si ambos números están en el mismo grupo → Premio: V × 16

**Casos especiales:**
- El número 00 pertenece al grupo 24 (último grupo, con 97, 98, 99)

