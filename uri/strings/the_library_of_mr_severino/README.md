# 2137 - The Library of Mr. Severino

## Problema

En un tranquilo pueblo rural, el Sr. Severino ha decidido crear su propia biblioteca, ya que colecciona varios libros desde su juventud. Él no sabe nada de programación, así que le pidió a su nieto que hiciera un programa que registrara y ordenara sus libros por su código. Sin embargo, su nieto todavía está en la escuela primaria, y como sabe muy poco de programación, terminó haciendo un programa que solo registra los libros, pero no los ordena.

Por lo tanto, Severino ha recurrido a ti, porque conoce tus habilidades de programación. Tu tarea es simple: ordenar los libros por su código.

**Input**
- Múltiples casos de prueba
- Primera línea: entero N (1 ≤ N ≤ 1000)
- Siguientes N líneas: códigos de libros en formato "xxxx" (siempre 4 dígitos con ceros a la izquierda)
- Leer hasta EOF

**Output**
- Imprimir códigos de libros ordenados
- Sin línea en blanco entre casos de prueba

**Ejemplo:**
```
Input:
3
1233
0015
0100

Output:
0015
0100
1233
```

## Análisis

1. **Tipo de problema**: Ordenamiento simple de strings

2. **Observaciones clave**:
   - Los códigos son strings de exactamente 4 caracteres
   - Todos tienen el mismo formato (con ceros a la izquierda)
   - El ordenamiento lexicográfico de strings funciona perfectamente porque todos tienen la misma longitud
   - Múltiples casos de prueba hasta EOF

3. **Algoritmo**:
   - Mientras haya entrada (hasta EOF):
     - Leer N
     - Leer N códigos
     - Ordenar los códigos lexicográficamente
     - Imprimir cada código ordenado

## Solución

**Estrategia:**
- Leer todos los códigos de un caso de prueba
- Usar la función de ordenamiento estándar de strings
- Imprimir en orden

**Complejidad:**
- Temporal: O(N log N) por caso de prueba (ordenamiento)
- Espacial: O(N) para almacenar los códigos
