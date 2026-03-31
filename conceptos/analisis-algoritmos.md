# Análisis de Algoritmos

## 1. Fundamentos y Estrategias

### ¿Qué es un Algoritmo?

Un algoritmo es una secuencia lógica, finita y definida de pasos para resolver un problema o realizar una tarea. En computación, se caracteriza por transformar una entrada (input) en una salida (output). Un buen algoritmo debe ser preciso e independiente del lenguaje de programación.

### Análisis de Algoritmos

Es el proceso de encontrar la complejidad computacional de un algoritmo. Su objetivo es predecir los recursos necesarios (principalmente tiempo de ejecución y memoria) a medida que el tamaño de los datos de entrada (n) aumenta. Esto permite comparar soluciones y elegir la más eficiente.

### Divide y Vencerás

Es un paradigma de diseño algorítmico que resuelve un problema en tres pasos:

- **Dividir:** Descomponer el problema original en subproblemas más pequeños del mismo tipo.
- **Vencer:** Resolver los subproblemas de forma recursiva.
- **Combinar:** Unir las soluciones de los subproblemas para obtener la solución final.

### Recursividad

Técnica donde una función se llama a sí misma para resolver versiones más pequeñas del mismo problema. Todo algoritmo recursivo requiere:

- **Caso Base:** La condición que detiene las llamadas para evitar un bucle infinito.
- **Caso Recursivo:** La lógica que reduce el problema y lo acerca hacia el caso base.

## 2. Notación Asintótica

Es el lenguaje matemático utilizado para describir el comportamiento del tiempo de ejecución de un algoritmo cuando la entrada tiende al infinito.

### Notación Asintótica (General)

Sirve para ignorar constantes y términos de menor orden, enfocándose en la tasa de crecimiento. Por ejemplo, un algoritmo con tiempo 3n^2 + 5n + 10 se simplifica asintóticamente a un comportamiento cuadrático.

### Cota Inferior Asintótica (Omega - Ω)

Representa el mejor escenario o el límite inferior de un algoritmo. Decimos que f(n) = Ω(g(n)) si el tiempo de ejecución es al menos una constante por g(n) para entradas suficientemente grandes.

### Cota Superior Asintótica (O Grande - O)

Es la más utilizada en la industria. Representa el peor escenario o el límite superior. Garantiza que el algoritmo nunca tardará más que ese tiempo. Se define formalmente como f(n) = O(g(n)).

### Cota Ajustada Asintótica (Theta - Θ)

Describe un comportamiento exacto. Un algoritmo tiene cota Θ si sus límites superior e inferior son iguales. Significa que el tiempo de ejecución crece exactamente a la misma tasa que g(n).

## 3. Algoritmos de Ordenación y Estructuras

### Algoritmos de Ordenación

Son procedimientos para organizar elementos de una lista en un orden específico (numérico o lexicográfico). La eficiencia de estos algoritmos es el ejemplo clásico para estudiar el análisis de complejidad.

### Insertion Sort (Ordenamiento por Inserción)

Algoritmo intuitivo (similar a como ordenas cartas en la mano). Construye la lista final un elemento a la vez, insertando el valor actual en su posición correcta respecto a los ya ordenados.

- **Complejidad:** O(n^2) en el peor caso.
- **Uso:** Eficiente para listas pequeñas o casi ordenadas.

### Merge Sort (Ordenamiento por Mezcla)

Algoritmo basado en Divide y Vencerás. Divide la lista a la mitad repetidamente hasta tener elementos individuales y luego los mezcla de forma ordenada.

- **Complejidad:** Θ(n log n).
- **Ventaja:** Muy eficiente para grandes volúmenes de datos y estable.

### Heaps (Montículos)

Es una estructura de datos basada en un árbol binario casi completo.

- **Max-Heap:** El valor del nodo padre es siempre mayor o igual que el de sus hijos.
- **Min-Heap:** El valor del padre es siempre menor o igual que el de sus hijos.

### Heap Sort

Algoritmo de ordenación que utiliza la estructura de Heap. Inserta todos los elementos en un montículo y extrae repetidamente el elemento máximo (o mínimo) para construir la lista ordenada.

- **Complejidad:** O(n log n).
- **Ventaja:** A diferencia de Merge Sort, no requiere memoria extra significativa (es in-place).