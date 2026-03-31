# Patrones de Diseño

## 1. Factory Method (Creacional)

Permite crear objetos sin especificar la clase exacta del objeto que se va a crear.

- **Problema:** Evitar el uso excesivo de `new` y la dependencia directa de clases concretas.
- **Ejemplo:** Una fábrica de vehículos que devuelve un Coche o una Moto a través de una interfaz común. El código que lo usa no sabe qué vehículo es, solo sabe cómo usarlo.

## 2. Strategy (Comportamiento)

Permite definir una familia de algoritmos, encapsular cada uno y hacerlos intercambiables en tiempo de ejecución.

- **Ejemplo:** En un videojuego, un enemigo puede cambiar de arma (espada, lanza). Cada arma es una estrategia de ataque diferente que se inyecta al enemigo dinámicamente.
- **Nota moderna:** En lenguajes como JavaScript/TypeScript, a menudo se sustituye pasando funciones directamente.

## 3. Observer (Comportamiento)

Define una relación de uno a muchos, de modo que cuando un objeto cambia su estado, todos sus dependientes son notificados.

- **Ejemplo:** Un canal de YouTube (sujeto) que notifica a todos sus suscriptores (observadores) cada vez que sube un nuevo video.

## 4. Decorator (Estructural)

Permite añadir funcionalidades a un objeto de forma dinámica sin modificar su estructura original (funciona como un wrapper).

- **Ejemplo:** Añadir un sistema de logs o comprobación de permisos a una función que hace consultas a una base de datos.
- **Azúcar sintáctico:** Los decoradores con `@` en Python o TypeScript son implementaciones de este patrón.

## 5. Adapter (Estructural)

Permite que interfaces incompatibles trabajen juntas. Actúa como un traductor entre un sistema antiguo (Legacy) y uno nuevo.

- **Ejemplo:** Un conversor que recibe datos en XML y los transforma a JSON para que el sistema moderno pueda procesarlos.

## 6. Builder (Creacional)

Separa la construcción de un objeto complejo de su representación, permitiendo crear diferentes tipos y representaciones usando el mismo proceso.

- **Problema:** Evita constructores gigantes con 20 parámetros (ej. `new Clase(true, null, "dato", 0, false...)`).
- **Metáfora:** El Builder es como un contratista al que le vas pidiendo por partes cómo quieres tu casa (suelo, paredes, techo) y al final te entrega el objeto terminado con `.build()`.

## 7. Singleton (Creacional)

Garantiza que una clase tenga una única instancia y proporciona un punto de acceso global a ella.

- **Uso común:** Conexiones a bases de datos.
- **Crítica:** Es muy polémico porque puede dificultar las pruebas unitarias (testing) y ocultar dependencias, convirtiéndose en un "foco de riesgos" si no se usa con cuidado.

## 8. Chain of Responsibility (Cadena de Responsabilidad, Comportamiento)

Consiste en delegar un proceso a una cadena de manejadores (handlers). Cada manejador decide si procesa la solicitud o la pasa al siguiente en la cadena.

- **Ejemplo cotidiano:** Los Middlewares en frameworks como Express o Flask. Recibes una request, haces un proceso (ej. autenticación) y llamas a `next()` para pasarla al siguiente middleware.
- **Uso:** Sistemas de permisos secuenciales o procesamiento de datos por etapas.

## 9. Composite (Compuesto, Estructural)

Permite tratar objetos individuales y grupos de objetos de manera uniforme. Es ideal para estructuras jerárquicas o en forma de árbol.

- **Ejemplo cotidiano:** Sistemas de archivos (una carpeta puede contener archivos o más carpetas) o videojuegos (un enemigo "padre" que tiene "minions" vinculados; si el padre muere, los hijos también).
- **Uso:** Cuando necesitas que el código cliente ignore la diferencia entre composiciones de objetos y objetos individuales.

## 10. Prototype (Prototipo, Creacional)

Permite copiar objetos existentes sin que el código dependa de sus clases específicas. Es muy útil cuando la creación de un objeto es costosa o tiene atributos privados.

- **Ejemplo cotidiano:** El método `clone()` en muchos lenguajes o el sistema de prototipos de JavaScript. En lugar de instanciar y copiar atributo por atributo, el objeto se clona a sí mismo.
- **Uso:** Duplicar objetos complejos de forma eficiente y escalable.

## 11. State (Estado, Comportamiento)

Permite que un objeto cambie su comportamiento cuando su estado interno cambia. Evita las marañas de condicionales (`if/else` o `switch`) masivos.

- **Ejemplo cotidiano:** Un Post en una red social. Dependiendo de si está en estado Borrador, Privado o Público, las acciones disponibles (editar, ver, compartir) cambian completamente.
- **Uso:** Implementar máquinas de estado robustas y tipadas donde cada estado es una clase independiente.

## 12. Flyweight (Peso Ligero, Estructural)

Este es el más difícil de entender, pero clave para el rendimiento. Se basa en compartir estados comunes entre muchos objetos para ahorrar memoria RAM.

- **Ejemplo cotidiano:** Sistemas de partículas en videojuegos (humo, chispas). Si tienes 1,000,000 de chispas, todas comparten la misma textura. En lugar de copiar la imagen un millón de veces, cada partícula guarda su posición única pero apunta (referencia) a una sola textura en caché.
- **Uso:** Cuando necesitas manejar una cantidad masiva de objetos que comparten gran parte de su información.