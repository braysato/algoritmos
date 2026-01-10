# Zero or One - 1467

## Problema
**Categoría:** Ad-Hock  
**ID:** 1467  
**Fuente:** URI Online Judge

## Descripción
Alice, Bob y Clara juegan "Zero or One", un juego donde cada participante elige 0 o 1. El ganador es quien elige un valor diferente a los otros dos. Si todos eligen el mismo valor, o si no hay un único valor diferente, no hay ganador.

**Entrada:**
- Múltiples casos de prueba hasta EOF
- Cada caso: tres enteros A, B, C (0 o 1) representando las elecciones de Alice, Bob y Clara

**Salida:**
- 'A' si Alice gana
- 'B' si Bob gana  
- 'C' si Clara gana
- '*' si no hay ganador

**Ejemplos:**
- 1 1 0 → C (Clara eligió diferente a los otros dos)
- 0 0 0 → * (todos eligieron lo mismo)
- 1 0 0 → A (Alice eligió diferente a los otros dos)

## Análisis
Para que alguien gane, debe cumplirse que:
- Su valor sea diferente al de los otros dos
- Los otros dos deben tener el mismo valor

Casos posibles:
1. A ≠ B y A ≠ C y B = C → Alice gana
2. B ≠ A y B ≠ C y A = C → Bob gana
3. C ≠ A y C ≠ B y A = B → Clara gana
4. Cualquier otro caso → No hay ganador

## Enfoque de Solución
1. Leer los tres valores A, B, C
2. Verificar si alguno de los tres tiene un valor único (diferente a los otros dos)
3. Imprimir el ganador o '*' si no hay

## Complejidad
- **Tiempo:** O(1) por caso de prueba
- **Espacio:** O(1)
