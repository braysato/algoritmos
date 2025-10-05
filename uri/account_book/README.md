# Account Book - Análisis del Problema

## Descripción del Problema

La FCC (Foundation for Combating Corruption) desmanteló un esquema de corrupción en Nlogonia. Durante la operación, se incautaron varios libros contables con transacciones ilícitas documentadas.

### Contexto
- Cada página contiene transacciones (ingresos o gastos) en nilogos (N$)
- Los delincuentes no registraron el tipo de cada transacción (+ o -)
- Solo se registró el flujo de caja final con su signo
- Los fiscales necesitan determinar con certeza qué transacciones son ingresos o gastos

### Ejemplo
- Transacciones: 7, 2, 3, 1, 11
- Flujo de caja: -4
- Ecuación: ±7 ±2 ±3 ±1 ±11 = -4

**Análisis del ejemplo:**
- N$ 7: Definitivamente ingreso (+7)
- N$ 11: Definitivamente gasto (-11)
- N$ 2, 3, 1: Indeterminados (pueden ser + o -)

## Especificaciones Técnicas

### Entrada
- Múltiples casos de prueba
- Primera línea: N (número de transacciones, 2 ≤ N ≤ 40) y F (flujo de caja, -16000 ≤ F ≤ 16000)
- Siguientes N líneas: Ti (valor de la i-ésima transacción, 1 ≤ Ti ≤ 1000)
- Terminación: "0 0"

### Salida
Para cada caso:
- **'+'**: Definitivamente es ingreso
- **'-'**: Definitivamente es gasto
- **'?'**: Indeterminado (puede ser ambos)
- **'*'**: Imposible alcanzar el flujo registrado

## Análisis de Complejidad

### Problema Base
- **Tipo**: Subset Sum con signos
- **Objetivo**: Determinar qué transacciones tienen signo único
- **Desafío**: N hasta 40 hace que 2^40 sea inviable

### Enfoque de Solución

#### Para N ≤ 20: Fuerza Bruta Optimizada
- Generar todas las 2^N combinaciones de signos
- Complejidad: O(2^N × N)
- Identificar configuraciones válidas que sumen F
- Determinar qué transacciones tienen signo fijo

#### Para N > 20: Meet-in-the-Middle
- Dividir en dos mitades de tamaño N/2
- Primera mitad: 2^(N/2) combinaciones
- Segunda mitad: 2^(N/2) combinaciones
- Buscar pares que sumen F
- Complejidad: O(2^(N/2) × N)

### Optimizaciones Implementadas

1. **Verificación rápida inicial**:
   ```cpp
   if (F > total_sum || F < -total_sum || (total_sum + F) % 2 != 0)
   ```

2. **Uso de máscaras de bits** para representar configuraciones

3. **Evitar recálculos** almacenando todas las configuraciones válidas

## Casos de Prueba Analizados

### Caso 1: [1,2,3,4,5] con flujo 7
- **Salida**: `?+??+`
- **Análisis**: 
  - Posición 2 (valor 2): Solo puede ser ingreso
  - Posición 5 (valor 5): Solo puede ser ingreso
  - Resto: Indeterminados

### Caso 2: [3,5,7,11] con flujo 15
- **Salida**: `*`
- **Análisis**: Imposible alcanzar flujo 15 con estas transacciones

### Caso 3: [6,7,7,1,7] con flujo 12
- **Salida**: `+??-?`
- **Análisis**:
  - Posición 1 (valor 6): Solo puede ser ingreso
  - Posición 4 (valor 1): Solo puede ser gasto

## Implementación Final

### Características Clave
- **Lenguaje**: C++20
- **Estructura**: Función main directa sin clases
- **Memoria**: Eficiente usando vectores y máscaras
- **Rendimiento**: Optimizado para límites de tiempo estrictos

### Complejidad Temporal
- **N ≤ 20**: O(2^N × N) ≈ 20M operaciones para N=20
- **N > 20**: O(2^(N/2) × N) ≈ 40M operaciones para N=40

### Complejidad Espacial
- O(2^(N/2)) para almacenar configuraciones
- Eficiente para los límites del problema

## Conclusión

El problema requiere un enfoque híbrido que maneje eficientemente tanto casos pequeños como grandes. La implementación con meet-in-the-middle permite resolver casos con N=40 dentro de los límites de tiempo, mientras que la fuerza bruta optimizada es más directa para casos pequeños.

La clave está en reconocer que una transacción es "determinable" si y solo si aparece con el mismo signo en todas las configuraciones válidas que suman al flujo objetivo.