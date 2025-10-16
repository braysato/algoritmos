# 1420 - Counting Seconds

## Problema

A través de investigaciones y desarrollo de técnicas revolucionarias de computación, estadística e intuición, un grupo de mujeres de la Universidad de Torrinha desarrolló un software capaz de predecir el futuro. El programa puede identificar cuándo ocurrirán los próximos 5 eventos más importantes en la vida de una persona.

El programa tiene dos problemas:
1. La persona debe pasar cientos de horas respondiendo preguntas
2. Las predicciones se hacen en número de segundos desde el fin del procesamiento

Necesitamos resolver el segundo problema: convertir los segundos a fechas y horas completas.

### Entrada

La entrada consiste de varias predicciones relacionadas con diferentes personas. La primera línea de cada predicción contiene el momento exacto en que los 5 números fueron impresos por el programa. Este momento sigue el formato:

```
<día de semana>,<día><mes><año>:<hora>:<minuto>:<segundo>
```

Donde:
- **día de semana**: Abreviatura de 3 caracteres: DOM, SEG, TER, QUA, QUI, SEX, SAB
- **día**: Día del mes con dos dígitos (con cero a la izquierda si es necesario)
- **mes**: Abreviatura de 3 caracteres: JAN, FEV, MAR, ABR, MAI, JUN, JUL, AGO, SET, OUT, NOV, DEZ
- **año**: Año con cuatro dígitos
- **hora**, **minuto**, **segundo**: Con dos dígitos (con cero a la izquierda si es necesario)

Las siguientes 5 líneas contienen un número entero cada una, representando segundos (0 < d < 2.000.000.000) desde el momento del procesamiento.

Después del último caso de prueba, una línea comenzando con 'FIM' indica el fin del archivo de entrada.

**Restricciones:**
- Ninguna predicción es anterior al 01 de marzo de 2002
- El programa solo se usará hasta 23:59:59 del 31/12/2099

### Salida

El programa debe identificar cada predicción con un número de serie en la primera línea: `Previsao #N`

Las siguientes 5 líneas deben escribir los 5 momentos completos en el mismo formato usado en la entrada.

Debe haber una línea en blanco al final de cada predicción.

### Ejemplo de Entrada
```
DOM,18AGO2002:12:00:00
1
2
60
3600
86400
QUI,28FEV2002:23:59:59
1
2
3
4
5
FIM
```

### Ejemplo de Salida
```
Previsao #1
DOM,18AGO2002:12:00:01
DOM,18AGO2002:12:00:02
DOM,18AGO2002:12:01:00
DOM,18AGO2002:13:00:00
SEG,19AGO2002:12:00:00

Previsao #2
SEX,01MAR2002:00:00:00
SEX,01MAR2002:00:00:01
SEX,01MAR2002:00:00:02
SEX,01MAR2002:00:00:03
SEX,01MAR2002:00:00:04

```

## Análisis del Problema

### Comprensión
- Debemos leer una fecha/hora inicial en un formato específico
- Leer 5 números que representan segundos a sumar a esa fecha inicial
- Calcular las nuevas fechas/horas sumando esos segundos
- Imprimir las 5 fechas resultantes en el mismo formato, incluyendo el día de la semana correcto

### Observaciones Clave
1. **Manejo de fechas**: Debemos manejar correctamente el paso de segundos, minutos, horas, días, meses y años
2. **Años bisiestos**: Febrero tiene 29 días en años bisiestos
3. **Día de la semana**: Debemos calcular correctamente el día de la semana después de sumar segundos
4. **Formato específico**: La salida debe mantener el formato exacto con ceros a la izquierda
5. **Múltiples casos de prueba**: Procesar hasta encontrar "FIM"

### Desafíos
- Implementar aritmética de fechas correctamente
- Calcular el día de la semana después de sumar segundos
- Manejar años bisiestos
- Parsear y formatear fechas en el formato específico requerido

## Enfoque de Solución

### Estructura de Datos
- Arrays para almacenar los nombres de meses y días de la semana
- Variables para almacenar componentes de fecha/hora (día, mes, año, hora, minuto, segundo)

### Algoritmo Principal

**1. Parseo de la fecha inicial:**
- Extraer cada componente de la cadena de entrada usando `substr()`
- Convertir las abreviaturas de mes a números (1-12)

**2. Suma de segundos:**
- Añadir los segundos al campo de segundos
- Propagar el overflow a minutos (segundos / 60)
- Propagar el overflow a horas (minutos / 60)
- Propagar el overflow a días (horas / 24)
- Ajustar el mes y año cuando los días excedan el límite del mes

**3. Cálculo del día de la semana (Algoritmo de Zeller):**

El algoritmo de Zeller es una fórmula para calcular el día de la semana para cualquier fecha del calendario gregoriano:

```
h = (d + ⌊(13(m+1))/5⌋ + y + ⌊y/4⌋ - ⌊y/100⌋ + ⌊y/400⌋) mod 7
```

Donde:
- `d` = día del mes
- `m` = mes (con ajuste: enero y febrero se cuentan como meses 13 y 14 del año anterior)
- `y` = año (ajustado si el mes es enero o febrero)
- `h` = día de la semana (0 = domingo, 1 = lunes, ..., 6 = sábado)

**Implementación:**
```cpp
int getDayOfWeek(int d, int m, int y) {
    int t[] = {0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4};
    y -= m < 3;
    return (y + y/4 - y/100 + y/400 + t[m-1] + d) % 7;
}
```

**4. Formateo de salida:**
- Usar `setfill('0')` y `setw(2)` para asegurar ceros a la izquierda
- Concatenar todos los componentes en el formato requerido

### Funciones Auxiliares

- `getMonthNumber()` - Convierte abreviatura de mes a número
- `isLeapYear()` - Verifica si un año es bisiesto
- `getDaysInMonth()` - Retorna días en un mes (considera años bisiestos)
- `getDayOfWeek()` - Calcula el día de la semana usando Zeller
- `addSeconds()` - Suma segundos y ajusta la fecha/hora completa

## Complejidad

- **Tiempo**: O(N × 5) donde N es el número de predicciones. Para cada predicción procesamos 5 valores.
- **Espacio**: O(1) - espacio constante (solo variables simples)
