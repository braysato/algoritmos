# Emoticons :-)

## Descripción del Problema

Los emoticones se utilizan en conversaciones de chat y correo electrónico para intentar expresar las emociones que las palabras impresas no pueden. Esto puede parecer una buena característica para muchos, pero muchas personas lo encuentran realmente molesto y quieren deshacerse de los emoticones.

George es una de esas personas. Odia tanto los emoticones que está preparando un plan para eliminar todos los emoticones de todos los correos electrónicos del mundo. Como compartes sus planes visionarios, estás preparando un programa especial para ayudarlo.

Tu programa recibirá la lista de emoticones a proscribir. Cada emoticón será una cadena de caracteres que no incluye ningún espacio en blanco. También recibirás varias líneas de texto. Lo que necesitas hacer es cambiar algunos caracteres del texto a espacios para asegurar que no quede ningún emoticón en el texto. Para que se considere que un emoticón aparece en el texto, debe aparecer en una sola línea y estar formado por caracteres consecutivos.

Para ayudar a que el plan de George permanezca en secreto el mayor tiempo posible, necesitas hacer tu trabajo con la menor cantidad posible de cambios de caracteres.

## Entrada

El archivo de entrada contiene varios casos de prueba. Cada caso de prueba consiste en varias líneas. La primera línea de cada caso de prueba contendrá dos enteros separados por un solo espacio: N, el número de emoticones a proscribir, y M, el número de líneas que tiene el texto. Las siguientes N líneas contienen un emoticón cada una, una cadena no vacía de hasta 15 caracteres. Cada una de las últimas M líneas del caso de prueba contiene una línea de texto de hasta 80 caracteres. Puedes asumir 1 ≤ N, M ≤ 100.

Los caracteres de entrada válidos para emoticones son letras mayúsculas y minúsculas, dígitos y los símbolos "!?.,:;-_'#$%&/=*+(){}[]". Cada línea del texto puede contener los mismos caracteres con la adición del carácter de espacio.

La entrada termina con N = M = 0.

## Salida

Para cada caso de prueba, imprime exactamente una línea que contenga un solo entero que indique el número mínimo de cambios que necesitas hacer en todo el texto para asegurar que ningún emoticón de la lista aparezca en él.

## Ejemplo de Entrada
```
4 6
:-)
:-(
(-:
)-:
Hello uncle John! :-) :-D
I am sad or happy? (-:-(?
I feel so happy, my head spins
(-:-)(-:-)(-:-)(-:-) :-) (-: :-)
but then sadness comes :-(
Loves you, Joanna :-)))))
3 1
:)
):
))
:):)):)):)):(:((:(((:):)
0 0
```

## Ejemplo de Salida
```
11
8
```

## Análisis y Enfoque de Solución

Este es un problema de **eliminación de subcadenas con mínimos cambios** que puede resolverse con programación dinámica o búsqueda greedy.

### Observaciones Clave

1. **Un emoticón aparece solo si es una secuencia consecutiva** de caracteres en una línea (no atraviesa líneas ni espacios)
2. **Cambiar un carácter a espacio** puede romper múltiples emoticones que lo contengan
3. **Objetivo**: Minimizar el número de cambios (caracteres convertidos a espacios)

### Estrategia de Solución

Para cada línea de texto:

1. **Identificar todas las ocurrencias** de todos los emoticones en la línea
2. **Encontrar el conjunto mínimo de posiciones** a cambiar para eliminar todas las ocurrencias
3. Este es un problema de **cobertura mínima de intervalos**

#### Enfoque Greedy con Ordenamiento de Intervalos

Este problema es equivalente al **Hitting Set Problem** para intervalos, que tiene una solución greedy óptima:

Para cada línea:
1. Encontrar todas las posiciones donde aparece cada emoticón (como intervalos [inicio, fin])
2. Ordenar los intervalos por su punto final (end)
3. Aplicar algoritmo greedy:
   - Inicializar lastHit = -1
   - Para cada intervalo en orden:
     - Si lastHit < inicio del intervalo (no está cubierto):
       - Colocar un hit en el fin del intervalo
       - lastHit = fin del intervalo
       - Incrementar contador

Esta estrategia garantiza el mínimo número de cambios porque:
- Al ordenar por punto final, procesamos intervalos que terminan primero
- Al colocar el hit al final del intervalo, maximizamos la posibilidad de cubrir intervalos futuros
- Es demostrable que este algoritmo greedy produce la solución óptima para el problema de hitting set en intervalos

#### Implementación Detallada

```
Para cada línea de texto:
  1. Crear lista de ocurrencias: (inicio, fin) para cada emoticón encontrado
  2. Si la lista está vacía, continuar con la siguiente línea
  3. Ordenar ocurrencias por punto final (second)
  4. lastHit = -1
  5. Para cada ocurrencia:
     - Si lastHit < inicio:
       - lastHit = fin
       - Incrementar contador de cambios
  6. Sumar cambios al total
```

### Ejemplo

Línea: `:-) :-D`
Emoticones: `:-)`, `:-(`

- Ocurrencia de `:--)` en posición [0,2]
- Cambiar posición 1 (el `-`) elimina el emoticón
- Total: 1 cambio

### Complejidad

- **Tiempo**: O(M × L × N × E) donde M es líneas, L es longitud de línea, N es número de emoticones, E es longitud de emoticón
- **Espacio**: O(M × L) para almacenar ocurrencias

### Nota Importante

Este problema requiere encontrar el **conjunto mínimo de hits** que cubra todos los intervalos, que es un problema clásico de cobertura de conjuntos (NP-hard en general, pero con enfoque greedy razonable).
