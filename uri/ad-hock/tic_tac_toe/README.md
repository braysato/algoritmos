# Tic-Tac-Toe?

## Descripción del Problema

Se juega una versión tridimensional de tic-tac-toe con un tablero de `n × n` columnas (pines), cada una con `n` niveles verticales. Los jugadores (blanco y azul) alternan turnos dejando caer bolas en un pin; la bola baja por gravedad hasta el primer nivel libre del pin. Gana quien primero complete una línea recta de `n` bolas del mismo color en cualquier dirección (horizontal, vertical, diagonales en planos o en el espacio). Si nadie lo logra tras todos los movimientos, la partida termina en empate.

## Entrada

Se proporcionan varias instancias. La primera línea de cada instancia contiene un entero `n` (0 ≤ n ≤ 30). Si `n = 0`, no hay más instancias. En las siguientes `n³` líneas se indican, alternadamente empezando por el jugador blanco, los movimientos realizados. Cada movimiento es un par `i j` (1 ≤ i, j ≤ n) que indica el pin donde se deja caer la bola.

## Salida

Para cada instancia, imprimir la cabecera `Instancia k` (con `k` iniciando en 1) seguida en la línea siguiente por:
- `Branco ganhou` si el jugador blanco gana,
- `Azul ganhou` si gana el jugador azul,
- `Empate` si nadie forma una línea de `n` bolas.

Se debe imprimir una línea en blanco tras cada instancia.

## Análisis y Enfoque de Solución

Se simula la partida almacenando un cubo `n × n × n`. Para cada pin se guarda la próxima altura libre, de modo que insertar una bola cuesta `O(1)`. Después de cada jugada se verifica si el jugador actual formó una línea de longitud `n`. Para ello, desde la posición recién ocupada se exploran las 13 direcciones únicas del cubo (considerando pares de direcciones opuestas) y se cuenta cuántas fichas consecutivas del mismo color existen en ambas. Si alguna suma alcanza `n`, el jugador gana y se ignoran las comprobaciones posteriores (aunque se siguen leyendo los movimientos restantes para consumir la entrada).

### Complejidad

Cada verificación examina a lo sumo `O(n)` celdas en cada una de las 13 direcciones; como hay `n³` jugadas, el peor caso es `O(13·n⁴)`, pero con `n ≤ 30` es perfectamente manejable. En la práctica, el juego termina tan pronto aparece un ganador.
