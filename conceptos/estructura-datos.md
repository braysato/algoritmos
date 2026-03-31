# Estructuras de Datos

## 1. Arrays (Vectores) y Listas Enlazadas

El autor las explica juntas para resaltar sus diferencias en el uso de la memoria:

- **Arrays:** Los elementos se guardan de forma contigua en memoria. Esto permite un acceso muy rapido (directo por indice), pero hace que insertar o eliminar elementos sea costoso porque hay que desplazar el resto de los datos [00:49].
- **Listas Enlazadas:** Los elementos pueden estar dispersos en la memoria. Cada item (nodo) tiene "punteros" al elemento anterior y al siguiente. Son ideales para inserciones y eliminaciones rapidas, pero el acceso a un elemento especifico es mas lento porque hay que recorrer la lista desde el inicio [02:12].

## 2. Pilas (Stacks) y Colas (Queues)

Estas estructuras gestionan el orden en que se procesan los datos:

- **Pilas (LIFO - Last In, First Out):** Como una pila de platos; el ultimo en entrar es el primero en salir. Se usan en algoritmos de recorrido de grafos o para deshacer acciones [04:12].
- **Colas (FIFO - First In, First Out):** Como una fila en el cine; el primero en entrar es el primero en ser atendido. Son esenciales para gestionar turnos de procesamiento [05:09].

## 3. Grafos y Arboles

Subiendo un nivel de complejidad, estas estructuras manejan relaciones y jerarquias:

- **Grafos:** Conjunto de nodos conectados por "aristas". Se usan para modelar redes sociales, mapas de GPS o dependencias de codigo. Pueden representarse mediante matrices de adyacencia (mas rapidas para consulta) o listas de adyacencia (ahorran espacio si hay pocas conexiones) [06:32].
- **Arboles:** Un tipo especial de grafo sin ciclos y con una estructura jerarquica (un nodo raiz y padres con hijos). El ejemplo mas claro es el DOM de una pagina HTML [09:05].

## 4. Heaps y Tries (Estructuras Avanzadas)

- **Heaps (Monticulos):** Implementados generalmente como arboles binarios, se utilizan para crear colas de prioridad. Permiten acceder de forma muy rapida al elemento minimo o maximo de un conjunto que cambia constantemente [10:45].
- **Tries (Arboles de Prefijos):** Estructura extremadamente potente para implementar autocompletado y busquedas de texto. Organizan las palabras por sus prefijos comunes, lo que hace que buscar una palabra entre millones sea muy eficiente [12:18].

## Como elegir la adecuada

El video concluye que la eleccion depende de la operacion que mas realices [16:00]:

- **Acceso rapido:** Arrays.
- **Insercion/Eliminacion frecuente:** Listas enlazadas.
- **Jerarquias:** Arboles.
- **Busqueda de texto:** Tries.
- **Relaciones complejas:** Grafos.
