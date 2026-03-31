# Los Principios SOLID: Guía Completa

SOLID es una regla mnemotécnica impulsada por Robert C. Martin (Uncle Bob) para ayudar a los desarrolladores a evitar el código rígido y difícil de cambiar.

## 1. S: Single Responsibility Principle (Responsabilidad Única)

Una clase debe tener una sola razón para cambiar, es decir, debe encargarse de una única parte del sistema.

- **Ejemplo:** Si una clase Usuario crea el objeto y además encripta la contraseña, tiene dos responsabilidades. Si cambias el algoritmo de encriptación, debes modificar la clase Usuario.
- **Solución:** Mover la lógica de encriptación a una clase propia (PasswordHasher) para que Usuario solo se encargue de los datos del perfil.

## 2. O: Open/Closed Principle (Abierto/Cerrado)

Las entidades de software deben estar abiertas para su extensión, pero cerradas para su modificación. Para añadir funciones, debemos escribir código nuevo, no cambiar el que ya funciona.

- **Ejemplo:** Si una clase calcula el área de rectángulos, y luego quieres añadir triángulos, no deberías modificar el método original con un if/else.
- **Solución:** Usar una clase base o interfaz Forma con un método calcularArea(). Cada nueva figura (Círculo, Triángulo) hereda e implementa su propia lógica sin tocar la clase principal.

## 3. L: Liskov Substitution Principle (Sustitución de Liskov)

Las clases hijas deben poder sustituir a sus clases padre sin que el sistema falle o se comporte de forma inesperada.

- **El meme del pato:** Si tienes una clase Pato con el método volar(), y creas un PatoDeGoma, este no puede volar. Si al llamar a volar() lanzas un error, estás rompiendo el principio.
- **Solución:** Rediseñar la jerarquía. El PatoDeGoma no debería heredar de un pato que vuela, sino implementar interfaces específicas como Nadable o Ruidoso.

## 4. I: Interface Segregation Principle (Segregación de Interfaz)

Es mejor tener muchas interfaces específicas que una sola interfaz general "gorda". Ningún cliente debería ser obligado a depender de métodos que no utiliza.

- **El caso de Xerox:** Una clase Job para una impresora hacía de todo (imprimir, grapar, fax). Las partes que solo querían imprimir tenían acceso (y dependían) de la lógica del fax.
- **Solución:** Crear interfaces pequeñas como PrintJob y StapleJob. Así, cada componente solo conoce los métodos que realmente necesita.

## 5. D: Dependency Inversion Principle (Inversión de Dependencias)

Los módulos de alto nivel no deben depender de módulos de bajo nivel; ambos deben depender de abstracciones (interfaces).

- **Beneficio:** Reduce el acoplamiento. Si tu lógica de negocio depende de una interfaz de BaseDeDatos, puedes cambiar de MongoDB a PostgreSQL sin tocar una sola línea de tu lógica principal. La implementación concreta es transparente para el sistema.