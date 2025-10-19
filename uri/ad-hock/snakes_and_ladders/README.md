# 1569 - Snakes and Ladders

## Problema

Serpientes y Escaleras es un juego de mesa que se juega en una cuadrícula de 10x10. Las casillas de la cuadrícula están numeradas del 1 al 100. Cada jugador tiene una ficha que se coloca en la casilla número 1 al inicio del juego.

### Reglas del Juego

Los jugadores se turnan para tirar un dado que proporciona un número aleatorio entre 1 y 6. Después de tirar, el jugador avanza su ficha el número de casillas mostrado en el dado.

**Reglas especiales:**

1. Si esto colocaría la ficha más allá de la casilla 100, la ficha avanza hasta la casilla 100
2. Si la ficha está en una casilla que contiene el **fondo de una escalera**, la ficha debe moverse a la casilla que contiene la **cima de la escalera**
3. Si la ficha está en una casilla que contiene la **boca de una serpiente**, la ficha debe moverse a la casilla que contiene la **cola de la serpiente**
4. Ninguna casilla contiene más de un extremo de cualquier serpiente o escalera
5. La casilla 100 no contiene la boca de una serpiente ni el fondo de una escalera
6. Un jugador gana cuando su ficha alcanza la casilla 100. En ese momento, **el juego termina**
7. Las tiradas de dado después del fin del juego deben ser **ignoradas**

### Entrada

La primera línea es el número de casos de prueba a seguir. Los casos de prueba siguen uno tras otro; el formato de cada caso de prueba es el siguiente:

- La primera línea contiene tres enteros positivos:
  - **a**: número de jugadores
  - **b**: número de serpientes o escaleras
  - **c**: número de tiradas de dado
  - Habrá no más de 1,000,000 jugadores y no más de 1,000,000 tiradas de dado

- Cada una de las siguientes **b** líneas contiene dos enteros especificando una serpiente o escalera:
  - Primer entero: casilla que contiene la boca de la serpiente o el fondo de la escalera
  - Segundo entero: casilla que contiene la cola de la serpiente o la cima de la escalera

- Las siguientes **c** líneas cada una contiene un entero dando el número tirado en el dado

### Salida

Para cada jugador, imprimir una sola línea conteniendo una cadena de la forma `Position of player N is P.`, donde N es reemplazado con el número del jugador y P es reemplazado con la posición final del jugador.

### Ejemplo de Entrada
```
1
3 1 3
4 20
3
4
5
```

### Ejemplo de Salida
```
Position of player 1 is 20.
Position of player 2 is 5.
Position of player 3 is 6.
```

## Análisis del Problema

### Comprensión
- Simular el juego de Serpientes y Escaleras
- Todos los jugadores comienzan en la casilla 1
- Los jugadores se turnan tirando el dado
- Después de cada movimiento, verificar si hay serpiente o escalera
- El juego termina cuando alguien llega a la casilla 100

### Observaciones Clave

1. **Turnos rotativos**: Los jugadores tiran el dado en orden (1, 2, 3, ..., a, 1, 2, ...)
2. **Movimiento**: Posición actual + valor del dado (máximo 100)
3. **Serpientes/Escaleras**: Se aplican automáticamente después de mover
4. **Fin del juego**: Cuando un jugador llega a 100, las tiradas restantes se ignoran
5. **Estructura de datos**: Usar un mapa/diccionario para almacenar serpientes y escaleras

### Pasos para Resolver

**1. Leer la configuración del tablero:**
   - Almacenar las serpientes y escaleras en un mapa (origen → destino)

**2. Inicializar jugadores:**
   - Todos los jugadores comienzan en la posición 1

**3. Procesar las tiradas del dado:**
   - Para cada tirada:
     - Si el juego ya terminó, ignorar
     - Determinar qué jugador tira (turno actual % número de jugadores)
     - Mover al jugador: posición += dado (máximo 100)
     - Verificar si hay serpiente o escalera en la nueva posición
     - Verificar si el jugador llegó a 100 (fin del juego)
     - Incrementar el turno

**4. Imprimir posiciones finales:**
   - Para cada jugador, imprimir su posición final

### Ejemplo Explicado

**Entrada:** 3 jugadores, 1 escalera (4→20), 3 tiradas (3, 4, 5)

- **Turno 1** - Jugador 1: posición 1 + 3 = 4 → **escalera** → posición 20
- **Turno 2** - Jugador 2: posición 1 + 4 = 5
- **Turno 3** - Jugador 3: posición 1 + 5 = 6

**Salida:**
- Jugador 1: posición 20
- Jugador 2: posición 5
- Jugador 3: posición 6

## Enfoque de Solución

### Estructura de Datos
- **Map/Diccionario**: Para almacenar serpientes y escaleras (clave: casilla origen, valor: casilla destino)
- **Array/Vector**: Para almacenar la posición actual de cada jugador

### Algoritmo
1. Leer y almacenar serpientes/escaleras
2. Inicializar posiciones de jugadores en 1
3. Para cada tirada del dado:
   - Si el juego no ha terminado:
     - Calcular jugador actual
     - Mover jugador
     - Aplicar serpiente/escalera si existe
     - Verificar si llegó a 100
4. Imprimir posiciones finales

## Complejidad

- **Tiempo**: O(c) donde c es el número de tiradas de dado
- **Espacio**: O(a + b) donde a es el número de jugadores y b es el número de serpientes/escaleras
