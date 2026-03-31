# Arquitectura de APIs

## 1. REST (Representational State Transfer)

Es el estilo más común. Se basa en el protocolo HTTP y es "stateless" (sin estado), lo que significa que cada petición es independiente.

- **Métodos:** Utiliza verbos HTTP como GET (leer), POST (crear), PUT (actualizar todo), PATCH (actualizar parte) y DELETE (eliminar).
- **Formato:** Típicamente usa JSON.
- **Limitación:** El cliente siempre debe iniciar la comunicación; el servidor no puede "avisar" al cliente por sí solo [07:04].

## 2. WebSockets

Permite una comunicación bidireccional y en tiempo real. A diferencia de REST, la conexión se mantiene abierta.

- **Casos de uso:** Chats, notificaciones instantáneas, paneles de control (dashboards) en vivo y juegos multijugador.
- **Funcionamiento:** Una vez establecida la conexión, tanto el servidor como el cliente pueden enviarse mensajes en cualquier momento sin esperar una petición previa.

## 3. GraphQL

Es un lenguaje de consulta para APIs que soluciona los problemas de Overfetching (recibir datos de más) y Underfetching (necesitar varias peticiones para completar la información).

- **Ventaja:** El cliente define exactamente qué campos necesita. Es ideal para aplicaciones móviles donde el ancho de banda es crítico.
- **Funcionamiento:** Utiliza un solo endpoint (normalmente /graphql) y peticiones POST con un cuerpo que describe la consulta.

## 4. Webhooks

Es un estilo de comunicación de "servidor a servidor". Se usa cuando un sistema externo necesita notificar a tu backend que algo ha sucedido.

- **Ejemplo:** Cuando un usuario paga en PayPal o Stripe, estas plataformas envían una petición POST automática a tu servidor para confirmar el pago.
- **Concepto:** Es básicamente una API a la inversa; tú expones un endpoint para que otros te llamen.

## 5. SOAP (Simple Object Access Protocol)

Un estándar veterano, muy común en sectores altamente regulados como la banca, salud o gobierno.

- **Formato:** Exclusivamente XML, lo que lo hace más pesado y "verboso" que JSON.
- **Ventaja:** Seguridad muy robusta (firmas digitales y encriptación a nivel de mensaje) y estándares estrictos.

## 6. gRPC (Google Remote Procedure Call)

Creado por Google, está diseñado para la comunicación de alta velocidad, principalmente entre microservicios.

- **Tecnología:** Utiliza HTTP/2 y Protocol Buffers (Protobuf) en lugar de JSON.
- **Ventaja:** Al enviar datos en formato binario, es extremadamente rápido y eficiente en comparación con REST o GraphQL.