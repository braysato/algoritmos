# Escaping the Cell Phone - 3143

## Problema

Ritcheli necesita un algoritmo que cuente las líneas totales que ocuparán los mensajes recibidos en su celular.

### Reglas:
- Cada línea del celular muestra máximo N caracteres
- Si el texto excede N caracteres, continúa en la siguiente línea
- Si el primer carácter de una nueva línea es un espacio, se ignora y comienza con el siguiente carácter no-espacio
- Cada mensaje nuevo comienza en una línea nueva

## Entrada

- Primera línea: N (10 ≤ N ≤ 1000), número de caracteres por línea
- Siguientes líneas: mensajes (hasta EOF), cada uno de máximo 10,000 caracteres

## Salida

Un único entero: el total de líneas que ocuparán todos los mensajes.

## Ejemplo

**Entrada:**
```
10
Oi Ritch
Eh o Roger
Vc ta vivo ainda? Faz dias q vc n me respondeu mais :(:(:(:(:( 
```

**Salida:**
```
8
```

### Desglose del ejemplo:
| Mensaje | Caracteres | Líneas | Detalle |
|---------|------------|--------|---------|
| "Oi Ritch" | 8 | 1 | Cabe en una línea |
| "Eh o Roger" | 10 | 1 | Exactamente N caracteres |
| "Vc ta vivo ainda?..." | 62 | 6 | Se divide en múltiples líneas |

**Total: 1 + 1 + 6 = 8 líneas**

## Análisis y Solución

### Consideraciones clave:
1. **Trailing spaces**: Los mensajes pueden tener espacios al final que deben eliminarse antes de procesar
2. **Líneas vacías**: Se ignoran (no se cuentan)
3. **Salto de espacios**: Al truncar una línea, si el siguiente carácter es espacio, se salta hasta encontrar un carácter no-espacio
4. **Verificación post-salto**: Después de saltar espacios, verificar si aún hay contenido antes de contar una línea adicional

### Algoritmo:
```
1. Leer N (caracteres por línea)
2. Para cada mensaje:
   a. Eliminar espacios al final del mensaje (trim)
   b. Si está vacío, ignorar
   c. Iniciar líneas = 1, char_count = 0, i = 0
   d. Mientras i < longitud del mensaje:
      - Incrementar char_count e i
      - Si char_count == N y hay más caracteres:
        * Saltar todos los espacios consecutivos
        * Si después de saltar aún hay contenido:
          - Incrementar líneas
          - Reiniciar char_count = 0
   e. Sumar líneas al total
3. Imprimir total
```

### Complejidad:
- **Tiempo**: O(M * L) donde M es el número de mensajes y L la longitud promedio
- **Espacio**: O(L) para almacenar cada mensaje
