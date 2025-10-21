# 1593 - Binary Function

## Problema

Definimos la **paridad** de un entero como la suma de sus bits en forma binaria módulo dos. Por ejemplo, el número 21₁₀ = 10101₂ tiene tres 1's en su representación binaria y por lo tanto tiene una paridad impar.

En este problema, necesitas calcular el **número de bits 1** en un entero I dado en la entrada, es decir, calcular el número de 1's en la representación binaria.

### Entrada

La primera línea contiene un entero T (T ≤ 100) indicando el número de casos de prueba.

Para cada caso, habrá una sola línea con el número I:
- 1 ≤ I < 10¹⁸ (para aproximadamente el 90% de los casos)
- 1 ≤ I < 10¹⁰⁰⁰ (para los otros casos de prueba)

El número de entrada no tendrá ceros a la izquierda.

### Salida

Imprimir el número de 1's en la representación binaria para cada caso en una sola línea.

### Ejemplo de Entrada
```
3
21
3
123456789123456789123456789
```

### Ejemplo de Salida
```
3
2
50
```

## Análisis del Problema

### Comprensión
- Debemos contar cuántos bits 1 tiene un número en su representación binaria
- El problema tiene dos tipos de casos:
  - **90% de casos:** Números que caben en un `long long` (< 10¹⁸)
  - **10% de casos:** Números muy grandes que NO caben en tipos primitivos (< 10¹⁰⁰⁰)

### Observaciones Clave

1. **Números pequeños:** Para números < 10¹⁸, podemos usar operaciones bit a bit estándar
2. **Números grandes:** Para números gigantes (hasta 10¹⁰⁰⁰), debemos trabajar con el número como **string** y simular la conversión a binario
3. **Conversión a binario:** Dividir repetidamente entre 2 y contar los residuos que sean 1

### Ejemplos Explicados

**Ejemplo 1:** 21
- 21₁₀ = 10101₂
- Número de 1's: **3**

**Ejemplo 2:** 3
- 3₁₀ = 11₂
- Número de 1's: **2**

**Ejemplo 3:** 123456789123456789123456789
- Este es un número muy grande (27 dígitos)
- En binario tiene **50** bits en 1

### Pasos para Resolver

**Para números pequeños (< 10¹⁸):**
1. Leer el número como `long long`
2. Usar operaciones bit a bit para contar 1's
3. Método: Mientras el número sea > 0:
   - Si `n & 1 == 1`, incrementar contador
   - Hacer `n >>= 1` (shift a la derecha)

**Para números grandes (≥ 10¹⁸):**
1. Leer el número como **string**
2. Simular la división entre 2 manualmente:
   - Para cada división, el residuo indica si hay un bit 1
   - Continuar dividiendo hasta llegar a 0
3. Contar cuántos residuos fueron 1

### Conversión Manual a Binario (para strings)

```
Ejemplo: "21"
21 ÷ 2 = 10, residuo 1 → bit 1
10 ÷ 2 = 5,  residuo 0 → bit 0
 5 ÷ 2 = 2,  residuo 1 → bit 1
 2 ÷ 2 = 1,  residuo 0 → bit 0
 1 ÷ 2 = 0,  residuo 1 → bit 1

Binario: 10101 (leyendo residuos de abajo hacia arriba)
Total de 1's: 3
```

## Enfoque de Solución

### Estrategia

Debido a que algunos números son extremadamente grandes (hasta 10¹⁰⁰⁰), debemos:

1. **Leer como string** siempre
2. **Simular la división entre 2** manualmente trabajando con el string
3. **Contar los residuos** que sean 1

### Algoritmo de División Manual

Para dividir un número grande (string) entre 2:
```
1. Inicializar residuo = 0
2. Para cada dígito del string:
   - Calcular: actual = residuo * 10 + dígito
   - Nuevo dígito del resultado = actual / 2
   - Nuevo residuo = actual % 2
3. El residuo final indica si ese bit es 1 o 0
```

### Implementación

- Función para dividir string entre 2 y obtener el residuo
- Mientras el string no sea "0":
  - Dividir entre 2
  - Si el residuo es 1, incrementar contador
  - Actualizar el string con el resultado de la división

## Complejidad

- **Tiempo**: O(T × n × log(I)) donde n es el número de dígitos del número I
- **Espacio**: O(n) para almacenar el string del número
