# Patrones de Arquitectura

## 1. El Monolito

El monolito tiene mala fama porque a menudo se convierte en una "Gran Bola de Barro" (Big Ball of Mud): código acoplado y difícil de mantener. Sin embargo, el problema suele ser la mala implementación, no el patrón en sí.

- **Definición:** Todo el código se despliega como una unidad única.
- **Ventaja:** Simplicidad en el despliegue y desarrollo inicial.
- **Riesgo:** Si no hay disciplina, las responsabilidades se mezclan y el sistema se vuelve rígido.

## 2. Arquitectura por Capas

Es la forma más sencilla de organizar un monolito o un microservicio. Divide el software en responsabilidades horizontales:

- **Capa de Transporte:** Maneja la comunicación externa (HTTP, Express, TRPC, CLI).
- **Capa de Dominio:** Contiene la lógica de negocio y cálculos, aislada del medio de comunicación.
- **Capa de Datos (Repositorios):** Gestiona el acceso a bases de datos o APIs externas.

## 3. Clean Architecture y Arquitectura Hexagonal

Van un paso más allá en la abstracción y el manejo de dependencias.

- **Objetivo:** Aislar el núcleo del negocio (Domain) de los detalles técnicos (bases de datos, frameworks).
- **Crítica:** Requieren mucha definición previa y pueden ralentizar el desarrollo en fases de prototipado rápido (Agile). No siempre es la mejor opción para proyectos pequeños.

## 4. El Monolito Modular

Un punto medio ideal entre el monolito simple y los microservicios.

- **Concepto:** El código está dividido en módulos bien separados (ej. pagos, compras, usuarios) que no se rompen entre sí, pero se despliegan juntos.
- **Ventaja:** Permite escalar el equipo y el código sin la complejidad de red que traen los microservicios.

## 5. Microservicios

Consiste en separar los módulos en unidades de despliegue independientes.

- **Cuándo usarlos:** Solo cuando el monolito modular ya no escala o cuando el tamaño del equipo es lo suficientemente grande como para justificar la carga operativa.
- **Regla de Oro:** Según Martin Fowler y Sam Newman, nunca empieces por microservicios. Empieza con un monolito modular y extrae servicios solo cuando sea necesario.

## 6. CQRS (Command Query Responsibility Segregation)

Un patrón avanzado para sistemas con alta carga de datos o basados en eventos.

- **Mecánica:** Separa las acciones que cambian datos (Commands) de las que solo los consultan (Queries).
- **Uso:** Ideal para sistemas asíncronos (como WebSockets) o aplicaciones con millones de eventos de usuario (clics, scroll).

## 7. Serverless y Lambdas

A menudo se confunde con microservicios, pero es un modelo de despliegue.

- **Cold Start:** El retraso que ocurre cuando una función lambda se ejecuta después de estar inactiva.
- **Costo:** Puede ser más caro que un servidor dedicado (EC2) si el tráfico es constante. Es ideal para tráfico con picos de demanda.
