# Evento

## Descripción del Problema

Prog y Cackto recientemente empezaron a jugar un juego de rol llamado Fortaleza. En este juego, para el jugador de evolucionar su nivel que necesitan para derrotar a los monstruos, lo que da un valor de la experiencia (XP) para el jugador.

El productor del juego, Extreme Games, ha anunciado que la próxima semana llevará a cabo el primer evento XP de este juego en el que aumentará la experiencia de monstruos en un valor de X. Como Prog y Cackto son a muy alto nivel en el que los monstruos tienen una muy alta cantidad de puntos de experiencia, que están teniendo dificultades en el cálculo de la cantidad de puntos de experiencia que los monstruos tendrán durante el evento. ¿Usted puede ayudar a ellos?

## Entrada

Habrá varios casos de prueba. Cada caso de prueba contiene dos valores: X (0 < X ≤ 3) que indican el incremento de valor de EXP de monstruos y M (10 ≤ M ≤ 2³²-1) que indican el valor EXP del monstruo. La entrada termina con valores X == 0 y M == 0, en la que no debe ser procesada.

## Salida

Para cada caso, el programa debe mostrar un valor E, el valor de la nueva Monster EXP.

## Lógica del Incremento

El incremento X funciona como un **multiplicador simple**:
- **Fórmula**: E = X * M
- El valor X se multiplica directamente por la experiencia base M
- No es exponenciación, es multiplicación lineal

## Ejemplos

| Ejemplo de entrada | Ejemplo de salida | Cálculo |
|-------------------|-------------------|---------|
| 1 544768710 | 544768710 | 1 × 544768710 |
| 2 538533133 | 1077066266 | 2 × 538533133 |
| 3 38884958 | 116654874 | 3 × 38884958 |
| 0 0 | (no procesado) | Condición de parada |

## Análisis

- **Entrada múltiple**: Se procesan varios casos hasta encontrar X=0 y M=0
- **Condición de parada**: X=0 y M=0 (no se procesa este caso)
- **Tipos de datos**: Ambos valores pueden ser muy grandes, requiere `unsigned long long`
- **Operación**: Multiplicación simple X * M
- **Rango**: X (0 < X ≤ 3), M (10 ≤ M ≤ 2³²-1)

## Corrección de Error Inicial

### Error en mi primera implementación:
Interpreté incorrectamente que X era un **exponente**, implementando M^X (potenciación).

### ¿Por qué estaba mal?
1. **Malinterpretación**: Asumí que "incremento" significaba exponenciación
2. **Complejidad innecesaria**: Implementé condicionales para diferentes potencias
3. **Tipos incorrectos**: Usé `int` para X cuando debía ser `unsigned long long`

### Implementación correcta:
- **Operación simple**: E = X * M (multiplicación directa)
- **Tipos consistentes**: `unsigned long long` para ambos valores
- **Lógica directa**: No necesita condicionales, solo multiplicación

El problema es mucho más simple de lo que inicialmente pensé: solo multiplica el factor X por la experiencia base M.

## Implementación Técnica

Se usa `scanf` con verificación de EOF para mayor robustez en programación competitiva, siguiendo las mejores prácticas para jueces online.